import { invoke } from '@tauri-apps/api/core'
import type {
  PluginInfo,
  AppConfig,
  BackupInfo,
  UpdateProgress,
  EnvCheckItem,
  MarketPlugin,
  SelfUpdateInfo,
  AutoUpdateState,
  CatalogStatus,
} from '../types'
import { listen } from '@tauri-apps/api/event'

/** 本地 DSH Web 服务器状态（dsh_server.rs ServerStatus） */
export interface ServerStatus {
  running: boolean
  port_open: boolean
  port: number
  pid: number
  process_name: string
  url: string
  auth_url: string
  message: string
}

/** 本地 DSH Web 服务器管理（启动/停止/重启/状态） */
export const serverApi = {
  status: (): Promise<ServerStatus> => invoke('server_status'),
  start: (port?: number): Promise<string> => invoke('server_start', { port }),
  stop: (): Promise<string> => invoke('server_stop'),
  restart: (port?: number): Promise<string> => invoke('server_restart', { port }),
}

/**
 * 运行环境检测：
 * - Tauri 桌面端：走真实 invoke
 * - 纯浏览器（如服务器预览调试）：走 Mock 数据，避免 invoke 不存在导致白屏
 */
export const isTauriEnv =
  typeof window !== 'undefined' &&
  ('__TAURI_INTERNALS__' in window || '__TAURI__' in window)
const isTauri = isTauriEnv

/* ============================================================
 * 插件 API：仅 Tauri 桌面端（在线版已下线，浏览器 Mock 层已移除）
 * ============================================================ */

export const pluginApi = {
      scanPlugins: (directory: string): Promise<PluginInfo[]> =>
        invoke('scan_plugins', { directory }),

      autoScanPlugins: (): Promise<PluginInfo[]> => invoke('auto_scan_plugins'),

      checkUpdates: (): Promise<PluginInfo[]> => invoke('check_updates'),

      checkSingleUpdate: (pluginId: string): Promise<PluginInfo> =>
        invoke('check_single_update', { pluginId }),

      updatePlugin: (pluginId: string, force?: boolean): Promise<string> =>
        invoke('update_plugin', { pluginId, force: force ?? false }),

      uninstallPlugin: (pluginId: string): Promise<void> =>
        invoke('uninstall_plugin', { pluginId }),

      setPluginEnabled: (pluginId: string, enabled: boolean): Promise<void> =>
        invoke('set_plugin_enabled', { pluginId, enabled }),

      openPluginFolder: (pluginId: string): Promise<void> =>
        invoke('open_plugin_folder', { pluginId }),

      getConfig: (): Promise<AppConfig> => invoke('get_config'),

      testServerConnection: (): Promise<string> =>
      invoke('test_server_connection'),

    syncToServer: (kind: 'app' | 'catalog' | 'plugins'): Promise<string> =>
      invoke('sync_to_server', { kind }),

    checkEnvironment: (): Promise<EnvCheckItem[]> =>
      invoke('check_environment'),

    listCatalogPlugins: (): Promise<MarketPlugin[]> =>
      invoke('list_catalog_plugins'),

    installPlugin: (npmName: string, targetDir: string): Promise<string> =>
      invoke('install_plugin', { npmName, targetDir }),

    listInstallTargets: (): Promise<string[]> => invoke('list_install_targets'),

    listDshProcesses: (): Promise<{ pid: number; name: string }[]> =>
      invoke('list_dsh_processes'),

    killDshProcesses: (): Promise<number> => invoke('kill_dsh_processes'),

    openExternal: (url: string): Promise<void> => invoke('open_external', { url }),

    pickDirectory: (): Promise<string | null> => invoke('pick_directory'),

    killDshProcessesElevated: (): Promise<number> => invoke('kill_dsh_processes_elevated'),

    updateConfig: (config: AppConfig): Promise<void> =>
        invoke('update_config', { newConfig: config }),

      listBackups: (pluginId: string): Promise<BackupInfo[]> =>
        invoke('list_backups', { pluginId }),

      restoreBackup: (backupId: string): Promise<void> =>
        invoke('restore_backup', { backupId }),

      isDshRunning: (): Promise<boolean> => invoke('is_dsh_running'),

      checkSelfUpdate: (): Promise<SelfUpdateInfo> => invoke('check_self_update'),

      selfUpdate: (): Promise<string> => invoke('self_update'),

      getAutoUpdateState: (): Promise<AutoUpdateState> => invoke('get_auto_update_state'),

      getCatalogTrust: (): Promise<CatalogStatus> => invoke('get_catalog_trust'),

      launchAutoUpdate: (tempPath: string): Promise<void> =>
        invoke('launch_auto_update', { tempPath }),

      onUpdateProgress: (callback: (progress: UpdateProgress) => void) =>
        listen<UpdateProgress>('update_progress', (event) => {
          callback(event.payload)
        }),

      onDeepLinkAction: (callback: (action: string) => void) =>
        listen<string>('deep-link-action', (event) => {
          callback(event.payload)
        }),
    }

export const eventApi = {
  onUpdateProgress: (callback: (progress: UpdateProgress) => void) => {
    if (!isTauri) {
      return Promise.resolve(() => {})
    }
    return listen<UpdateProgress>('update_progress', (event) => {
      callback(event.payload)
    })
  },

  onAutoUpdateCheck: (callback: (data: any) => void) => {
    if (!isTauri) {
      return Promise.resolve(() => {})
    }
    return listen<any>('auto_update_check', (event) => {
      callback(event.payload)
    })
  },

  onAutoUpdateProgress: (callback: (data: any) => void) => {
    if (!isTauri) {
      return Promise.resolve(() => {})
    }
    return listen<any>('auto_update_progress', (event) => {
      callback(event.payload)
    })
  },

  onAutoUpdateDone: (callback: (data: any) => void) => {
    if (!isTauri) {
      return Promise.resolve(() => {})
    }
    return listen<any>('auto_update_done', (event) => {
      callback(event.payload)
    })
  },

  /** V3 安全：目录签名验证失败时，后端通过 catalog_status 事件推送告警 */
  onCatalogStatus: (callback: (status: CatalogStatus) => void) => {
    if (!isTauri) {
      return Promise.resolve(() => {})
    }
    return listen<CatalogStatus>('catalog_status', (event) => {
      callback(event.payload)
    })
  },
}

export function formatFileSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
  return `${(bytes / (1024 * 1024 * 1024)).toFixed(2)} GB`
}
