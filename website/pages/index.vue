<template>
  <div class="home">
    <!-- 背景光晕 -->
    <div class="bg-glow" aria-hidden="true"></div>

    <!-- Hero -->
    <section class="hero">
      <div class="container hero-content">
        <div class="hero-badge">
          <span class="dot"></span>
          v1.14.0 · 安全加固版已发布
        </div>
        <h1>
          DSH 插件升级管理
          <span class="gradient-text">一站式插件生态</span>
        </h1>
        <p class="hero-desc">
          浏览 2189+ 插件市场、一键安装行业组合包，或在桌面客户端完成扫描、
          更新、启停、卸载与 MCP 服务管理——全链路 Ed25519 签名验证，防篡改。
        </p>
        <div class="hero-buttons">
          <NuxtLink to="/plugins" class="btn btn-primary">
            浏览插件市场
          </NuxtLink>
          <NuxtLink to="/bundles" class="btn btn-primary">
            行业组合包
          </NuxtLink>
          <NuxtLink to="/download" class="btn btn-outline">
            下载桌面客户端
          </NuxtLink>
        </div>

        <!-- 实时统计条 -->
        <div class="stats-bar glass">
          <div class="stat">
            <span class="stat-value">{{ stats?.total_plugins ?? '—' }}</span>
            <span class="stat-label">收录插件</span>
          </div>
          <div class="stat-divider"></div>
          <div class="stat">
            <span class="stat-value">{{ stats ? formatStars(stats.total_stars) : '—' }}</span>
            <span class="stat-label">GitHub Stars</span>
          </div>
          <div class="stat-divider"></div>
          <div class="stat">
            <span class="stat-value">99+</span>
            <span class="stat-label">行业组合包</span>
          </div>
          <div class="stat-divider"></div>
          <div class="stat">
            <span class="stat-value" :class="{ live: true }">已验证</span>
            <span class="stat-label">签名保护</span>
          </div>
        </div>
      </div>
    </section>

    <!-- 热门排行 -->
    <section class="section">
      <div class="container">
        <div class="section-title">
          <h2>热门排行</h2>
          <p>按 GitHub Star 实时排名 · 每 10 分钟自动刷新</p>
        </div>

        <div v-if="pending" class="rank-grid">
          <div v-for="i in 6" :key="i" class="rank-card card">
            <div class="skeleton rank-skeleton-icon"></div>
            <div class="rank-info">
              <div class="skeleton rank-skeleton-line"></div>
              <div class="skeleton rank-skeleton-line short"></div>
            </div>
            <div class="skeleton rank-skeleton-star"></div>
          </div>
        </div>

        <div v-else class="rank-grid">
          <a
            v-for="(plugin, index) in topPlugins"
            :key="plugin.id"
            :href="plugin.github_url"
            target="_blank"
            rel="noopener"
            class="rank-card card"
          >
            <div class="rank-num" :class="`rank-${index + 1}`">{{ index + 1 }}</div>
            <div class="rank-info">
              <h3>{{ plugin.name }}</h3>
              <p>{{ plugin.description }}</p>
            </div>
            <div class="rank-stars" title="GitHub Stars">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
                <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z" />
              </svg>
              {{ plugin.stars }}
            </div>
          </a>
        </div>

        <div class="section-more">
          <NuxtLink to="/plugins" class="btn btn-outline btn-sm">查看全部插件 →</NuxtLink>
        </div>
      </div>
    </section>

    <!-- 双端形态 -->
    <section class="section">
      <div class="container">
        <div class="section-title">
          <h2>桌面客户端 · 功能完整</h2>
          <p>Windows / Linux 单文件运行，无需安装</p>
        </div>
        <div class="modes-grid">
          <div class="mode-card card featured">
            <div class="mode-badge">推荐</div>
            <div class="mode-icon desktop">🖥️</div>
            <h3>桌面客户端</h3>
            <p class="mode-tagline">插件 · 组合包 · MCP · 快照 · 离线部署</p>
            <ul class="mode-features">
              <li>本地插件目录智能扫描，一键更新自动备份</li>
              <li>行业组合包安装（插件 + MCP 模板 + Skill）</li>
              <li>MCP 服务集中管理，密钥加密存系统凭据库</li>
              <li>快照导出 / 离线打包，隔离环境批量部署</li>
              <li>本地 DSH Web 服务器启停面板</li>
            </ul>
            <NuxtLink to="/download" class="btn btn-primary mode-btn">
              下载客户端
            </NuxtLink>
          </div>

          <div class="mode-card card">
            <div class="mode-icon web">🛡️</div>
            <h3>V3 安全体系</h3>
            <p class="mode-tagline">全链路 Ed25519 签名验证</p>
            <ul class="mode-features">
              <li>目录与自更新清单响应体逐字节验签</li>
              <li>签名失败自动拒绝 + 降级本地缓存</li>
              <li>更新包 SHA256 完整性校验</li>
              <li>密钥从未入库，轮换流程版本化</li>
            </ul>
            <NuxtLink to="/docs#security" class="btn btn-outline mode-btn">
              了解安全设计 →
            </NuxtLink>
          </div>
        </div>
      </div>
    </section>

    <!-- 核心能力 -->
    <section class="section">
      <div class="container">
        <div class="section-title">
          <h2>核心能力</h2>
          <p>专为 DSH 生态打造的插件管理解决方案</p>
        </div>
        <div class="features-grid">
          <div v-for="f in features" :key="f.title" class="feature-card card">
            <div class="feature-icon" :class="f.color">{{ f.icon }}</div>
            <h3>{{ f.title }}</h3>
            <p>{{ f.desc }}</p>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
