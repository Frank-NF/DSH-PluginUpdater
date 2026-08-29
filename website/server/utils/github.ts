/**
 * DSH 插件注册表 + GitHub 实时数据合并
 *
 * 策略：
 * 1. REGISTRY 是官方收录的插件清单（id / 仓库 / 分类）
 * 2. 运行时从 GitHub API 拉取每个仓库的实时数据（stars / 更新时间 / 描述 / 语言）
 * 3. 内存缓存 10 分钟，避免触发 GitHub 60 次/小时的限流
 * 4. GitHub 拉取失败时降级使用缓存或注册表静态数据
 */

export interface RegistryEntry {
  id: string
  repo: string // GitHub 仓库全名 owner/name
  name: string
  description: string
  category: string
  type: 'agent-core' | 'plugin'
}

export interface PluginData extends RegistryEntry {
  stars: number
  forks: number
  github_description: string | null
  language: string | null
  pushed_at: string | null
  github_url: string
  topics: string[]
  fetched: boolean // GitHub 数据是否获取成功
}

/** catalog 分类 key → 中文显示名（与桌面端 21 种分类一致） */
const CATEGORY_ZH: Record<string, string> = {
  ui: '界面', usage: '使用', theme: '主题', model: '模型', identity: '身份',
  session: '会话', memory: '记忆', tools: '工具', browser: '浏览器', vision: '视觉',
  voice: '语音', docs: '文档', skill: '技能', workflow: '工作流', git: 'Git',
  notify: '通知', dev: '开发', security: '安全', remote: '远程', market: '市场',
  fun: '娱乐',
}

/** 官网静态分发目录下的市场目录（与桌面端 dsh-plugin-catalog 同源） */
const CATALOG_PATH = '/var/www/dsh-updater/plugins.json'

interface CatalogEntry {
  name: string
  owner: string
  url: string
  category: string
  description: { en: string; zh: string }
  npm: string
  stars: number
  downloads: number
  added: string
}

/** 官方插件注册表：新插件在这里添加一行即可上架 */
const REGISTRY: RegistryEntry[] = [
  {
    id: 'dsh-agent-core',
    repo: 'Frank-NF/DSH-Agent',
    name: 'DSH Agent 本体',
    description: '核心程序，提供插件运行环境与基础服务',
    category: '核心',
    type: 'agent-core',
  },
  {
    id: 'dsh-plugin-comfyui',
    repo: 'Frank-NF/dsh-plugin-comfyui',
    name: 'ComfyUI 集成插件',
    description: '集成 ComfyUI 工作流，支持 AI 绘图和视频生成任务调度',
    category: 'AI 绘图',
    type: 'plugin',
  },
  {
    id: 'dsh-plugin-ollama',
    repo: 'Frank-NF/dsh-plugin-ollama',
    name: 'Ollama 本地模型插件',
    description: '通过 Ollama 进行本地大模型推理，支持视觉模型',
    category: '本地 AI',
    type: 'plugin',
  },
  {
    id: 'dsh-plugin-memory',
    repo: 'Frank-NF/dsh-plugin-memory',
    name: '长期记忆插件',
    description: '向量数据库支持的长期记忆存储和语义检索能力',
    category: '能力扩展',
    type: 'plugin',
  },
  {
    id: 'dsh-plugin-browser',
    repo: 'Frank-NF/dsh-plugin-browser',
    name: '浏览器自动化插件',
    description: '基于浏览器自动化的网页操作、数据采集和交互能力',
    category: '自动化',
    type: 'plugin',
  },
  {
    id: 'dsh-plugin-tts',
    repo: 'Frank-NF/dsh-plugin-tts',
    name: '语音合成插件',
    description: '多语言文本转语音，支持多种音色和情感表达',
    category: '多媒体',
    type: 'plugin',
  },
  {
    id: 'dsh-plugin-mcp',
    repo: 'Frank-NF/dsh-plugin-mcp',
    name: 'MCP 协议插件',
    description: 'Model Context Protocol 支持，连接外部工具和数据源',
    category: '协议',
    type: 'plugin',
  },
  {
    id: 'dsh-plugin-translator',
    repo: 'Frank-NF/dsh-plugin-translator',
    name: '翻译增强插件',
    description: '支持上下文感知的多语言翻译',
    category: '效率',
    type: 'plugin',
  },
]

/* ---------- 缓存 ---------- */
const CACHE_TTL = 10 * 60 * 1000 // 10 分钟

let cache: {
  data: PluginData[]
  fetchedAt: number
} | null = null

/* ---------- GitHub 拉取 ---------- */
async function fetchRepo(repo: string): Promise<Partial<PluginData> | null> {
  try {
    const res = await $fetch<any>(`https://api.github.com/repos/${repo}`, {
      timeout: 8000,
      headers: {
        Accept: 'application/vnd.github+json',
        'User-Agent': 'DSH-PluginUpdater-Website',
      },
      retry: 1,
    })
    return {
      stars: res.stargazers_count ?? 0,
      forks: res.forks_count ?? 0,
      github_description: res.description ?? null,
      language: res.language ?? null,
      pushed_at: res.pushed_at ?? null,
      topics: res.topics ?? [],
      fetched: true,
    }
  } catch {
    return null
  }
}

/** 获取全部插件（市场目录 plugins.json 优先 → 注册表兜底，10 分钟缓存） */
export async function getPlugins(): Promise<PluginData[]> {
  // 1. 命中缓存直接返回
  if (cache && Date.now() - cache.fetchedAt < CACHE_TTL) {
    return cache.data
  }

  let results: PluginData[] = []

  // 2. 优先读取官方市场目录（2189+ 款，stars/分类/双语描述为真实数据）
  try {
    const fs = await import('node:fs/promises')
    const raw = JSON.parse(await fs.readFile(CATALOG_PATH, 'utf8'))
    const list: CatalogEntry[] = Array.isArray(raw) ? raw : (raw.plugins ?? [])
    if (list.length) {
      results = list.map((e): PluginData => ({
        id: e.npm || e.name,
        repo: `${e.owner}/${e.name}`,
        name: e.name,
        description: e.description?.zh || e.description?.en || '',
        category: CATEGORY_ZH[e.category] || e.category || '工具',
        type: 'plugin',
        stars: e.stars ?? 0,
        forks: 0,
        github_description: e.description?.en ?? null,
        language: null,
        pushed_at: e.added ?? null,
        github_url: e.url || `https://github.com/${e.owner}/${e.name}`,
        topics: e.npm ? [e.npm] : [],
        fetched: true,
      }))
    }
  } catch {
    // 目录不可用时落回注册表
  }

  // 3. 兜底：目录缺失/为空 → REGISTRY（GitHub 实时数据）
  if (!results.length) {
    results = await Promise.all(
      REGISTRY.map(async (entry): Promise<PluginData> => {
        const r = await fetchRepo(entry.repo)
        return {
          ...entry,
          stars: r?.stars ?? 0,
          forks: r?.forks ?? 0,
          github_description: r?.github_description ?? null,
          language: r?.language ?? null,
          pushed_at: r?.pushed_at ?? null,
          github_url: `https://github.com/${entry.repo}`,
          topics: r?.topics ?? [],
          fetched: r?.fetched ?? false,
        }
      })
    )
  }

  cache = { data: results, fetchedAt: Date.now() }
  return results
}

/** 按 stars 降序排行 */
export async function getRankedPlugins(): Promise<PluginData[]> {
  const plugins = await getPlugins()
  return [...plugins].sort((a, b) => b.stars - a.stars)
}
