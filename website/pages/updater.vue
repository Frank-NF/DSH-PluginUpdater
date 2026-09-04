<template>
  <div class="updater-page">
    <div class="container">
      <div class="hero">
        <h1>DSH 插件升级管理工具</h1>
        <p class="subtitle">专业插件管理 · 一键更新 · 智能扫描</p>
        <div class="actions">
          <NuxtLink to="/download" class="btn btn-primary">
            ↓ 下载客户端
          </NuxtLink>
          <a :href="previewUrl" target="_blank" rel="noopener" class="btn btn-secondary">
            → 在线预览
          </a>
        </div>
      </div>

      <div class="features">
        <div class="feature-card">
          <div class="feature-icon">🔍</div>
          <h3>自动扫描</h3>
          <p>智能扫描 DSH 插件目录，快速发现已安装插件</p>
        </div>
        <div class="feature-card">
          <div class="feature-icon">⬆️</div>
          <h3>一键更新</h3>
          <p>检测最新版本，一键下载安装，支持断点续传</p>
        </div>
        <div class="feature-card">
          <div class="feature-icon">⚙️</div>
          <h3>灵活配置</h3>
          <p>支持多服务器管理，自定义更新策略和备份方案</p>
        </div>
        <div class="feature-card">
          <div class="feature-icon">🛡️</div>
          <h3>安全可靠</h3>
          <p>香港中转加速，签名验证确保安装包完整</p>
        </div>
      </div>

      <div class="stats">
        <div class="stat-item">
          <span class="stat-number">2189+</span>
          <span class="stat-label">插件数量</span>
        </div>
        <div class="stat-item">
          <span class="stat-number">100+</span>
          <span class="stat-label">开发者</span>
        </div>
        <div class="stat-item">
          <span class="stat-number">5000+</span>
          <span class="stat-label">安装次数</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const runtimeConfig = useRuntimeConfig()
const previewUrl = runtimeConfig.public.previewUrl || 'https://64.90.30.139:8071/'
</script>

<style scoped>
.updater-page {
  min-height: 100vh;
  background: #0f172a;
  padding: 80px 0;
}

.hero {
  text-align: center;
  margin-bottom: 60px;
}

.hero h1 {
  font-size: 48px;
  font-weight: 700;
  margin-bottom: 16px;
  background: linear-gradient(135deg, #6366f1, #818cf8);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.subtitle {
  font-size: 20px;
  color: #cbd5e1;
  margin-bottom: 32px;
}

.actions {
  display: flex;
  gap: 16px;
  justify-content: center;
}

.btn {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 12px 24px;
  border-radius: 12px;
  font-size: 16px;
  font-weight: 500;
  text-decoration: none;
  transition: all 0.24s cubic-bezier(0.4, 0, 0.2, 1);
  cursor: pointer;
  border: none;
}

.btn-primary {
  background: #6366f1;
  color: white;
}

.btn-primary:hover {
  background: #4f46e5;
  transform: translateY(-2px);
}

.btn-secondary {
  background: #334155;
  color: #f8fafc;
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.btn-secondary:hover {
  background: #1e293b;
  border-color: #818cf8;
}

.features {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
  gap: 24px;
  margin-bottom: 60px;
}

.feature-card {
  background: rgba(30, 41, 59, 0.7);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 16px;
  padding: 32px;
  text-align: center;
  transition: all 0.24s cubic-bezier(0.4, 0, 0.2, 1);
}

.feature-card:hover {
  transform: translateY(-4px);
  border-color: rgba(129, 140, 248, 0.5);
  box-shadow: 0 8px 32px rgba(99, 102, 241, 0.2);
}

.feature-icon {
  font-size: 32px;
  margin-bottom: 16px;
}

.feature-card h3 {
  font-size: 20px;
  margin: 16px 0 8px;
  color: #f8fafc;
}

.feature-card p {
  color: #94a3b8;
  font-size: 14px;
  line-height: 1.6;
}

.stats {
  display: flex;
  justify-content: center;
  gap: 64px;
  padding: 40px 0;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
}

.stat-item {
  text-align: center;
}

.stat-number {
  display: block;
  font-size: 36px;
  font-weight: 700;
  color: #818cf8;
}

.stat-label {
  font-size: 14px;
  color: #94a3b8;
  margin-top: 4px;
  display: block;
}
</style>
