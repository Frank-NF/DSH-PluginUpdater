/**
 * Bundle 协议 V2：官方组合包种子数据 + 行聚合（V2 §2）
 * 种子幂等（INSERT OR IGNORE），由 API 模块懒加载调用。
 * 枚举白名单：mode=preset、scope=user、transport=stdio；env_keys 仅键名永不存值。
 */
import Database from 'better-sqlite3'

interface SeedMcpServer {
  serverId: string
  name: string
  transport: string
  command: string
  args: string[]
  envKeys: string[]
  optional: boolean
  description: string
}

interface SeedSkill {
  skillId: string
  name: string
  source: string
  scope: string
  optional: boolean
}

interface SeedBundle {
  id: string
  name: string
  description: string
  tags: string[]
  mode: string
  minDshVersion: string
  maxDshVersion: string
  recommendPreset: string
  version: string
  createTime: string
  plugins: string[]
  mcpServers: SeedMcpServer[]
  skills: SeedSkill[]
}

const OFFICIAL_BUNDLES: SeedBundle[] = [
  {
    id: 'bundle-starter',
    name: '小白入门包',
    description: '新手友好的一键入门组合：Markdown 速投、视觉路由、用量统计与历史树，开箱即用。',
    tags: ['入门', '基础'],
    mode: 'preset',
    minDshVersion: '*',
    maxDshVersion: '',
    recommendPreset: '',
    version: '1.0.0',
    createTime: '2026-08-31',
    plugins: ['dsh-drop-md', 'dsh-vision-router', 'dsh-cost-meter', 'dsh-history-tree'],
    mcpServers: [],
    skills: []
  },
  {
    id: 'bundle-dev-full',
    name: 'AI 开发者包',
    description: '面向 AI 开发者的完整工具链：MCP 服务管理、逻辑探针、LSP 动作、多模态识图与浏览器驾驶舱。',
    tags: ['开发', '工具'],
    mode: 'preset',
    minDshVersion: '*',
    maxDshVersion: '',
    recommendPreset: '',
    version: '1.0.0',
    createTime: '2026-08-31',
    plugins: [
      'dsh-mcp-manager',
      'dsh-logicprobe',
      'dsh-lsp-actions',
      '@liustack/modlens',
      'dsh-pilot',
      'dsh-cost-meter'
    ],
    mcpServers: [
      {
        serverId: 'mcp-github',
        name: 'GitHub MCP',
        transport: 'stdio',
        command: 'npx',
        args: ['-y', '@modelcontextprotocol/server-github'],
        envKeys: ['GITHUB_TOKEN'],
        optional: false,
        description: 'GitHub 官方 MCP 服务：仓库 / Issue / PR 操作'
      }
    ],
    skills: [
      {
        skillId: 'skill-logicprobe',
        name: '逻辑探针技能',
        source: 'dsh-logicprobe',
        scope: 'user',
        optional: false
      }
    ]
  },
  {
    id: 'bundle-content',
    name: '内容创作包',
    description: '内容创作场景组合：视觉路由、在线表格文档、Markdown 投放与生成式 UI。',
    tags: ['创作', '效率'],
    mode: 'preset',
    minDshVersion: '*',
    maxDshVersion: '',
    recommendPreset: '',
    version: '1.0.0',
    createTime: '2026-08-31',
    plugins: [
      'dsh-vision-router',
      'dsh-univer-office',
      'dsh-drop-md',
      '@changfenhuang/dsh-genui'
    ],
    mcpServers: [],
    skills: []
  },
  {
    id: 'bundle-research',
    name: '学术 RAG 包',
    description: '学术研究场景组合：文档表格处理、上下文管理与记忆留存，构建轻量 RAG 工作流。',
    tags: ['学术', '资料'],
    mode: 'preset',
    minDshVersion: '*',
    maxDshVersion: '',
    recommendPreset: '',
    version: '1.0.0',
    createTime: '2026-08-31',
    plugins: ['dsh-univer-office', 'dsh-context', 'dsh-memoir'],
    mcpServers: [],
    skills: []
  },
  {
    id: 'bundle-enterprise',
    name: '企业安全包',
    description: '企业安全基线组合：内置开关管控、用量成本计量与上下文治理，适合团队统一配置。',
    tags: ['企业', '安全'],
    mode: 'preset',
    minDshVersion: '*',
    maxDshVersion: '',
    recommendPreset: '',
    version: '1.0.0',
    createTime: '2026-08-31',
    plugins: ['dsh-builtin-toggles', 'dsh-cost-meter', 'dsh-context'],
    mcpServers: [],
    skills: []
  }
]

let seeded = false

