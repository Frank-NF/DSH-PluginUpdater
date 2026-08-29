export interface PluginManifest {
  id: string
  name: string
  description: string
  github_repo: string
  current_version: string
  enabled: boolean
  type: string
  author: string
  homepage: string
}

/** 官方市场插件（目录精简条目） */
export interface MarketPlugin {
  name: string
  category: string | null
  stars: number | null
  downloads: number | null
  desc_zh: string | null
  desc_en: string | null
  npm: string | null
  url: string | null
}

/** DSH 运行环境检查项 */
export interface EnvCheckItem {
  id: string
  name: string
  status: 'ok' | 'warn' | 'error'
  message: string
  fix_hint: string
}

export interface PluginInfo {
  manifest: PluginManifest
  install_path: string
  latest_version: string | null
  release_url: string | null
  download_url: string | null
  release_notes: string | null
  update_available: boolean
  check_error: string | null
  description_zh: string | null
  description_en: string | null
  category: string | null
  stars: number | null
  downloads: number | null
}

export interface UpdateProgress {
  plugin_id: string
  phase: string
  percent: number
  message: string
}

export interface AppConfig {
  server_host: string
  server_port: number
  server_user: string
  server_key: string
  server_remote_dir: string
  server_dsh_dir: string
  server_update_cmd: string
  proxy_base_url: string
  plugin_directory: string
  auto_check_updates: boolean
  backup_before_update: boolean
  install_registry: string
}

export interface BackupInfo {
  name: string
  path: string
  size: number
  created: string
}

export type UpdateStatus = 'idle' | 'checking' | 'updating' | 'success' | 'error'
