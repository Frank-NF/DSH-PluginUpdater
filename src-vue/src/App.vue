<template>
  <div class="w-app">
    <HeaderBar
      :plugin-directory="pluginStore.config?.plugin_directory || ''"
      :is-scanning="pluginStore.isScanning"
      :is-checking-updates="pluginStore.isCheckingUpdates"
      :auto-scanning="isAutoScanning"
      :plugin-count="pluginStore.plugins.length"
      :updatable-count="pluginStore.updatablePlugins.length"
      :market-count="pluginStore.marketPlugins.length"
      :last-scan-time="pluginStore.lastScanTime"
      :theme="theme"
      @scan="handleScan"
      @check-updates="handleCheckUpdates"
      @auto-scan="handleAutoScan"
      @open-settings="showSettings = true"
      @open-repair="showRepair = true"
      @open-website="openWebsite"
      @toggle-locale="toggleLocale"
      @toggle-theme="toggleTheme"
    />

    <div ref="bodyEl" class="w-body">
      <div class="w-page">
        <!-- 错误提示条 -->
        <div v-if="pluginStore.errorMessage" class="w-alert">
          <WIcon name="alert" :size="16" />
          <span class="w-flex-1">{{ pluginStore.errorMessage }}</span>
          <WButton size="mini" icon="refresh" @click="handleAutoScan">
            {{ t('common.retry') }}
          </WButton>
        </div>

        <!-- 首次启动：加载中 -->
        <WEmpty
          v-if="booting"
          type="loading"
          :title="t('app.loadingTitle')"
          :desc="t('app.loadingDesc')"
        />

        <!-- 空状态：既没扫到插件，也没有市场数据 -->
        <WEmpty
          v-else-if="!hasContent"
          type="empty"
          :title="t('empty.title')"
          :desc="t('empty.hint1')"
          icon="inbox"
        >
          <template #action>
            <WButton
              type="primary"
              icon="wand"
              :loading="isAutoScanning"
              @click="handleAutoScan"
            >
              {{ t('header.autoScan') }}
            </WButton>
            <WButton icon="settings" @click="showSettings = true">
              {{ t('empty.openSettings') }}
            </WButton>
          </template>
          <template #tips>
            <span class="w-text-2">{{ t('empty.tips') }}</span>
          </template>
        </WEmpty>

        <!-- 主内容：插件列表 -->
        <PluginTable
          v-else
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
    </div>

    <!-- 全局宿主：轻提示 / 确认框 / 动作面板 -->
    <WToast />
    <WConfirmHost />
    <WActionHost />

    <SettingsDialog
      v-model="showSettings"
      :config="pluginStore.config"
      @save="handleSaveConfig"
    />

    <ReleaseNotesDialog
      v-model="showReleaseNotes"
      :plugin="currentReleaseNotesPlugin"
      @update="handleUpdatePlugin"
    />

    <RepairDialog v-model="showRepair" />
  </div>
</template>

<script setup lang="ts">
import { computed, nextTick, onMounted, ref } from 'vue'
import { usePluginStore } from './stores/pluginStore'
import { t, toggleLocale, locale } from './i18n'
import { pluginApi } from './api'
import { useToast } from './composables/useToast'
import { useConfirm } from './composables/useConfirm'
import { useActionSheet } from './composables/useActionSheet'
import { useTheme } from './composables/useTheme'
import { fadeSlideIn } from './composables/useMotion'
import type { PluginInfo, AppConfig } from './types'

import HeaderBar from './components/HeaderBar.vue'
import PluginTable from './components/PluginTable.vue'
import SettingsDialog from './components/SettingsDialog.vue'
import ReleaseNotesDialog from './components/ReleaseNotesDialog.vue'
import RepairDialog from './components/RepairDialog.vue'
import WToast from './components/WToast.vue'
import WConfirmHost from './components/WConfirmHost.vue'
import WActionHost from './components/WActionHost.vue'
import WEmpty from './components/WEmpty.vue'
import WButton from './components/WButton.vue'
import WIcon from './components/WIcon.vue'

const pluginStore = usePluginStore()
const toast = useToast()
const { confirm } = useConfirm()
const { actionSheet } = useActionSheet()
const { theme, toggleTheme } = useTheme()

const booting = ref(true)
const showSettings = ref(false)
const showRepair = ref(false)
const isAutoScanning = ref(false)
const showReleaseNotes = ref(false)
const currentReleaseNotesPlugin = ref<PluginInfo | null>(null)
const bodyEl = ref<HTMLElement | null>(null)

