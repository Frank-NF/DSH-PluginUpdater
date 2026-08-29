<template>
  <div class="download-page">
    <div class="bg-glow" aria-hidden="true"></div>

    <section class="page-head">
      <div class="container">
        <h1>下载中心</h1>
        <p>在线版即开即用，桌面客户端单文件运行无需安装</p>
      </div>
    </section>

    <section class="download-body">
      <div class="container">
        <!-- 在线版优先推荐 -->
        <div class="online-banner card">
          <div class="online-info">
            <div class="online-icon">🌐</div>
            <div>
              <h3>在线版 · 无需下载</h3>
              <p>打开浏览器即可管理插件，功能与客户端一致，自动保持最新版本</p>
            </div>
          </div>
          <a
            href="http://64.90.30.139:8071/"
            target="_blank"
            rel="noopener"
            class="btn btn-primary"
          >
            立即打开在线版
          </a>
        </div>

        <!-- 桌面客户端 -->
        <div class="section-title" style="margin-top: 56px">
          <h2>桌面客户端</h2>
          <p>选择适合您平台的版本</p>
        </div>

        <div class="download-grid">
          <div class="download-card card">
            <div class="download-icon win">🪟</div>
            <h3>Windows 版</h3>
            <p class="version">v1.0.0 · 64位 · EXE 安装包</p>
            <ul class="download-info">
              <li>支持 Windows 10/11</li>
              <li>单文件安装，自动创建快捷方式</li>
              <li>大小约 15MB</li>
            </ul>
            <a href="/dsh-plugin-updater.exe" download class="btn btn-primary download-btn">
              下载 .exe
            </a>
            <p class="download-hash mono">绿色免安装 · 单文件 · 无需管理员权限</p>
          </div>

          <div class="download-card card">
            <div class="download-icon linux">🐧</div>
            <h3>Linux 版</h3>
            <p class="version">v1.0.0 · AppImage · x86_64</p>
            <ul class="download-info">
              <li>支持 Ubuntu 20.04+ / Debian 11+</li>
              <li>AppImage 格式，开箱即用</li>
              <li>大小约 20MB</li>
            </ul>
            <a href="/downloads/DSH-PluginUpdater_1.0.0_amd64.AppImage" class="btn btn-primary download-btn">
              下载 .AppImage
            </a>
            <p class="download-hash mono">SHA256: f6e5d4c3b2a1…</p>
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
                <li>或手动输入插件目录路径后扫描</li>
                <li>检查更新并按需升级插件</li>
                <li>所有更新前自动备份，可回滚</li>
              </ol>
            </div>
          </div>
        </div>

        <!-- 系统要求 -->
        <div class="sys-req card">
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
              <span>需访问 GitHub（内置代理，无需科学上网）</span>
            </div>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
useHead({
  title: '下载中心 - DSH 插件升级管理',
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

/* ---------- 在线版横幅 ---------- */
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
