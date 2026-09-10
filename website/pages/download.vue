<template>
  <div class="download-page">
    <div class="bg-glow" aria-hidden="true"></div>

    <section class="page-head">
      <div class="container">
        <h1>下载中心</h1>
        <p>桌面客户端单文件运行 · 全链路 Ed25519 签名验证 · v{{ winVersion }} 安全加固版</p>
      </div>
    </section>

    <section class="download-body">
      <div class="container">
        <!-- 桌面客户端 -->
        <div class="section-title">
          <h2>桌面客户端</h2>
          <p>选择适合您平台的版本</p>
        </div>

        <div class="download-grid">
          <div class="download-card card">
            <div class="download-icon win">🪟</div>
            <h3>Windows 版</h3>
            <p class="version">v{{ winVersion }} · 64位 · EXE 安装包</p>
            <ul class="download-info">
              <li>支持 Windows 10/11</li>
              <li>单文件安装，自动创建快捷方式</li>
              <li>大小约 {{ winSizeMB }}MB</li>
            </ul>
            <a :href="winUrl" download class="btn btn-primary download-btn">
              下载 .exe
            </a>
            <p class="download-hash mono" :title="winSha256">SHA256: {{ winSha256Short }}</p>
          </div>

          <div class="download-card card">
            <div class="download-icon linux">🐧</div>
            <h3>Linux 版</h3>
            <template v-if="linuxReady">
              <p class="version">v{{ linuxVersion }} · AppImage · x86_64</p>
              <ul class="download-info">
                <li>支持 Ubuntu 20.04+ / Debian 11+</li>
                <li>AppImage 格式，开箱即用</li>
                <li>大小约 {{ linuxSizeMB }}MB</li>
              </ul>
              <a :href="linuxUrl" download class="btn btn-primary download-btn">下载 .AppImage</a>
              <p class="download-hash mono" :title="linuxSha256">SHA256: {{ linuxSha256Short }}</p>
            </template>
            <template v-else>
              <p class="version">AppImage · x86_64</p>
              <ul class="download-info">
                <li>支持 Ubuntu 20.04+ / Debian 11+</li>
                <li>AppImage 格式，开箱即用</li>
              </ul>
              <a class="btn btn-primary download-btn is-disabled" aria-disabled="true">暂未提供</a>
              <p class="download-hash mono">可先使用 Windows 版</p>
            </template>
          </div>
        </div>

        <!-- 一键唤起（dshupdater:// 协议） -->
        <div class="section-title" style="margin-top: 56px">
          <h2>已在用桌面客户端？</h2>
          <p>安装最新版工具后，可直接从浏览器唤起</p>
        </div>
        <div class="online-banner card">
          <div class="online-info">
            <div class="online-icon">🚀</div>
            <div>
              <h3>一键唤起工具</h3>
              <p>点击下方按钮直接打开桌面客户端（首次使用请先升级到 v1.10.0 及以上，工具会自动注册唤起协议）</p>
            </div>
          </div>
          <div style="display: flex; gap: 12px; flex-wrap: wrap">
            <a href="dshupdater://open" class="btn btn-primary">打开工具</a>
            <a href="dshupdater://check-updates" class="btn" style="background: var(--card); color: var(--text); border: 1px solid var(--border)">检查插件更新</a>
            <NuxtLink to="/offline" class="btn" style="background: var(--card); color: var(--text); border: 1px solid var(--border)">离线部署指引 →</NuxtLink>
          </div>
        </div>

        <!-- 安装说明 -->
        <div class="install-guide">
          <h3>安装说明</h3>
          <div class="guide-grid">
            <div class="guide-item card">
              <h4>Windows</h4>
              <ol>
                <li>下载 .exe 安装包</li>
                <li>双击运行安装程序</li>
                <li>按照向导完成安装</li>
                <li>从桌面或开始菜单启动</li>
              </ol>
            </div>
            <div class="guide-item card">
              <h4>Linux</h4>
              <ol>
                <li>下载 .AppImage 文件</li>
                <li>
                  添加执行权限: <code>chmod +x DSH-*.AppImage</code>
                </li>
                <li>双击运行或命令行执行</li>
                <li>可选：移动到应用目录</li>
              </ol>
            </div>
            <div class="guide-item card">
              <h4>首次使用</h4>
              <ol>
                <li>启动后点击「自动扫描」定位插件目录</li>
                <li>到「组合包」标签页按行业一键安装套件</li>
                <li>检查更新并按需升级插件（更新前自动备份）</li>
                <li>「设置 → MCP 服务管理」集中配置 MCP 密钥</li>
              </ol>
            </div>
          </div>
        </div>

        <!-- 系统要求（id=security：首页「V3 安全体系」卡片深链至此） -->
        <div id="security" class="sys-req card">
          <h3>系统要求</h3>
          <div class="req-grid">
            <div class="req-item">
              <span class="req-label">Windows</span>
              <span>Windows 10 1903+ / Windows 11，x64</span>
            </div>
            <div class="req-item">
              <span class="req-label">Linux</span>
              <span>Ubuntu 20.04+ / Debian 11+，x86_64</span>
            </div>
            <div class="req-item">
              <span class="req-label">网络</span>
              <span>访问 GitHub 与 npm 官方源，内置直连加速</span>
            </div>
            <div class="req-item">
              <span class="req-label">安全</span>
              <span>目录 / 自更新清单 Ed25519 验签，更新包 SHA256 校验</span>
            </div>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
