import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { PluginInfo, AppConfig, UpdateProgress, MarketPlugin, SelfUpdateInfo } from '../types'
import { pluginApi, eventApi } from '../api'
import { t } from '../i18n'

export const usePluginStore = defineStore('plugin', () => {
  const plugins = ref<PluginInfo[]>([])
  const marketPlugins = ref<MarketPlugin[]>([])
  const config = ref<AppConfig | null>(null)
  const isScanning = ref(false)
  const isCheckingUpdates = ref(false)
  const updatingPlugins = ref<Set<string>>(new Set())
  const updateProgressMap = ref<Map<string, UpdateProgress>>(new Map())
  const lastScanTime = ref<string>('')
  const errorMessage = ref<string>('')
  const installingNpm = ref<string | null>(null)
const selfUpdateInfo = ref<SelfUpdateInfo | null>(null)
const isCheckingSelfUpdate = ref(false)

  const updatablePlugins = computed(() =>
    plugins.value.filter(
      (p) => p.update_available && p.manifest.type !== 'agent-core' && !p.bundled
    )
  )

  const enabledPlugins = computed(() =>
    plugins.value.filter((p) => p.manifest.enabled)
  )

  const disabledPlugins = computed(() =>
    plugins.value.filter((p) => !p.manifest.enabled)
  )

  const agentCore = computed(() =>
    plugins.value.find((p) => p.manifest.type === 'agent-core')
  )

  const regularPlugins = computed(() =>
    plugins.value.filter((p) => p.manifest.type !== 'agent-core')
  )

  async function loadConfig() {
    try {
      config.value = await pluginApi.getConfig()
    } catch (e) {
      console.error(t('error.loadConfig'), e)
    }
  }

  async function saveConfig(newConfig: AppConfig) {
    try {
      await pluginApi.updateConfig(newConfig)
      config.value = newConfig
    } catch (e) {
      console.error(t('error.saveConfig'), e)
      throw e
    }
  }

  async function autoScanPlugins() {
    isScanning.value = true
    errorMessage.value = ''
    try {
      const result = await pluginApi.autoScanPlugins()
      plugins.value = result
      lastScanTime.value = new Date().toLocaleString('zh-CN')
      // Save the found directory to config

      return result
    } catch (e: any) {
      errorMessage.value = e?.toString() || t('error.autoScan')
      throw e
    } finally {
      isScanning.value = false
    }
  }

  async function scanPlugins(directory: string) {
    isScanning.value = true
    errorMessage.value = ''
    try {
      const result = await pluginApi.scanPlugins(directory)
      plugins.value = result
      lastScanTime.value = new Date().toLocaleString('zh-CN')
      return result
    } catch (e: any) {
      errorMessage.value = e?.toString() || t('error.scan')
      throw e
    } finally {
      isScanning.value = false
    }
  }

  async function checkAllUpdates() {
    isCheckingUpdates.value = true
    errorMessage.value = ''
    try {
      const result = await pluginApi.checkUpdates()
      plugins.value = result
      return result
    } catch (e: any) {
      errorMessage.value = e?.toString() || t('error.checkUpdates')
      throw e
    } finally {
      isCheckingUpdates.value = false
    }
  }

  async function checkSingleUpdate(pluginId: string) {
    try {
      const updated = await pluginApi.checkSingleUpdate(pluginId)
      const index = plugins.value.findIndex((p) => p.manifest.id === pluginId)
      if (index !== -1) {
        plugins.value[index] = updated
      }
      return updated
    } catch (e) {
      console.error(t('error.checkSingle'), e)
      throw e
    }
  }

  /** 从市场安装插件到目标 profile 目录，完成后自动扫描刷新列表 */
  async function checkSelfUpdate() {
    isCheckingSelfUpdate.value = true
    try {
      selfUpdateInfo.value = await pluginApi.checkSelfUpdate()
      return selfUpdateInfo.value
    } finally {
      isCheckingSelfUpdate.value = false
    }
  }

  async function selfUpdate() {
    return pluginApi.selfUpdate()
  }

  async function installPlugin(npmName: string, targetDir: string) {
    installingNpm.value = npmName
    try {
      const msg = await pluginApi.installPlugin(npmName, targetDir)
      // 安装后扫描目标目录，刷新已安装列表
      try {
        await scanPlugins(targetDir)
      } catch {
        // 扫描失败不阻塞安装结果
      }
      return msg
    } finally {
      installingNpm.value = null
    }
  }
  async function updatePlugin(pluginId: string, force = false) {
    if (updatingPlugins.value.has(pluginId)) return

    // DSH 运行检测与确认由 App.vue 的 handleUpdatePlugin 统一处理（force=用户已确认仍要继续）
    updatingPlugins.value.add(pluginId)
    updateProgressMap.value.set(pluginId, {
      plugin_id: pluginId,
      phase: 'starting',
      percent: 0,
      message: t('update.confirmBtn') + '...',
    })

    try {
      const newVersion = await pluginApi.updatePlugin(pluginId, force)

      // 更新本地状态
      const index = plugins.value.findIndex((p) => p.manifest.id === pluginId)
      if (index !== -1) {
        plugins.value[index].manifest.current_version = newVersion
        plugins.value[index].update_available = false
        plugins.value[index].latest_version = newVersion
      }

      return newVersion
    } catch (e) {
      console.error(t('error.updatePlugin'), e)
      throw e
    } finally {
      updatingPlugins.value.delete(pluginId)
      updateProgressMap.value.delete(pluginId)
    }
  }

  async function uninstallPlugin(pluginId: string) {
    try {
      await pluginApi.uninstallPlugin(pluginId)
      plugins.value = plugins.value.filter((p) => p.manifest.id !== pluginId)
    } catch (e) {
      console.error(t('error.uninstall'), e)
      throw e
    }
  }

  async function setPluginEnabled(pluginId: string, enabled: boolean) {
    try {
      await pluginApi.setPluginEnabled(pluginId, enabled)
      const plugin = plugins.value.find((p) => p.manifest.id === pluginId)
      if (plugin) {
        plugin.manifest.enabled = enabled
      }
    } catch (e) {
      console.error(t('error.setEnabled'), e)
      throw e
    }
  }

  async function openPluginFolder(pluginId: string) {
    try {
      await pluginApi.openPluginFolder(pluginId)
    } catch (e) {
      console.error(t('error.openFolder'), e)
      throw e
    }
  }

  function setupEventListeners() {
    eventApi.onUpdateProgress((progress) => {
      updateProgressMap.value.set(progress.plugin_id, progress)
    })
  }

  function getUpdateProgress(pluginId: string): UpdateProgress | undefined {
    return updateProgressMap.value.get(pluginId)
  }

  function isUpdating(pluginId: string): boolean {
    return updatingPlugins.value.has(pluginId)
  }

  function clearPlugins() {
    plugins.value = []
    lastScanTime.value = ''
  }

  async function fetchMarket() {
    try {
      marketPlugins.value = await pluginApi.listCatalogPlugins()
    } catch (e) {
      console.error('拉取插件市场失败:', e)
      marketPlugins.value = []
    }
  }

return {
    marketPlugins,
    fetchMarket,
    plugins,
    config,
    isScanning,
    isCheckingUpdates,
    updatingPlugins,
    updateProgressMap,
    lastScanTime,
    errorMessage,
    updatablePlugins,
    enabledPlugins,
    disabledPlugins,
    agentCore,
    regularPlugins,
    loadConfig,
    saveConfig,
    scanPlugins,
    checkAllUpdates,
    checkSingleUpdate,
    updatePlugin,
    uninstallPlugin,
    setPluginEnabled,
    openPluginFolder,
    setupEventListeners,
    getUpdateProgress,
    isUpdating,
    clearPlugins,
    autoScanPlugins,
    installingNpm,
    installPlugin,
    checkSelfUpdate,
    selfUpdate,
    selfUpdateInfo,
    isCheckingSelfUpdate,
  }
})
