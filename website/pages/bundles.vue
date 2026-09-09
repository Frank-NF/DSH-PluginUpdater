<template>
  <div class="bundles-page">
    <div class="bg-glow" aria-hidden="true"></div>
    <section class="page-head">
      <div class="container">
        <h1>行业组合包</h1>
        <p>{{ bundleTotal }} 个精选组合包 · 覆盖 30+ 行业 · 一键安装插件 + MCP + Skill</p>
      </div>
    </section>
    <section class="market">
      <div class="container">
        <div class="market-toolbar">
          <div class="search-box">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/>
            </svg>
            <input v-model="search" type="search" placeholder="搜索组合包…" />
          </div>
          <div class="category-tabs">
            <button class="cat-tab" :class="{ active: activeTag === '全部' }" @click="activeTag = '全部'">全部</button>
            <button v-for="tag in allTags" :key="tag" class="cat-tab" :class="{ active: activeTag === tag }" @click="activeTag = tag">{{ tag }}</button>
          </div>
        </div>
        <div v-if="bundlePending" class="bundle-grid">
          <div v-for="i in 6" :key="i" class="bundle-card card">
            <div class="skeleton sk-title"></div>
            <div class="skeleton sk-desc"></div>
          </div>
        </div>
        <div v-else-if="filtered.length" class="bundle-grid">
          <div v-for="bundle in paged" :key="bundle.id" class="bundle-card card" @click="selectedBundle = bundle">
            <div class="card-head">
              <div class="bundle-icon">📦</div>
              <div class="bundle-info">
                <h3>{{ bundle.name }}</h3>
                <p class="bundle-desc">{{ bundle.description }}</p>
              </div>
            </div>
            <div class="bundle-tags">
              <span v-for="tag in bundle.tags" :key="tag" class="bundle-tag">{{ tag }}</span>
              <span class="bundle-meta">{{ bundle.plugins.length }} 插件 · {{ bundle.mcpServers.length }} MCP</span>
            </div>
          </div>
        </div>
        <div v-else class="empty">
          <div class="empty-icon">🔍</div>
          <h3>没有找到匹配的组合包</h3>
          <p>换个关键词试试</p>
          <button class="btn btn-outline btn-sm" @click="clearFilters">清除筛选</button>
        </div>
        <div v-if="totalPages > 1" class="pager">
          <button class="page-btn" :disabled="page <= 1" @click="gotoPage(page - 1)">← 上一页</button>
          <span class="page-info">第 {{ page }} / {{ totalPages }} 页 · 共 {{ filtered.length }} 个</span>
          <button class="page-btn" :disabled="page >= totalPages" @click="gotoPage(page + 1)">下一页 →</button>
        </div>
      </div>
    </section>
    <!-- 详情弹窗 -->
    <Teleport to="body">
      <div v-if="selectedBundle" class="bundle-overlay" @click.self="selectedBundle = null">
        <div class="bundle-detail glass" role="dialog" aria-modal="true">
          <button class="dialog-close" @click="selectedBundle = null">×</button>
          <div class="detail-header">
            <div class="bundle-icon-lg">📦</div>
            <div>
              <h2>{{ selectedBundle.name }}</h2>
              <p class="detail-desc">{{ selectedBundle.description }}</p>
              <div class="detail-tags">
                <span v-for="tag in selectedBundle.tags" :key="tag" class="detail-tag">{{ tag }}</span>
              </div>
            </div>
          </div>
          <div class="detail-section" v-if="selectedBundle.plugins.length">
            <h4>🔌 插件清单（{{ selectedBundle.plugins.length }} 项）</h4>
            <ul class="detail-list">
              <li v-for="p in selectedBundle.plugins" :key="p.pluginRef">
                <span class="req-badge" :class="p.required ? 'required' : 'optional'">{{ p.required ? '必装' : '可选' }}</span>
                <code>{{ p.pluginRef }}</code>
              </li>
            </ul>
          </div>
          <div class="detail-section" v-if="selectedBundle.mcpServers.length">
            <h4>🧠 MCP 服务（{{ selectedBundle.mcpServers.length }} 项）</h4>
            <ul class="detail-list">
              <li v-for="m in selectedBundle.mcpServers" :key="m.serverId">
                <span class="req-badge" :class="m.optional ? 'optional' : 'required'">{{ m.optional ? '可选' : '必装' }}</span>
                <strong>{{ m.name }}</strong>
                <span>{{ m.description }}</span>
              </li>
            </ul>
          </div>
          <div class="detail-section" v-if="selectedBundle.skills.length">
            <h4>⚡ Skills（{{ selectedBundle.skills.length }} 项）</h4>
            <ul class="detail-list">
              <li v-for="s in selectedBundle.skills" :key="s.skillId">
                <code>{{ s.name }}</code>
              </li>
            </ul>
          </div>
          <div class="detail-footer">
            <a href="/download" class="btn btn-primary">客户端安装</a>
            <button class="btn btn-outline" @click="selectedBundle = null">关闭</button>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