/** 幂等写入 5 个官方组合包（重复调用零副作用） */
export function seedBundles(db: Database.Database): void {
  if (seeded) return
  const insertBundle = db.prepare(
    'INSERT OR IGNORE INTO bundles ' +
    '(id, name, description, tags, mode, min_dsh_version, max_dsh_version, recommend_preset, version, create_time) ' +
    'VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)'
  )
  const insertPlugin = db.prepare(
    'INSERT OR IGNORE INTO bundle_plugins (bundle_id, plugin_ref, required) VALUES (?, ?, ?)'
  )
  const insertMcp = db.prepare(
    'INSERT OR IGNORE INTO bundle_mcp_servers ' +
    '(bundle_id, server_id, name, transport, command, args, env_keys, optional, description) ' +
    'VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)'
  )
  const insertSkill = db.prepare(
    'INSERT OR IGNORE INTO bundle_skills (bundle_id, skill_id, name, source, scope, optional) VALUES (?, ?, ?, ?, ?, ?)'
  )
  const run = db.transaction(() => {
    for (const b of OFFICIAL_BUNDLES) {
      insertBundle.run(
        b.id,
        b.name,
        b.description,
        JSON.stringify(b.tags),
        b.mode,
        b.minDshVersion,
        b.maxDshVersion,
        b.recommendPreset,
        b.version,
        b.createTime
      )
      for (const p of b.plugins) insertPlugin.run(b.id, p, 1)
      for (const m of b.mcpServers) {
        insertMcp.run(
          b.id,
          m.serverId,
          m.name,
          m.transport,
          m.command,
          JSON.stringify(m.args),
          JSON.stringify(m.envKeys),
          m.optional ? 1 : 0,
          m.description
        )
      }
      for (const s of b.skills) {
        insertSkill.run(b.id, s.skillId, s.name, s.source, s.scope, s.optional ? 1 : 0)
      }
    }
  })
  run()
  seeded = true
}

// ---------- 行类型与聚合（DB snake_case → API camelCase） ----------

export interface BundleRow {
  id: string
  name: string
  description: string
  tags: string
  mode: string
  min_dsh_version: string
  max_dsh_version: string
  recommend_preset: string
  version: string
  create_time: string
}

export interface BundlePluginRow {
  plugin_ref: string
  required: number
}

export interface BundleMcpServerRow {
  server_id: string
  name: string
  transport: string
  command: string
  args: string
  env_keys: string
  optional: number
  description: string
}

export interface BundleSkillRow {
  skill_id: string
  name: string
  source: string
  scope: string
  optional: number
}

function parseJsonArray(raw: string | null | undefined): string[] {
  if (!raw) return []
  try {
    const v = JSON.parse(raw)
    return Array.isArray(v) ? v.map(String) : []
  } catch {
    return []
  }
}

/** 把 bundles 主表行 + 三张子表聚合成一个完整的 Bundle 对象（camelCase） */
export function buildBundle(db: Database.Database, row: BundleRow) {
  const plugins = db.prepare(
    'SELECT plugin_ref, required FROM bundle_plugins WHERE bundle_id = ? ORDER BY rowid ASC'
  ).all(row.id) as unknown as BundlePluginRow[]
  const mcpServers = db.prepare(
    'SELECT server_id, name, transport, command, args, env_keys, optional, description FROM bundle_mcp_servers WHERE bundle_id = ? ORDER BY rowid ASC'
  ).all(row.id) as unknown as BundleMcpServerRow[]
  const skills = db.prepare(
    'SELECT skill_id, name, source, scope, optional FROM bundle_skills WHERE bundle_id = ? ORDER BY rowid ASC'
  ).all(row.id) as unknown as BundleSkillRow[]

  return {
    id: row.id,
    name: row.name,
    description: row.description,
    tags: parseJsonArray(row.tags),
    mode: row.mode,
    minDshVersion: row.min_dsh_version,
    maxDshVersion: row.max_dsh_version,
    recommendPreset: row.recommend_preset,
    version: row.version,
    createTime: row.create_time,
    plugins: plugins.map((p) => ({ pluginRef: p.plugin_ref, required: !!p.required })),
    mcpServers: mcpServers.map((m) => ({
      serverId: m.server_id,
      name: m.name,
      transport: m.transport,
      command: m.command,
      args: parseJsonArray(m.args),
      envKeys: parseJsonArray(m.env_keys),
      optional: !!m.optional,
      description: m.description
    })),
    skills: skills.map((s) => ({
      skillId: s.skill_id,
      name: s.name,
      source: s.source,
      scope: s.scope,
      optional: !!s.optional
    }))
  }
}
