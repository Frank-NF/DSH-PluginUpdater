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
          content: 'DSH 独立插件升级管理工具，支持扫描、更新、启用、禁用、卸载插件，通过香港中转代理访问 GitHub，访问速度快，界面友好。',
        },
        { name: 'keywords', content: 'DSH,插件管理,升级工具,GitHub代理,香港中转' },
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

  runtimeConfig: {
    public: {
      proxyBaseUrl: process.env.PROXY_BASE_URL || '',
      appVersion: '1.0.0',
    },
  },

  nitro: {
    preset: 'node-server',
  },
})
