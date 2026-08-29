import { invoke } from '@tauri-apps/api/core'
import type { PluginInfo, AppConfig, BackupInfo, UpdateProgress } from '../types'
import { listen } from '@tauri-apps/api/event'

/**
 * 运行环境检测：
 * - Tauri 桌面端：走真实 invoke
 * - 纯浏览器（如服务器预览调试）：走 Mock 数据，避免 invoke 不存在导致白屏
 */
const isTauri =
  typeof window !== 'undefined' &&
  ('__TAURI_INTERNALS__' in window || '__TAURI__' in window)

/* ============================================================
 * Mock 层（仅浏览器预览用，Tauri 环境完全不加载这部分逻辑）
 * ============================================================ */

const mockConfig: AppConfig = {
  proxy_base_url: '',
  plugin_directory: 'C:\\DSH\\plugins',
  auto_check_updates: true,
  backup_before_update: true,
}

const makePlugin = (
  id: string,
  name: string,
  description: string,
  repo: string,
  current: string,
  latest: string | null,
  enabled: boolean,
  type = 'plugin'
): PluginInfo => ({
  manifest: {
    id,
    name,
    description,
    github_repo: repo,
    current_version: current,
    enabled,
    type,
    author: 'DSH-Team',
    homepage: 'https://dsh.huilinsh.cn',
  },
  install_path: `C:\\DSH\\plugins\\${id}`,
  latest_version: latest,
  release_url: latest ? `https://github.com/${repo}/releases/tag/v${latest}` : null,
  download_url: null,
  release_notes: latest
    ? `## 更新内容\n\n- 新增工作流模板导入导出\n- 优化显存占用，峰值降低约 18%\n- 修复部分节点断线后无法自动重连的问题\n- 升级内置推理内核至最新稳定版`
    : null,
  update_available: !!latest && latest !== current,
  check_error: null,
  description_zh: description,
  description_en: description,
  category: null,
  stars: null,
  downloads: null,
})

let mockPlugins: PluginInfo[] = [
  makePlugin(
    'dsh-agent-core',
    'DSH Agent 本体',
    '核心程序，提供插件运行环境与基础服务',
    'DSH-Team/DSH-Agent',
    '1.0.0',
    null,
    true,
    'agent-core'
  ),
  makePlugin(
    'dsh-plugin-comfyui',
    'ComfyUI 集成插件',
    'Integrates ComfyUI workflows for image generation pipelines',
    'DSH-Team/dsh-plugin-comfyui',
    '0.9.0',
    '1.0.0',
    true
  ),
  makePlugin(
    'dsh-plugin-memory',
    '长期记忆插件',
    '基于向量数据库的长期记忆存储与检索',
    'DSH-Team/dsh-plugin-memory',
    '0.8.0',
    null,
    false
  ),
  makePlugin(
    'dsh-plugin-ollama',
    'Ollama 本地模型插件',
    'Local LLM inference via Ollama, supports qwen3-vl vision models',
    'DSH-Team/dsh-plugin-ollama',
    '0.5.2',
    '0.6.0',
    true
  ),
  makePlugin(
    'dsh-plugin-browser',
    '浏览器自动化插件',
    '网页浏览、内容抓取与自动化操作',
    'DSH-Team/dsh-plugin-browser',
    '0.3.1',
    null,
    true
  ),
]

const delay = (ms: number) => new Promise((r) => setTimeout(r, ms))