useHead({
  title: '行业组合包 - DSH 插件升级管理',
  meta: [
    {
      name: 'description',
      content:
        '99+ 行业组合包：插件 + MCP 服务模板 + Skill 一键装齐，安装前冲突预检、失败自动回滚。在桌面客户端组合包标签页安装。',
    },
  ],
})
const route = useRoute()
const search = ref('')
const activeTag = ref('全部')
const page = ref(1)
const PAGE_SIZE = 12
const selectedBundle = ref<any>(null)

interface BundleData {
  id: string; name: string; description: string; tags: string[]
  plugins: Array<{ pluginRef: string; required: boolean }>
  mcpServers: Array<{ serverId: string; name: string; description: string; optional: boolean }>
  skills: Array<{ skillId: string; name: string }>
}

const { data: bundlesData, pending: bundlePending } = await useFetch<{ bundles: BundleData[]; total: number }>(
  '/api/bundles?page_size=100',
  { default: () => ({ bundles: [], total: 0 }), lazy: false }
).catch(() => ({ data: ref({ bundles: [], total: 0 }), pending: ref(false) }))

const bundleTotal = computed(() => bundlesData.value?.total ?? 0)

const allTags = computed(() => {
  const tags = new Set<string>()
  for (const b of bundlesData.value?.bundles ?? []) {
    for (const t of b.tags) tags.add(t)
  }
  return Array.from(tags)
})

const filtered = computed(() => {
  let list = bundlesData.value?.bundles ?? []
  const q = search.value.trim().toLowerCase()
  if (q) list = list.filter((b) => b.name.toLowerCase().includes(q) || b.description.toLowerCase().includes(q))
  if (activeTag.value !== '全部') list = list.filter((b) => b.tags.includes(activeTag.value))
  return list
})

const totalPages = computed(() => Math.max(1, Math.ceil(filtered.value.length / PAGE_SIZE)))
const paged = computed(() => {
  const start = (page.value - 1) * PAGE_SIZE
  return filtered.value.slice(start, start + PAGE_SIZE)
})

watch([search, activeTag], () => { page.value = 1 })
function gotoPage(n: number) {
  page.value = Math.min(Math.max(1, n), totalPages.value)
  window.scrollTo({ top: 0, behavior: 'smooth' })
}
function clearFilters() {
  search.value = ''
  activeTag.value = '全部'
}

watch(() => route.hash, (hash) => {
  if (hash) {
    const id = hash.replace('#', '')
    const b = bundlesData.value?.bundles?.find((x) => x.id === id)
    if (b) selectedBundle.value = b
  }
}, { immediate: true })
</script>

