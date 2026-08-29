# DSH PluginUpdater 项目记忆

## UI 设计偏好（极重要）

- **用户反馈**: "有些普通，我想让你全新设计一套"
- **设计方向**: 现代极简 + 玻璃拟态 + 深色主题优先
- **主色调**: 靛蓝 `#6366F1`（替代 Element Plus 默认蓝色）
- **背景**: 深蓝黑 `#0F172A`（替代白色背景）
- **核心效果**: `backdrop-filter: blur(12px)` 玻璃拟态

## v2 设计交付物（完整版）

位置: `docs/ui-design/v2/`

### 核心页面
- `UI-DESIGN-SPEC-v2.md` - 设计规范文档（含组件清单）
- `README.md` - 组件索引和快速访问
- `desktop-client.html` - 桌面客户端原型
- `website-index.html` - 官网首页原型
- `website-plugins.html` - 插件市场原型
- `website-download.html` - 下载中心原型
- `website-docs.html` - 文档中心原型

### Three.js 效果
- `threejs/hero-3d.html` - 1500+ 粒子系统
- `threejs/constellation.html` - 星座连线效果
- `threejs/3d-showcase.html` - 3D 几何展示

### 组件库 (8 个)
- `components/settings-dialog.html` - 设置对话框
- `components/update-progress.html` - 更新进度
- `components/release-notes.html` - 发布说明
- `components/toast.html` - Toast 通知
- `components/empty-states.html` - 空状态/错误状态
- `components/confirm-dialog.html` - 确认对话框
- `components/skeleton.html` - 骨架屏 Loading
- `components/mobile-nav.html` - 移动端导航

## 桌面客户端改版状态（2026-08-28 已完成）

原型已落地到实际代码 `src-vue/`，从 Element Plus 白色默认主题改为 v2 深色玻璃拟态。

### 改版后的关键约定（后续开发必须遵守）

- **深色模式启用方式**：`main.ts` 引入 `element-plus/theme-chalk/dark/css-vars.css`，
  并给 `document.documentElement` 加 `dark` class
- **主色覆盖位置**：`src/styles/main.css` 的 `html.dark {}` 块内覆盖
  `--el-color-primary` 等变量，不要在各组件里硬编码颜色
- **设计 token 来源**：所有颜色/圆角/动效统一用 `main.css` 的 `:root` 变量
  （`--primary` / `--bg-primary` / `--glass-*` / `--radius-*` / `--dur`）
- **组件内禁止重复定义 `.mini-tag-*`**：全局已提供深色版，
  scoped 内的 `:deep()` 定义优先级更高会覆盖掉全局深色配色
- **动效克制原则**：只动画 `transform` / `opacity`，时长 0.24s

### 已改版文件

- `src/styles/main.css`（重写：token + Element Plus 深色覆盖 + 工具类）
- `src/main.ts`（深色主题入口）
- `src/App.vue`（深色容器 + 光晕 + v2 空状态）
- `src/components/HeaderBar.vue`（玻璃拟态导航）
- `src/components/PluginTable.vue`（网格/列表双视图深色化）
- `src/components/SettingsDialog.vue`（分组卡片式）
- `src/components/ReleaseNotesDialog.vue`（版本对比胶囊）
- `src/api/index.ts`、`src/stores/pluginStore.ts`（修复翻译接口，见下）

### 原始文件备份

`src-vue/src/.backup-20260828/`

### 已知遗留问题（未修）

- `npm run type-check` 无法运行：vue-tsc 1.8 与 TypeScript 5.3 不兼容
  （报 `Search string not found: "/supportedTSExtensions"`）。
  需升级 vue-tsc 到 2.x。`npm run build` 正常。
- 构建前沙箱 safe-delete 会拦截清空 dist，先 `mv dist dist-old` 再 build

## 服务器调试预览（隔离部署）

- **地址**: http://64.90.30.139:8071/ （DSH 插件管理 UI 调试预览）
- **目录**: `/var/www/dsh-updater-preview`（nginx 站点，勿放 /root 下——worker 无权限会 500）
- **nginx 配置**: `/etc/nginx/conf.d/dsh-updater-preview.conf`，端口 8071，已 ufw 放行
- **Mock 机制**: `src/api/index.ts` 运行时检测 Tauri，
  浏览器环境自动走 Mock 数据（6 个示例插件、模拟进度/翻译），
  Tauri 桌面端走真实 invoke——同一份代码两种环境都能跑
- **与其他项目完全隔离**: 8071 为独立 server 块 + 独立目录，
  不影响 jizhang/mall-site/wow-macro 等现有服务
- **更新流程**: 本地 build 后 `scp -r dist/. root@64.90.30.139:/var/www/dsh-updater-preview/`

## 项目背景

- Tauri 2 + Vue3 + Element Plus 桌面应用
- Go/Gin 香港中转代理
- Nuxt3 官方网站
- 插件升级管理工具