/** 有内容可展示 = 扫到了插件，或市场数据已到位 */
const hasContent = computed(
  () => pluginStore.plugins.length > 0 || pluginStore.marketPlugins.length > 0
)

onMounted(async () => {
  await boot()
  booting.value = false
  // 内容区淡入（GSAP）
  nextTick(() => fadeSlideIn(bodyEl.value?.querySelector('.w-page') ?? null, { y: 12 }))
})

async function boot() {
  await pluginStore.loadConfig()
  pluginStore.setupEventListeners()

  // 打开即加载插件市场
  await pluginStore.fetchMarket()

  // 自动定位并扫描已安装插件
  try {
    if (!pluginStore.config?.plugin_directory) {
      await pluginStore.autoScanPlugins()
    } else {
      await pluginStore.scanPlugins(pluginStore.config.plugin_directory)
    }
  } catch {
    /* 扫描失败不阻塞，市场仍可浏览 */
  }

  // 打开即自动检查更新
  if (pluginStore.plugins.length > 0) {
    try {
      await pluginStore.checkAllUpdates()
    } catch {
      /* 忽略 */
    }
  }
}

async function handleAutoScan() {
  isAutoScanning.value = true
  try {
    await pluginStore.autoScanPlugins()
    toast.success(t('scan.autoDone'))
  } catch (e) {
    toast.error(e?.toString() || t('scan.autoFailed'))
  } finally {
    isAutoScanning.value = false
  }
}

async function handleScan(directory: string) {
  try {
    await pluginStore.scanPlugins(directory)
    toast.success(t('scan.done', { n: pluginStore.plugins.length }))

    if (pluginStore.config?.auto_check_updates && pluginStore.plugins.length > 0) {
      await pluginStore.checkAllUpdates()
    }
  } catch (e: unknown) {
    toast.error(toMessage(e, t('scan.failed')))
  }
}

async function handleCheckUpdates() {
  if (pluginStore.plugins.length === 0) {
    toast.warn(t('scan.needFirst'))
    return
  }
  try {
    await pluginStore.checkAllUpdates()
    const count = pluginStore.updatablePlugins.length
    if (count > 0) toast.text(t('check.found', { n: count }))
    else toast.success(t('check.allLatest'))
  } catch (e: unknown) {
    toast.error(toMessage(e, t('check.failed')))
  }
}

/**
 * 更新插件
 * - DSH 运行中 → 动作面板三选一（强杀后更新 / 仍要继续 / 放弃）
 * - 否则 → 确认框
 */
async function handleUpdatePlugin(plugin: PluginInfo) {
  let dshRunning = false
  try {
    dshRunning = await pluginApi.isDshRunning()
  } catch {
    dshRunning = false
  }

  const detail = t('update.confirmMsg', {
    name: plugin.manifest.name,
    current: plugin.manifest.current_version,
    latest: plugin.latest_version || '?',
  })

  let force = false

  if (dshRunning) {
    const message =
      detail +
      t('update.dshRunningHint')
    const choice = await actionSheet({
      title: t('update.dshRunningTitle'),
      message,
      items: [
        { label: t('update.killAndContinue'), value: 'kill' },
        { label: t('update.continueAnyway'), value: 'force', type: 'warn' },
      ],
    })
    if (!choice) return

    if (choice === 'force') {
      // 仍要继续（不推荐）→ 后端 force 放行
      force = true
    } else {
      const ok = await killDsh()
      if (!ok) return
    }
  } else {
    const ok = await confirm({
      title: t('update.confirmTitle'),
      message: detail + t('update.normalHint'),
      confirmText: t('update.confirmBtn'),
      cancelText: t('common.cancel'),
      type: 'warn',
    })
    if (!ok) return
  }

  try {
    const newVersion = await pluginStore.updatePlugin(plugin.manifest.id, force)
    toast.success(
      t('update.done', {
        name: plugin.manifest.name,
        version: newVersion ?? plugin.latest_version ?? '',
      })
    )
  } catch (e: unknown) {
    toast.error(toMessage(e, t('update.failed')))
  }
}