<style scoped>
.bundles-page { position: relative; overflow-x: hidden; }
.bg-glow {
  position: absolute; top: -200px; left: 50%; transform: translateX(-50%);
  width: 800px; height: 600px;
  background: radial-gradient(circle, rgba(99,102,241,0.12) 0%, transparent 70%);
  pointer-events: none;
}
.page-head { padding: 64px 0 36px; text-align: center; }
.page-head h1 { font-size: 36px; font-weight: 800; color: var(--text-primary); margin-bottom: 10px; }
.page-head p { font-size: 14px; color: var(--text-muted); }
.market { padding-bottom: 40px; }
.market-toolbar {
  display: flex; align-items: center; justify-content: space-between;
  gap: 16px; margin-bottom: 28px; flex-wrap: wrap;
}
.search-box {
  flex: 1; min-width: 240px; max-width: 380px;
  display: flex; align-items: center; gap: 10px;
  padding: 10px 16px; border-radius: var(--radius-md);
  background: rgba(0,0,0,0.25); border: 1px solid var(--glass-border);
  color: var(--text-muted);
}
.search-box:focus-within { border-color: var(--primary); box-shadow: 0 0 0 3px rgba(99,102,241,0.15); }
.search-box input { flex: 1; background: none; border: none; outline: none; color: var(--text-primary); font-size: 14px; }
.category-tabs { display: flex; gap: 8px; flex-wrap: wrap; }
.cat-tab {
  padding: 7px 16px; border-radius: 20px; font-size: 13px; font-weight: 500; cursor: pointer;
  background: rgba(255,255,255,0.04); border: 1px solid var(--glass-border);
  color: var(--text-secondary); transition: all var(--dur) var(--ease);
}
.cat-tab:hover { background: rgba(255,255,255,0.08); color: var(--text-primary); }
.cat-tab.active { background: var(--primary); border-color: var(--primary); color: #fff; }
.bundle-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 18px; }
.bundle-card {
  display: flex; flex-direction: column; padding: 22px; cursor: pointer;
  transition: transform 0.2s, box-shadow 0.2s;
}
.bundle-card:hover { transform: translateY(-3px); box-shadow: 0 8px 30px rgba(99,102,241,0.15); }
.card-head { display: flex; align-items: flex-start; gap: 12px; margin-bottom: 12px; }
.bundle-icon, .bundle-icon-lg {
  width: 48px; height: 48px; border-radius: 12px;
  background: rgba(99,102,241,0.14); border: 1px solid rgba(99,102,241,0.3);
  display: flex; align-items: center; justify-content: center; font-size: 22px; flex-shrink: 0;
}
.bundle-info h3 { font-size: 15px; font-weight: 600; color: var(--text-primary); margin-bottom: 4px; }
.bundle-desc {
  font-size: 13px; color: var(--text-secondary); line-height: 1.5;
  display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden;
}
.bundle-tags {
  display: flex; align-items: center; gap: 8px; flex-wrap: wrap; margin-top: auto; padding-top: 12px;
}
.bundle-tag {
  padding: 3px 10px; border-radius: 12px; font-size: 11px;
  background: rgba(99,102,241,0.12); color: var(--primary-light);
}
.bundle-meta { font-size: 12px; color: var(--text-muted); margin-left: auto; }
.empty { text-align: center; padding: 80px 20px; }
.empty-icon { font-size: 48px; margin-bottom: 16px; opacity: 0.6; }
.pager { display: flex; align-items: center; justify-content: center; gap: 18px; margin-top: 34px; }
.page-btn {
  padding: 9px 20px; border-radius: 10px; border: 1px solid var(--glass-border);
  background: var(--card); color: var(--text-primary); cursor: pointer; font-size: 14px;
}
.page-btn:disabled { opacity: 0.35; cursor: not-allowed; }
.page-info { color: var(--text-muted); font-size: 13px; }
.skeleton {
  background: linear-gradient(90deg, rgba(255,255,255,0.04) 25%, rgba(255,255,255,0.08) 50%, rgba(255,255,255,0.04) 75%);
  background-size: 200% 100%; animation: shimmer 1.5s infinite; border-radius: 6px;
}
.sk-title { height: 16px; width: 60%; margin-bottom: 12px; }
.sk-desc { height: 12px; width: 90%; }
@keyframes shimmer { 0% { background-position: 200% 0; } 100% { background-position: -200% 0; } }
.bundle-overlay {
  position: fixed; inset: 0; z-index: 200;
  background: rgba(0,0,0,0.6); backdrop-filter: blur(4px);
  display: flex; align-items: center; justify-content: center; padding: 24px;
}
.bundle-detail {
  position: relative; width: 600px; max-width: 100%; max-height: 85vh;
  overflow-y: auto; padding: 28px; border-radius: var(--radius-xl); background: var(--bg-secondary);
}
.dialog-close {
  position: absolute; top: 14px; right: 16px; width: 30px; height: 30px;
  border-radius: 8px; background: rgba(255,255,255,0.06); border: 1px solid var(--glass-border);
  color: var(--text-muted); font-size: 17px; cursor: pointer;
}
.detail-header { display: flex; gap: 16px; margin-bottom: 24px; }
.bundle-icon-lg { width: 64px; height: 64px; font-size: 30px; }
.detail-header h2 { font-size: 20px; font-weight: 700; color: var(--text-primary); margin-bottom: 6px; }
.detail-desc { font-size: 13px; color: var(--text-secondary); margin-bottom: 10px; }
.detail-tags { display: flex; gap: 6px; flex-wrap: wrap; }
.detail-tag {
  padding: 3px 10px; border-radius: 12px; font-size: 11px;
  background: rgba(99,102,241,0.12); color: var(--primary-light);
}
.detail-section { margin-bottom: 20px; }
.detail-section h4 { font-size: 14px; font-weight: 600; color: var(--text-primary); margin-bottom: 12px; }
.detail-list { list-style: none; padding: 0; margin: 0; display: flex; flex-direction: column; gap: 8px; }
.detail-list li {
  display: flex; align-items: center; gap: 10px; font-size: 13px; color: var(--text-secondary);
  padding: 8px 12px; border-radius: 8px; background: rgba(255,255,255,0.03);
}
.detail-list li code { font-family: monospace; font-size: 12px; color: var(--primary-light); }
.detail-list li strong { color: var(--text-primary); font-weight: 600; }
.req-badge {
  padding: 2px 8px; border-radius: 10px; font-size: 11px; font-weight: 600; flex-shrink: 0;
}
.req-badge.required { background: rgba(16,185,129,0.15); color: #10b981; }
.req-badge.optional { background: rgba(245,158,11,0.15); color: #f59e0b; }
.detail-footer {
  display: flex; gap: 12px; justify-content: flex-end; margin-top: 24px;
  padding-top: 20px; border-top: 1px solid var(--glass-border);
}
@media (max-width: 640px) {
  .page-head h1 { font-size: 28px; }
  .market-toolbar { flex-direction: column; align-items: stretch; }
  .search-box { max-width: none; }
}
</style>
