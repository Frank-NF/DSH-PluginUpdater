export default defineNuxtConfig({
  devtools: { enabled: true },

  app: {
    head: {
      title: 'DSH 插件升级管理工具 - 官方网站',
      htmlAttrs: {
        lang: 'zh-CN',
      },
      meta: [
        { charset: 'utf-8' },
        { name: 'viewport', content: 'width=device-width, initial-scale=1' },
        {
          name: 'description',
          content:
            'DSH 插件升级管理工具：2189+ 插件市场、行业组合包一键安装、MCP 服务管理、快照与离线部署、本地 DSH Web 服务器面板。全链路 Ed25519 签名验证，密钥三次轮换，防篡改安全体系。',
        },
        {
          name: 'keywords',
          content:
            'DSH,插件管理,插件市场,组合包,MCP管理,离线部署,Ed25519签名,自动更新,快照',
        },
        { property: 'og:title', content: 'DSH 插件升级管理工具 - 官方网站' },
        {
          property: 'og:description',
          content: '2189+ 插件市场 · 行业组合包 · MCP 管理 · 离线部署 · 全链路签名验证',
        },
        { property: 'og:type', content: 'website' },
      ],
      link: [
        { rel: 'icon', type: 'image/svg+xml', href: '/favicon.svg' },
      ],
    },
  },

  css: [
    '~/assets/css/main.css',
    'element-plus/dist/index.css',
  ],

  // 旧路由整理：/updater 为 1.0 时代遗留营销页（假统计），重定向到首页
  routeRules: {
    '/updater': { redirect: '/' },
  },

  runtimeConfig: {
    githubClientSecret: process.env.GITHUB_CLIENT_SECRET || '',
    // 超级管理员邮箱白名单（逗号分隔），GitHub 登录时匹配则自动提升为 admin
    superAdminEmails: process.env.SUPER_ADMIN_EMAILS || '',
    public: {
      proxyBaseUrl: process.env.PROXY_BASE_URL || '',
      appVersion: '1.14.0',
      githubClientId: process.env.GITHUB_CLIENT_ID || '',
      // 在线版已于 v1.13.15 下线；保留空值占位避免旧引用报错（页面上已全部移除入口）
      previewUrl: '',
    },
  },

  nitro: {
    preset: 'node-server',
  },
})