/** 强杀 DSH 进程树；必要时走 UAC 提权。返回是否成功 */
async function killDsh(): Promise<boolean> {
  try {
    const killed = await pluginApi.killDshProcesses()
    if (killed > 0) {
      toast.success(t('update.killDone', { n: killed }))
      await sleep(800)
      return true
    }

    // 普通权限强杀失败（目标可能提权运行）→ 请求 UAC 提权
    const ok = await confirm({
      title: t('update.dshRunningTitle'),
      message: t('update.elevatePrompt'),
      confirmText: t('update.killAndContinue'),
      cancelText: t('common.cancel'),
      type: 'warn',
    })
    if (!ok) return false

    await pluginApi.killDshProcessesElevated()
    toast.text(t('update.elevating'))

    // 轮询确认 DSH 已退出（给用户时间点 UAC 弹窗）
    for (let i = 0; i < 27; i++) {
      await sleep(1500)
      try {
        if (!(await pluginApi.isDshRunning())) return true
      } catch {
        /* 轮询失败继续等 */
      }
    }
    toast.error(t('update.elevateFailed'))
    return false
  } catch (e: unknown) {
    toast.error(toMessage(e, t('update.killFailed')))
    return false
  }
}

async function handleUninstallPlugin(plugin: PluginInfo) {
  if (plugin.manifest.type === 'agent-core') {
    toast.warn(t('uninstall.coreBlocked'))
    return
  }

  const ok = await confirm({
    title: t('uninstall.confirmTitle'),
    message: t('uninstall.confirmMsg', { name: plugin.manifest.name }),
    confirmText: t('uninstall.confirmBtn'),
    cancelText: t('common.cancel'),
    type: 'danger',
  })
  if (!ok) return

  try {
    await pluginStore.uninstallPlugin(plugin.manifest.id)
    toast.success(t('uninstall.done', { name: plugin.manifest.name }))
  } catch (e: unknown) {
    toast.error(toMessage(e, t('error.uninstall')))
  }
}

async function handleToggleEnabled(plugin: PluginInfo) {
  const next = !plugin.manifest.enabled
  try {
    await pluginStore.setPluginEnabled(plugin.manifest.id, next)
    const stateText = next
      ? locale.value === 'zh'
        ? '启用'
        : 'enabled'
      : locale.value === 'zh'
        ? '禁用'
        : 'disabled'
    toast.success(
      t('app.enabledToggled', { name: plugin.manifest.name, state: stateText })
    )
  } catch (e: unknown) {
    toast.error(toMessage(e, t('app.operationFailed')))
  }
}

async function handleOpenFolder(plugin: PluginInfo) {
  try {
    await pluginStore.openPluginFolder(plugin.manifest.id)
  } catch (e: unknown) {
    toast.error(toMessage(e, t('error.openFolder')))
  }
}

async function handleCheckSingle(plugin: PluginInfo) {
  try {
    await pluginStore.checkSingleUpdate(plugin.manifest.id)
    toast.success(t('app.checkDone', { name: plugin.manifest.name }))
  } catch (e: unknown) {
    toast.error(toMessage(e, t('table.checkFailed')))
  }
}

function handleViewReleaseNotes(plugin: PluginInfo) {
  currentReleaseNotesPlugin.value = plugin
  showReleaseNotes.value = true
}

async function handleSaveConfig(config: AppConfig) {
  try {
    await pluginStore.saveConfig(config)
    toast.success(t('settings.saved'))
    showSettings.value = false
  } catch (e: unknown) {
    toast.error(toMessage(e, t('error.saveConfig')))
  }
}

function openWebsite() {
  // 官网走系统浏览器统一入口（webview 内 window.open 不可靠）
  pluginApi.openExternal(WEBSITE_URL).catch(() => {})
}

/* ---------- 工具 ---------- */
const WEBSITE_URL = 'https://dsh.huilinsh.cn'

function sleep(ms: number) {
  return new Promise((r) => setTimeout(r, ms))
}

function toMessage(e: unknown, fallback: string): string {
  if (e == null) return fallback
  if (typeof e === 'string') return e
  if (e instanceof Error) return e.message || fallback
  return String(e)
}
</script>

<style scoped>
/* 错误提示条 */
.w-alert {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
  padding: 12px var(--sp-4);
  margin-bottom: var(--sp-4);
  border-radius: var(--r-md);
  background: rgba(239, 68, 68, 0.1);
  border: 1px solid rgba(239, 68, 68, 0.28);
  color: #f87171;
  font-size: 13px;
  line-height: 1.6;
  backdrop-filter: var(--glass-blur);
  -webkit-backdrop-filter: var(--glass-blur);
}
</style>
