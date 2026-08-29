import { ref, computed } from 'vue'
import { zh } from './zh'
import { en } from './en'

export type Locale = 'zh' | 'en'

const LOCALE_KEY = 'dsh-updater-locale'

function detectLocale(): Locale {
  // 手动选择优先（HeaderBar 中/EN 按钮）
  const saved = localStorage.getItem(LOCALE_KEY)
  if (saved === 'zh' || saved === 'en') return saved
  // 默认中文界面；系统语言为中文时也自动中文（英文用户可手动切 EN）
  const lang = (navigator.language || '').toLowerCase()
  return lang.startsWith('zh') ? 'zh' : 'zh'
}

export const locale = ref<Locale>(detectLocale())

export function setLocale(l: Locale) {
  locale.value = l
  localStorage.setItem(LOCALE_KEY, l)
  document.documentElement.lang = l === 'zh' ? 'zh-CN' : 'en'
}

export function toggleLocale() {
  setLocale(locale.value === 'zh' ? 'en' : 'zh')
}

const dicts = { zh, en }

/** 翻译：t('scan.complete', { n: 8 }) → 词典条目里的 {n} 会被替换 */
export function t(key: string, params?: Record<string, string | number>): string {
  const dict = dicts[locale.value]
  let text = dict[key] ?? dicts.zh[key] ?? key
  if (params) {
    for (const [k, v] of Object.entries(params)) {
      text = text.split('{' + k + '}').join(String(v))
    }
  }
  return text
}

/** 是否中文环境（用于选择插件描述语言） */
export const isZh = computed(() => locale.value === 'zh')

/** 官方目录 21 种分类：id → 双语名 */
export const CATEGORY_NAMES: Record<string, { zh: string; en: string }> = {
  ui: { zh: '界面', en: 'UI' },
  usage: { zh: '使用', en: 'Usage' },
  theme: { zh: '主题', en: 'Theme' },
  model: { zh: '模型', en: 'Model' },
  identity: { zh: '身份', en: 'Identity' },
  session: { zh: '会话', en: 'Session' },
  memory: { zh: '记忆', en: 'Memory' },
  tools: { zh: '工具', en: 'Tools' },
  browser: { zh: '浏览器', en: 'Browser' },
  vision: { zh: '视觉', en: 'Vision' },
  voice: { zh: '语音', en: 'Voice' },
  docs: { zh: '文档', en: 'Docs' },
  skill: { zh: '技能', en: 'Skill' },
  workflow: { zh: '工作流', en: 'Workflow' },
  git: { zh: 'Git', en: 'Git' },
  notify: { zh: '通知', en: 'Notify' },
  dev: { zh: '开发', en: 'Dev' },
  security: { zh: '安全', en: 'Security' },
  remote: { zh: '远程', en: 'Remote' },
  market: { zh: '市场', en: 'Market' },
  fun: { zh: '娱乐', en: 'Fun' },
}

/** 分类标签颜色（21 色循环） */
const CAT_COLORS = [
  '#409eff', '#67c23a', '#e6a23c', '#f56c6c', '#909399',
  '#9b59b6', '#1abc9c', '#e74c3c', '#3498db', '#f39c12',
  '#2ecc71', '#e91e63', '#00bcd4', '#8e44ad', '#d35400',
  '#16a085', '#c0392b', '#2980b9', '#7f8c8d', '#27ae60',
  '#f1c40f',
]
export function categoryColor(id: string | null | undefined): string {
  if (!id) return '#909399'
  let h = 0
  for (const ch of id) h = (h * 31 + ch.charCodeAt(0)) >>> 0
  return CAT_COLORS[h % CAT_COLORS.length]
}

/** 数字友好化：12453 → 1.2w / 12.5k；<1000 原样 */
export function formatCount(n: number | null | undefined): string {
  if (n == null) return '—'
  if (n >= 100000000) return (n / 100000000).toFixed(1).replace(/\.0$/, '') + (locale.value === 'zh' ? '亿' : 'B')
  if (n >= 10000) return (n / 10000).toFixed(1).replace(/\.0$/, '') + (locale.value === 'zh' ? 'w' : 'k')
  if (n >= 1000) return (n / 1000).toFixed(1).replace(/\.0$/, '') + 'k'
  return String(n)
}

/** 分类显示名（按当前语言） */
export function categoryName(id: string | null | undefined): string {
  if (!id) return ''
  const c = CATEGORY_NAMES[id]
  if (!c) return id
  return locale.value === 'zh' ? c.zh : c.en
}
