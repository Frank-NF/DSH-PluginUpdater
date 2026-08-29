<template>
  <div class="app-container">
    <HeaderBar
      :plugin-directory="pluginStore.config?.plugin_directory || ''"
      :is-scanning="pluginStore.isScanning"
      :is-checking-updates="pluginStore.isCheckingUpdates"
      :auto-scanning="isAutoScanning"
      :plugin-count="pluginStore.plugins.length"
      :updatable-count="pluginStore.updatablePlugins.length"
      :last-scan-time="pluginStore.lastScanTime"
      @scan="handleScan"
      @check-updates="handleCheckUpdates"
      @auto-scan="handleAutoScan"
      @open-settings="showSettings = true"
      @open-website="openWebsite"
      @open-repair="showRepair = true"
    />

    <div class="main-content">
      <!-- 背景光晕（纯装饰，不拦截交互） -->
      <div class="bg-glow" aria-hidden="true"></div>

      <div v-if="pluginStore.errorMessage" class="error-banner">
        <el-alert
          :title="pluginStore.errorMessage"
          type="error"
          show-icon
          :closable="false"
        />
      </div>

      <PluginTable
        v-if="pluginStore.plugins.length > 0 || pluginStore.marketPlugins.length > 0"
        :plugins="pluginStore.plugins"
        :market-plugins="pluginStore.marketPlugins"
        :is-checking-updates="pluginStore.isCheckingUpdates"
        @update="handleUpdatePlugin"
        @uninstall="handleUninstallPlugin"
        @toggle-enabled="handleToggleEnabled"
        @open-folder="handleOpenFolder"
        @check-single="handleCheckSingle"
        @view-release-notes="handleViewReleaseNotes"
        />
    </div>

    <SettingsDialog
      v-model="showSettings"
      :config="pluginStore.config"
      @save="handleSaveConfig"
    />

    <ReleaseNotesDialog
      v-model="showReleaseNotes"
      :plugin="currentReleaseNotesPlugin"
    />

    <RepairDialog v-model="showRepair" />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { FolderOpened, MagicStick, Tools } from '@element-plus/icons-vue'
import { usePluginStore } from './stores/pluginStore'
import { t, locale } from './i18n'
import { pluginApi } from './api'
import HeaderBar from './components/HeaderBar.vue'
import PluginTable from './components/PluginTable.vue'
import SettingsDialog from './components/SettingsDialog.vue'
import ReleaseNotesDialog from './components/ReleaseNotesDialog.vue'
import RepairDialog from './components/RepairDialog.vue'
import type { PluginInfo, AppConfig } from './types'

const pluginStore = usePluginStore()
const showSettings = ref(false)
const isAutoScanning = ref(false)
const showReleaseNotes = ref(false)
const showRepair = ref(false)
const currentReleaseNotesPlugin = ref<PluginInfo | null>(null)

onMounted(async () => {
  await pluginStore.loadConfig()
  pluginStore.setupEventListeners()

  // 打开即加载插件市场全部插件
  await pluginStore.fetchMarket()

  // 自动定位并扫描已安装插件（若已配置目录则直接扫描）
  if (!pluginStore.config?.plugin_directory) {
    try {
      await pluginStore.autoScanPlugins()
    } catch {
      /* 扫描失败不阻塞，市场仍可浏览 */
    }
  } else {
    try {
      await pluginStore.scanPlugins(pluginStore.config.plugin_directory)
    } catch {
      /* 忽略 */
    }
  }

  // 打开即自动检查更新（已装插件）
  if (pluginStore.plugins.length > 0) {
    try {
      await pluginStore.checkAllUpdates()
    } catch {
      /* 忽略 */
    }
  }
})

async function handleAutoScan() {
    isAutoScanning.value = true
    try {
      await pluginStore.autoScanPlugins()
      ElMessage.success(t('scan.autoDone'))
    } catch (e) {
      ElMessage.error(e?.toString() || t('scan.autoFailed'))
    } finally {
      isAutoScanning.value = false
    }
  }

    async function handleScan(directory: string) {
  try {
    await pluginStore.scanPlugins(directory)
    ElMessage.success(t('scan.done', { n: pluginStore.plugins.length }))

    // 如果配置了自动检查更新，则自动检查
    if (pluginStore.config?.auto_check_updates && pluginStore.plugins.length > 0) {
      await pluginStore.checkAllUpdates()
    }
  } catch (e: any) {
    ElMessage.error(e?.toString() || t('scan.failed'))
  }
}

