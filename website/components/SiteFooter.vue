<template>
  <footer class="site-footer">
    <div class="container">
      <div class="footer-content">
        <div class="footer-brand">
          <div class="brand-row">
            <div class="logo-mark">DSH</div>
            <h3>DSH 插件升级管理</h3>
          </div>
          <p>
            独立运行的插件升级管理工具，支持扫描、更新、启用、禁用、卸载插件。
            提供在线版与桌面客户端两种使用方式，通过网络代理访问 GitHub。
          </p>
        </div>
        <div class="footer-column">
          <h4>产品</h4>
          <ul>
            <li><NuxtLink to="/">首页</NuxtLink></li>
            <li><NuxtLink to="/plugins">插件市场</NuxtLink></li>
            <li><NuxtLink to="/download">下载客户端</NuxtLink></li>
            <li>
              <a href="http://64.90.30.139:8071/" target="_blank" rel="noopener">在线版</a>
            </li>
          </ul>
        </div>
        <div class="footer-column">
          <h4>资源</h4>
          <ul>
            <li><NuxtLink to="/docs">使用文档</NuxtLink></li>
            <li><a href="https://github.com/Frank-NF" target="_blank" rel="noopener">GitHub</a></li>
            <li>
              <a href="https://github.com/Frank-NF/DSH-PluginUpdater/issues" target="_blank" rel="noopener">问题反馈</a>
            </li>
          </ul>
        </div>
        <div class="footer-column">
          <h4>联系</h4>
          <ul>
            <li><a href="https://dsh.huilinsh.cn">dsh.huilinsh.cn</a></li>
            <li>CDN 节点</li>
            <li>7x24 小时在线</li>
          </ul>
        </div>
      </div>
      <div class="footer-bottom">
        <p>&copy; 2026 DSH Plugin Updater. All rights reserved. | 官方加速</p>
      </div>
    </div>
  </footer>
</template>

<script setup lang="ts">
</script>

<style scoped>
.site-footer {
  margin-top: 80px;
  border-top: 1px solid var(--glass-border);
  background: rgba(15, 23, 42, 0.85);
}

.footer-content {
  display: grid;
  grid-template-columns: 2fr 1fr 1fr 1fr;
  gap: 40px;
  padding: 56px 0 40px;
}

.brand-row {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 14px;
}

.logo-mark {
  width: 34px;
  height: 34px;
  border-radius: 10px;
  background: linear-gradient(135deg, var(--primary), var(--primary-light));
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 700;
  font-size: 11px;
  color: #fff;
}

.footer-brand h3 {
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary);
}

.footer-brand p {
  font-size: 13px;
  color: var(--text-muted);
  line-height: 1.7;
  max-width: 360px;
}

.footer-column h4 {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-secondary);
  margin-bottom: 14px;
}

.footer-column ul {
  list-style: none;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.footer-column a {
  font-size: 13px;
  color: var(--text-muted);
  transition: color var(--dur) var(--ease);
}

.footer-column a:hover {
  color: var(--primary-light);
}

.footer-column li {
  font-size: 13px;
  color: var(--text-muted);
}

.footer-bottom {
  border-top: 1px solid rgba(255, 255, 255, 0.06);
  padding: 20px 0;
  text-align: center;
}

.footer-bottom p {
  font-size: 12px;
  color: var(--text-muted);
}

@media (max-width: 900px) {
  .footer-content {
    grid-template-columns: 1fr 1fr;
    gap: 32px;
  }
}
</style>