interface PlatformAsset {
  version?: string
  url?: string
  sha256?: string
  size_bytes?: number
}
interface LatestInfo {
  version: string
  platforms: Record<string, PlatformAsset>
  changelog: string[]
  published_at: string | null
}

// 最新发布信息：单一权威源 /api/updater/latest（服务器 version.json）
const { data: latest } = await useFetch<LatestInfo>('/api/updater/latest')

const win = computed(() => latest.value?.platforms?.windows || null)
const linux = computed(() => latest.value?.platforms?.linux || null)

const winVersion = computed(() => win.value?.version || latest.value?.version || '1.0.0')
const winUrl = computed(() => win.value?.url || '/dsh-plugin-updater.exe')
const winSha256 = computed(() => win.value?.sha256 || '')
const winSha256Short = computed(() => (win.value?.sha256 || '').slice(0, 16) + '…')
const winSizeMB = computed(() =>
  win.value?.size_bytes ? Math.round((win.value.size_bytes / 1024 / 1024) * 10) / 10 : '—'
)

const linuxReady = computed(() => !!linux.value?.url)
const linuxVersion = computed(() => linux.value?.version || latest.value?.version || '')
const linuxUrl = computed(() => linux.value?.url || '#')
const linuxSha256 = computed(() => linux.value?.sha256 || '')
const linuxSha256Short = computed(() => (linux.value?.sha256 || '').slice(0, 16) + '…')
const linuxSizeMB = computed(() =>
  linux.value?.size_bytes ? Math.round((linux.value.size_bytes / 1024 / 1024) * 10) / 10 : '—'
)

useSiteSeo({
  title: '下载中心',
  description: '下载 DSH插件管家 Windows 安装包：独立运行、2189+ 插件市场、行业组合包一键安装。提供官方 SHA256 校验值，下载前可验证文件完整性。',
  path: '/download',
})

// SoftwareApplication 结构化数据：搜索引擎富摘要（版本/价格/评分）
useHead({
  script: [
    {
      type: 'application/ld+json',
      innerHTML: JSON.stringify({
        '@context': 'https://schema.org',
        '@type': 'SoftwareApplication',
        name: 'DSH插件管家',
        operatingSystem: 'Windows 10/11, Linux',
        applicationCategory: 'DeveloperApplication',
        offers: { '@type': 'Offer', price: '0', priceCurrency: 'CNY' },
        description: 'DeepSeek Harness 生态插件管理桌面工具：2189+ 插件市场、行业组合包、MCP 服务管理、快照与离线部署。',
      }),
    },
  ],
})
</script>

<style scoped>
.download-page {
  position: relative;
  overflow-x: hidden;
}