useHead({
  title: 'DSH 插件升级管理 - 官方网站 | 插件市场 · 组合包 · 桌面客户端',
})
interface PluginData {
  id: string
  name: string
  description: string
  category: string
  stars: number
  github_url: string
}

interface Stats {
  total_plugins: number
  total_stars: number
  github_data_ok: boolean
}

// 服务端渲染时直接取数据（SEO 友好），失败不阻塞页面
const { data: pluginsData, pending } = await useFetch<{
  plugins: PluginData[]
}>('/api/plugins', {
  default: () => ({ plugins: [] }),
  lazy: false,
}).catch(() => ({ data: ref({ plugins: [] }), pending: ref(false) }))

const { data: stats } = await useFetch<Stats>('/api/stats', {
  default: () => null,
}).catch(() => ({ data: ref(null) }))

const topPlugins = computed(() => (pluginsData.value?.plugins ?? []).slice(0, 6))

function formatStars(n: number): string {
  if (n >= 1000) return `${(n / 1000).toFixed(1)}k`
  return String(n)
}

const features = [
  { icon: '🔍', color: 'blue', title: '智能扫描', desc: '自动扫描插件目录，识别所有已装插件与 Agent 本体，信息一目了然。' },
  { icon: '📦', color: 'green', title: '组合包安装', desc: '99+ 行业组合包（插件 + MCP 模板 + Skill），安装前冲突预检、失败自动回滚。' },
  { icon: '⬆️', color: 'green', title: '一键更新', desc: 'npm/GitHub 双源检测最新版本，更新前自动备份，支持随时回滚。' },
  { icon: '🛡️', color: 'red', title: '签名验证', desc: '目录与自更新清单 Ed25519 全链路验签，签名失败拒绝消费并降级缓存。' },
  { icon: '🎛️', color: 'orange', title: 'MCP 服务管理', desc: 'MCP 集中配置，密钥加密存系统凭据库，探活检测连通性，一键写入运行配置。' },
  { icon: '📸', color: 'purple', title: '快照与离线', desc: '导出插件环境快照比对差异；离线打包自带全部文件，隔离环境两步部署。' },
  { icon: '🖥️', color: 'cyan', title: '服务器面板', desc: '本地 DSH Web 服务器启动/停止/重启，自动探测路径，带 token 地址一键直达。' },
  { icon: '🗑️', color: 'red', title: '安全卸载', desc: '卸载前自动备份可随时恢复，Agent 本体受保护防误删。' },
]
</script>