async function handleCheckUpdates() {
  if (pluginStore.plugins.length === 0) {
    ElMessage.warning(t('scan.needFirst'))
    return
  }
  try {
    await pluginStore.checkAllUpdates()
    const count = pluginStore.updatablePlugins.length
    if (count > 0) {
      ElMessage.info(t('check.found', { n: count }))
    } else {
      ElMessage.success(t('check.allLatest'))
    }
  } catch (e: any) {
    ElMessage.error(e?.toString() || t('check.failed'))
  }
}

async function handleUpdatePlugin(plugin: PluginInfo) {
  // 防护：检测 DSH 桌面端是否运行（运行时锁定插件目录，更新会失败）
  let dshRunning = false
  try {
    dshRunning = await pluginApi.isDshRunning()
  } catch {
    dshRunning = false
  }

  let confirmMessage = t('update.confirmMsg', {
    name: plugin.manifest.name,
    current: plugin.manifest.current_version,
    latest: plugin.latest_version || '?',
  })
  confirmMessage += dshRunning ? t('update.dshRunningHint') : t('update.normalHint')

  // DSH 运行中：三按钮（确认=强杀 DSH 并更新 / 取消=仍要继续不推荐 / 关闭=放弃）
  let action: 'confirm' | 'cancel' | 'close' = 'close'
  try {
    await ElMessageBox.confirm(confirmMessage, dshRunning ? t('update.dshRunningTitle') : t('update.confirmTitle'), {
      confirmButtonText: dshRunning ? t('update.killAndContinue') : t('update.confirmBtn'),
      cancelButtonText: dshRunning ? t('update.continueAnyway') : t('common.cancel'),
      distinguishCancelAndClose: true,
      type: 'warning',
    })
    action = 'confirm'
  } catch (act) {
    action = act === 'cancel' ? 'cancel' : 'close'
  }

  if (!dshRunning && action !== 'confirm') return
  if (action === 'close') return

  let force = false
  if (dshRunning) {
    if (action === 'cancel') {
      // 仍要继续（不推荐）→ 后端 force 放行
      force = true
    } else {
      // 强杀 DSH 进程树后更新（后端会再校验一次 DSH 已退出）
      try {
        const killed = await pluginApi.killDshProcesses()
        if (!killed) {
          // 普通权限强杀失败（目标可能提权运行）→ 提供 UAC 提权强杀，标准用户也可在系统弹窗授权
          try {
            await ElMessageBox.confirm(t('update.elevatePrompt'), t('update.dshRunningTitle'), {
              confirmButtonText: t('update.killAndContinue'),
              cancelButtonText: t('common.cancel'),
              type: 'warning',
            })
          } catch {
            return
          }
          await pluginApi.killDshProcessesElevated()
          ElMessage.info(t('update.elevating'))
          // 轮询确认 DSH 退出（给用户时间点 UAC 弹窗）
          let gone = false
          for (let i = 0; i < 27; i++) {
            await new Promise((r) => setTimeout(r, 1500))
            try {
              if (!(await pluginApi.isDshRunning())) {
                gone = true
                break
              }
            } catch {
              // 轮询失败继续等
            }
          }
          if (!gone) {
            ElMessage.error(t('update.elevateFailed'))
            return
          }
        } else {
          ElMessage.success(t('update.killDone', { n: killed }))
          await new Promise((r) => setTimeout(r, 800))
        }
      } catch (e: any) {
        ElMessage.error(e?.toString() || t('update.killFailed'))
        return
      }
    }
  }

  try {
    const newVersion = await pluginStore.updatePlugin(plugin.manifest.id, force)
    ElMessage.success(t('update.done', { name: plugin.manifest.name, version: newVersion }))
  } catch (e: any) {
    ElMessage.error(e?.toString() || t('update.failed'))
  }
}