.bg-glow {
  position: absolute;
  top: -200px;
  left: 50%;
  transform: translateX(-50%);
  width: 800px;
  height: 600px;
  background: radial-gradient(circle, rgba(99, 102, 241, 0.12) 0%, transparent 70%);
  pointer-events: none;
}

.download-page > section {
  position: relative;
}

.page-head {
  padding: 64px 0 36px;
  text-align: center;
}

.page-head h1 {
  font-size: 36px;
  font-weight: 800;
  color: var(--text-primary);
  margin-bottom: 10px;
}

.page-head p {
  font-size: 14px;
  color: var(--text-muted);
}

.download-body {
  padding-bottom: 40px;
}

/* ---------- 唤起横幅（复用卡片样式） ---------- */
.online-banner {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 24px;
  padding: 28px 32px;
  border-color: rgba(16, 185, 129, 0.35);
  background: rgba(16, 185, 129, 0.06);
  flex-wrap: wrap;
}

.online-info {
  display: flex;
  align-items: center;
  gap: 18px;
  min-width: 0;
}

.online-icon {
  width: 56px;
  height: 56px;
  border-radius: 14px;
  background: rgba(16, 185, 129, 0.15);
  border: 1px solid rgba(16, 185, 129, 0.3);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 26px;
  flex-shrink: 0;
}

.online-info h3 {
  font-size: 17px;
  font-weight: 700;
  color: var(--text-primary);
  margin-bottom: 4px;
}

.online-info p {
  font-size: 13px;
  color: var(--text-secondary);
}

/* ---------- 下载卡片 ---------- */
.download-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 24px;
  max-width: 820px;
  margin: 0 auto;
}

.download-card {
  padding: 36px 30px;
  text-align: center;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.download-icon {
  width: 68px;
  height: 68px;
  border-radius: 17px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 32px;
  margin-bottom: 18px;
}

.download-icon.win {
  background: rgba(99, 102, 241, 0.14);
  border: 1px solid rgba(99, 102, 241, 0.3);
}

.download-icon.linux {
  background: rgba(245, 158, 11, 0.12);
  border: 1px solid rgba(245, 158, 11, 0.3);
}

.download-card h3 {
  font-size: 19px;
  font-weight: 700;
  color: var(--text-primary);
  margin-bottom: 5px;
}

.version {
  font-size: 13px;
  color: var(--text-muted);
  margin-bottom: 18px;
}

.download-info {
  list-style: none;
  text-align: left;
  margin-bottom: 24px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.download-info li {
  font-size: 13px;
  color: var(--text-secondary);
  padding-left: 22px;
  position: relative;
}

.download-info li::before {
  content: '✓';
  position: absolute;
  left: 0;
  color: var(--accent);
  font-weight: 700;
}

.download-btn {
  width: 100%;
  margin-top: auto;
}

.download-btn.is-disabled {
  opacity: 0.5;
  pointer-events: none;
}

.download-hash {
  margin-top: 14px;
  font-size: 11px;
  color: var(--text-muted);
}

/* ---------- 安装说明 ---------- */
.install-guide {
  margin-top: 64px;
}

.install-guide h3 {
  font-size: 20px;
  font-weight: 700;
  color: var(--text-primary);
  text-align: center;
  margin-bottom: 28px;
}

.guide-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 20px;
}

.guide-item {
  padding: 24px;
}

.guide-item h4 {
  font-size: 15px;
  font-weight: 600;
  color: var(--primary-light);
  margin-bottom: 14px;
}

.guide-item ol {
  padding-left: 20px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.guide-item li {
  font-size: 13px;
  color: var(--text-secondary);
  line-height: 1.6;
}

.guide-item code {
  font-size: 12px;
  color: var(--primary-light);
  background: rgba(0, 0, 0, 0.3);
  padding: 2px 7px;
  border-radius: 5px;
}

/* ---------- 系统要求 ---------- */
.sys-req {
  margin-top: 40px;
  padding: 28px 32px;
}

.sys-req h3 {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 18px;
}

.req-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
  gap: 18px;
}

.req-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.req-label {
  font-size: 13px;
  font-weight: 600;
  color: var(--primary-light);
}

.req-item span:last-child {
  font-size: 13px;
  color: var(--text-secondary);
}
</style>
