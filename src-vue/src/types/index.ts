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

export interface SelfUpdateInfo {
  available: boolean
  current_version: string
  latest_version: string | null
  changelog: string[]
  release_url: string | null
  is_mandatory: boolean
}

export type UpdateStatus = 'idle' | 'checking' | 'updating' | 'success' | 'error'

/* ================= 组合包（Bundle 协议 V2，camelCase 与后端 serde 对齐） ================= */

export interface BundlePluginRef {
  pluginRef: string
  required: boolean
}

export interface BundleMcpServer {
  serverId: string
  name: string
  transport: string
  command: string
  args: string[]
  envKeys: string[]
  optional: boolean
  description: string
}

export interface BundleSkill {
  skillId: string
  name: string
  source: string
  scope: string
  optional: boolean
}

export interface BundleDef {
  id: string
  name: string
  description: string
  tags: string[]
  mode: string
  minDshVersion: string | null
  maxDshVersion: string | null
  recommendPreset: string | null
  version: string | null
  createTime: string | null
  plugins: BundlePluginRef[]
  mcpServers: BundleMcpServer[]
  skills: BundleSkill[]
}

export interface BundleConflictInfo {
  conflictWith: string
  reason: string | null
  severity: 'block' | 'warn' | string | null
}

export interface BundlePreviewItem {
  pluginRef: string
  required: boolean
  installed: boolean
  currentVersion: string | null
  action: 'install' | 'overwrite' | 'skip'
  /** 与当前已装插件集的已知冲突（官网知识库；空 = 无/未检） */
  conflicts: BundleConflictInfo[]
}

export interface BundlePreview {
  bundle: BundleDef
  targetDir: string
  items: BundlePreviewItem[]
  mcpServers: BundleMcpServer[]
  skills: BundleSkill[]
  /** 整包兼容结论（官网不可达时为 null） */
  compatAllCompatible: boolean | null
  hasBlockingConflict: boolean | null
}

export interface McpEnvKeyInfo {
  key: string
  hasSecret: boolean
}

export interface McpEntryInfo {
  serverId: string
  name: string
  transport: 'stdio' | 'streamable-http' | string
  command?: string
  args?: string[]
  url?: string
  envKeys: McpEnvKeyInfo[]
  description: string
  hasPlainValue: boolean
}

export interface McpListResult {
  enabled: McpEntryInfo[]
  disabled: McpEntryInfo[]
  configExists: boolean
}

export interface McpProbeResult {
  ok: boolean
  detail: string
  latencyMs: number
}

export interface BundlePluginResult {
  pluginRef: string
  status: string
  detail: string
}

export interface BundleInstallResult {
  taskId: string
  bundleId: string
  status: 'committed' | 'cancelled' | 'rolled_back' | 'failed'
  message: string
  plugins: BundlePluginResult[]
  /** mode=preset 时生成的会话预设建议文件路径 */
  presetSuggestionPath?: string
}

export interface BundleProgress {
  taskId: string
  bundleId: string
  stage: string
  percent: number
  message: string
}