async function handleUninstallPlugin(plugin: PluginInfo) {
  if (plugin.manifest.type === 'agent-core') {
    ElMessage.warning(t('uninstall.coreBlocked'))
    return
  }

  try {
    await ElMessageBox.confirm(
      t('uninstall.confirmMsg', { name: plugin.manifest.name }),
      t('uninstall.confirmTitle'),
      {
        confirmButtonText: t('uninstall.confirmBtn'),
        cancelButtonText: t('common.cancel'),
        type: 'error',
      }
    )

    await pluginStore.uninstallPlugin(plugin.manifest.id)
    ElMessage.success(t('uninstall.done', { name: plugin.manifest.name }))
  } catch (e: any) {
    if (e !== 'cancel') {
      ElMessage.error(e?.toString() || t('error.uninstall'))
    }
  }
}

async function handleToggleEnabled(plugin: PluginInfo) {
  try {
    await pluginStore.setPluginEnabled(plugin.manifest.id, !plugin.manifest.enabled)
    ElMessage.success(t('app.enabledToggled', { name: plugin.manifest.name, state: plugin.manifest.enabled ? (locale.value === 'zh' ? '禁用' : 'disabled') : (locale.value === 'zh' ? '启用' : 'enabled') }))
  } catch (e: any) {
    ElMessage.error(e?.toString() || t('app.operationFailed'))
  }
}

async function handleOpenFolder(plugin: PluginInfo) {
  try {
    await pluginStore.openPluginFolder(plugin.manifest.id)
  } catch (e: any) {
    ElMessage.error(e?.toString() || t('error.openFolder'))
  }
}

async function handleCheckSingle(plugin: PluginInfo) {
  try {
    await pluginStore.checkSingleUpdate(plugin.manifest.id)
    ElMessage.success(t('app.checkDone', { name: plugin.manifest.name }))
  } catch (e: any) {
    ElMessage.error(e?.toString() || t('table.checkFailed'))
  }
}

function handleViewReleaseNotes(plugin: PluginInfo) {
  currentReleaseNotesPlugin.value = plugin
  showReleaseNotes.value = true
}

async function handleSaveConfig(config: AppConfig) {
  try {
    await pluginStore.saveConfig(config)
    ElMessage.success(t('settings.saved'))
    showSettings.value = false
  } catch (e: any) {
    ElMessage.error(e?.toString() || t('error.saveConfig'))
  }
}

function openWebsite() {
  // 官网走系统浏览器统一入口（webview 内 window.open 不可靠）
  pluginApi.openExternal('https://dsh.huilinsh.cn').catch(() => {})
}
</script>

<style scoped>
.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
  background: var(--bg-primary);
}

.main-content {
  position: relative;
  flex: 1;
  overflow: auto;
  padding: 20px 24px 32px;
}

/* 背景光晕：靛蓝径向渐变，纯装饰 */
.bg-glow {
  position: fixed;
  top: -20%;
  left: 50%;
  transform: translateX(-50%);
  width: 900px;
  height: 900px;
  background: radial-gradient(
    circle,
    rgba(99, 102, 241, 0.13) 0%,
    rgba(99, 102, 241, 0.05) 35%,
    transparent 70%
  );
  pointer-events: none;
  z-index: 0;
}

.main-content > *:not(.bg-glow) {
  position: relative;
  z-index: 1;
}

.error-banner {
  margin-bottom: 16px;
}

/* ---------- v2 空状态 ---------- */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 72px 24px;
  text-align: center;
}

.empty-icon-wrap {
  width: 96px;
  height: 96px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 24px;
  background: rgba(99, 102, 241, 0.1);
  border: 1px solid rgba(99, 102, 241, 0.22);
  color: var(--primary-light);
  margin-bottom: 24px;
}

.empty-state h3 {
  margin: 0 0 8px;
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
}

.empty-state p {
  font-size: 14px;
  line-height: 1.6;
  color: var(--text-secondary);
  max-width: 440px;
  margin-bottom: 24px;
}

.empty-actions {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
  justify-content: center;
}

.empty-actions .el-button {
  border-radius: var(--radius-md);
  padding: 10px 20px;
}

/* 非主要按钮在空态下用描边样式 */
.empty-actions .el-button:not(.el-button--primary) {
  background: transparent;
  border: 1px solid var(--glass-border);
  color: var(--text-secondary);
}

.empty-actions .el-button:not(.el-button--primary):hover {
  background: rgba(255, 255, 255, 0.06);
  color: var(--text-primary);
  border-color: rgba(255, 255, 255, 0.22);
}
</style>