<style scoped>
/* ---------- 背景光晕 ---------- */
.home {
  position: relative;
  overflow-x: hidden;
}

.bg-glow {
  position: absolute;
  top: -260px;
  left: 50%;
  transform: translateX(-50%);
  width: 1000px;
  height: 1000px;
  background: radial-gradient(
    circle,
    rgba(99, 102, 241, 0.15) 0%,
    rgba(99, 102, 241, 0.04) 40%,
    transparent 70%
  );
  pointer-events: none;
}

.home > section {
  position: relative;
}

/* ---------- Hero ---------- */
.hero {
  padding: 110px 0 70px;
  text-align: center;
}

.hero-badge {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 6px 16px;
  border-radius: 20px;
  background: rgba(99, 102, 241, 0.12);
  border: 1px solid rgba(99, 102, 241, 0.3);
  color: var(--primary-light);
  font-size: 13px;
  font-weight: 500;
  margin-bottom: 28px;
}

.hero-badge .dot {
  width: 7px;
  height: 7px;
  border-radius: 50%;
  background: var(--accent);
  box-shadow: 0 0 8px rgba(16, 185, 129, 0.9);
}

.hero h1 {
  font-size: 52px;
  font-weight: 800;
  line-height: 1.15;
  margin-bottom: 22px;
  color: var(--text-primary);
}

.gradient-text {
  display: block;
  background: linear-gradient(135deg, var(--primary-light), #a5b4fc);
  -webkit-background-clip: text;
  background-clip: text;
  -webkit-text-fill-color: transparent;
}

.hero-desc {
  font-size: 17px;
  color: var(--text-secondary);
  max-width: 640px;
  margin: 0 auto 36px;
  line-height: 1.7;
}

.hero-buttons {
  display: flex;
  gap: 14px;
  justify-content: center;
  flex-wrap: wrap;
  margin-bottom: 48px;
}

.hero-buttons .btn {
  padding: 13px 28px;
  font-size: 15px;
}

.online-dot {
  width: 7px;
  height: 7px;
  border-radius: 50%;
  background: var(--accent);
  box-shadow: 0 0 6px rgba(16, 185, 129, 0.8);
}

/* 统计条 */
.stats-bar {
  display: inline-flex;
  align-items: center;
  gap: 0;
  padding: 20px 40px;
  border-radius: var(--radius-xl);
}

.stat {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2px;
  min-width: 130px;
}

.stat-value {
  font-size: 26px;
  font-weight: 700;
  color: var(--primary-light);
  font-family: 'JetBrains Mono', 'Consolas', monospace;
}

.stat-value.live {
  color: var(--accent);
  font-family: inherit;
  font-size: 22px;
}

.stat-label {
  font-size: 12px;
  color: var(--text-muted);
}

.stat-divider {
  width: 1px;
  height: 36px;
  background: var(--glass-border);
  margin: 0 24px;
}

/* ---------- 通用 section ---------- */
.section {
  padding: 70px 0;
}

.section-more {
  text-align: center;
  margin-top: 36px;
}

/* ---------- 热门排行 ---------- */
.rank-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
  gap: 16px;
}

.rank-card {
  display: flex;
  align-items: center;
  gap: 14px;
  padding: 18px 20px;
  color: inherit;
}

.rank-num {
  flex-shrink: 0;
  width: 32px;
  height: 32px;
  border-radius: 9px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 700;
  font-size: 14px;
  color: var(--text-muted);
  background: rgba(255, 255, 255, 0.06);
  border: 1px solid var(--glass-border);
}

.rank-num.rank-1 {
  color: #fbbf24;
  background: rgba(245, 158, 11, 0.15);
  border-color: rgba(245, 158, 11, 0.35);
}

.rank-num.rank-2 {
  color: #cbd5e1;
  background: rgba(148, 163, 184, 0.15);
  border-color: rgba(148, 163, 184, 0.35);
}

.rank-num.rank-3 {
  color: #f9a66c;
  background: rgba(234, 88, 12, 0.15);
  border-color: rgba(234, 88, 12, 0.35);
}

.rank-info {
  flex: 1;
  min-width: 0;
}