const mockApi = {
  scanPlugins: async (directory: string): Promise<PluginInfo[]> => {
    await delay(800)
    return mockPlugins
  },
  autoScanPlugins: async (): Promise<PluginInfo[]> => {
    await delay(1200)
    mockConfig.plugin_directory = 'C:\\Users\\niufe\\.dsh\\plugins'
    return mockPlugins
  },
  checkUpdates: async (): Promise<PluginInfo[]> => {
    await delay(1500)
    return mockPlugins
  },
  checkSingleUpdate: async (pluginId: string): Promise<PluginInfo> => {
    await delay(600)
    const p = mockPlugins.find((x) => x.manifest.id === pluginId)
    if (!p) throw new Error('插件不存在')
    return p
  },
  updatePlugin: async (pluginId: string, force?: boolean): Promise<string> => {
    // 模拟分阶段进度事件
    const phases: Array<[string, number, string]> = [
      ['downloading', 15, '正在下载更新包...'],
      ['downloading', 45, '正在下载更新包...'],
      ['downloading', 78, '正在下载更新包...'],
      ['downloading', 95, '下载完成，正在校验...'],
      ['installing', 100, '正在安装...'],
    ]
    for (const [phase, percent, message] of phases) {
      await delay(500)
      progressListeners.forEach((cb) =>
        cb({ plugin_id: pluginId, phase, percent, message })
      )
    }
    await delay(400)
    const p = mockPlugins.find((x) => x.manifest.id === pluginId)
    if (p && p.latest_version) {
      p.manifest.current_version = p.latest_version
      p.update_available = false
      const v = p.latest_version
      p.release_url = null
      return v
    }
    throw new Error('插件不存在')
  },
  uninstallPlugin: async (pluginId: string) => {
    await delay(700)
    mockPlugins = mockPlugins.filter((x) => x.manifest.id !== pluginId)
  },
  setPluginEnabled: async (pluginId: string, enabled: boolean) => {
    await delay(300)
    const p = mockPlugins.find((x) => x.manifest.id === pluginId)
    if (p) p.manifest.enabled = enabled
  },
  openPluginFolder: async () => {
    // 浏览器环境无文件夹可打开
  },
  getConfig: async (): Promise<AppConfig> => {
    await delay(200)
    return { ...mockConfig }
  },
  updateConfig: async (config: AppConfig) => {
    await delay(300)
    Object.assign(mockConfig, config)
  },
  installPlugin: async (npmName: string, targetDir: string): Promise<string> => {
    await delay(1500)
    return `已安装 ${npmName} 到 ${targetDir}`
  },
  listInstallTargets: async (): Promise<string[]> => {
    await delay(200)
    return ['C:\\Users\\you\\.dsh\\profiles\\desktop']
  },
  listDshProcesses: async (): Promise<{ pid: number; name: string }[]> => {
    await delay(200)
    return []
  },
  killDshProcesses: async (): Promise<number> => {
    await delay(500)
    return 0
  },
  openExternal: async (url: string): Promise<void> => {
    window.open(url, '_blank')
  },
  killDshProcessesElevated: async (): Promise<number> => {
    await delay(300)
    return 0
  },
  listBackups: async (): Promise<BackupInfo[]> => [],
  restoreBackup: async () => {},
}

/* 进度事件监听器（Mock 用） */
const progressListeners = new Set<(p: UpdateProgress) => void>()

/* ============================================================
 * 统一出口：Tauri 环境走 invoke，浏览器走 Mock
 * ============================================================ */

export const pluginApi = isTauri
  ? {
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

    killDshProcessesElevated: (): Promise<number> => invoke('kill_dsh_processes_elevated'),

    updateConfig: (config: AppConfig): Promise<void> =>
        invoke('update_config', { newConfig: config }),

      listBackups: (pluginId: string): Promise<BackupInfo[]> =>
        invoke('list_backups', { pluginId }),

      restoreBackup: (backupId: string): Promise<void> =>
        invoke('restore_backup', { backupId }),

      isDshRunning: (): Promise<boolean> => invoke('is_dsh_running'),

      onUpdateProgress: (callback: (progress: UpdateProgress) => void) =>
        listen<UpdateProgress>('update_progress', (event) => {
          callback(event.payload)
        }),
    }
  : {
      ...mockApi,
      // 在线版（浏览器）：市场数据走官网真实目录 API（2189 款，与桌面端/官网同源）
      listCatalogPlugins: async (): Promise<MarketPlugin[]> => {
        try {
          const res = await fetch('https://dsh.huilinsh.cn/api/plugins', { timeout: 15000 } as RequestInit)
          if (!res.ok) throw new Error(String(res.status))
          const data = await res.json()
          return (data.plugins ?? []).map((p: Record<string, unknown>) => ({
            name: (p.name as string) ?? '',
            category: (p.category as string) ?? null,
            stars: (p.stars as number) ?? null,
            downloads: null,
            desc_zh: (p.description as string) ?? null,
            desc_en: (p.github_description as string) ?? null,
            npm: (p.id as string) ?? null,
            url: (p.github_url as string) ?? null,
          }))
        } catch {
          // 网络不可达时市场置空，界面照常
          return []
        }
      },

      isDshRunning: async (): Promise<boolean> => false,


      onUpdateProgress: (callback: (progress: UpdateProgress) => void) => {
        progressListeners.add(callback)
        return Promise.resolve(() => progressListeners.delete(callback))
      },
    }

export const eventApi = {
  onUpdateProgress: (callback: (progress: UpdateProgress) => void) => {
    return listen<UpdateProgress>('update_progress', (event) => {
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