.rank-info h3 {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 3px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.rank-info p {
  font-size: 12px;
  color: var(--text-muted);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.rank-stars {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  gap: 5px;
  font-size: 13px;
  font-weight: 600;
  color: var(--warning);
  font-family: 'JetBrains Mono', 'Consolas', monospace;
}

/* 骨架屏 */
.rank-skeleton-icon {
  width: 32px;
  height: 32px;
  border-radius: 9px;
  flex-shrink: 0;
}

.rank-info {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.rank-skeleton-line {
  height: 13px;
  width: 70%;
}

.rank-skeleton-line.short {
  width: 90%;
}

.rank-skeleton-star {
  width: 44px;
  height: 16px;
  flex-shrink: 0;
}

/* ---------- 双端形态 ---------- */
.modes-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
  gap: 24px;
  max-width: 860px;
  margin: 0 auto;
}

.mode-card {
  position: relative;
  padding: 36px 32px;
  text-align: center;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.mode-card.featured {
  border-color: rgba(99, 102, 241, 0.45);
}

.mode-badge {
  position: absolute;
  top: -11px;
  right: 24px;
  padding: 3px 14px;
  border-radius: 20px;
  background: linear-gradient(135deg, var(--primary), var(--primary-light));
  color: #fff;
  font-size: 12px;
  font-weight: 600;
  box-shadow: 0 4px 12px rgba(99, 102, 241, 0.4);
}

.mode-icon {
  width: 64px;
  height: 64px;
  border-radius: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 30px;
  margin-bottom: 20px;
}

.mode-icon.web {
  background: rgba(16, 185, 129, 0.14);
  border: 1px solid rgba(16, 185, 129, 0.3);
}

.mode-icon.desktop {
  background: rgba(99, 102, 241, 0.14);
  border: 1px solid rgba(99, 102, 241, 0.3);
}

.mode-card h3 {
  font-size: 19px;
  font-weight: 700;
  color: var(--text-primary);
  margin-bottom: 6px;
}

.mode-tagline {
  font-size: 13px;
  color: var(--text-muted);
  margin-bottom: 20px;
}

.mode-features {
  list-style: none;
  text-align: left;
  margin-bottom: 26px;
  display: flex;
  flex-direction: column;
  gap: 9px;
}

.mode-features li {
  font-size: 13px;
  color: var(--text-secondary);
  padding-left: 22px;
  position: relative;
}

.mode-features li::before {
  content: '✓';
  position: absolute;
  left: 0;
  color: var(--accent);
  font-weight: 700;
}

.mode-btn {
  margin-top: auto;
  width: 100%;
}

/* ---------- 核心能力 ---------- */
.features-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 20px;
}

.feature-card {
  padding: 28px;
}

.feature-icon {
  width: 50px;
  height: 50px;
  border-radius: 13px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 24px;
  margin-bottom: 16px;
}

.feature-icon.blue {
  background: linear-gradient(135deg, #6366f1, #818cf8);
}
.feature-icon.green {
  background: linear-gradient(135deg, #10b981, #34d399);
}
.feature-icon.orange {
  background: linear-gradient(135deg, #f59e0b, #fbbf24);
}
.feature-icon.red {
  background: linear-gradient(135deg, #ef4444, #f87171);
}
.feature-icon.purple {
  background: linear-gradient(135deg, #8b5cf6, #a78bfa);
}
.feature-icon.cyan {
  background: linear-gradient(135deg, #06b6d4, #22d3ee);
}

.feature-card h3 {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 8px;
}

.feature-card p {
  font-size: 13px;
  color: var(--text-secondary);
  line-height: 1.7;
}

/* ---------- 响应式 ---------- */
@media (max-width: 768px) {
  .hero {
    padding: 70px 0 50px;
  }
  .hero h1 {
    font-size: 34px;
  }
  .stats-bar {
    padding: 16px 20px;
    width: 100%;
    justify-content: space-around;
  }
  .stat-divider {
    margin: 0 8px;
  }
  .stat {
    min-width: 0;
    flex: 1;
  }
  .section {
    padding: 50px 0;
  }
}
</style>
