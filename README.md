# DSH 插件升级管理工具

> 独立运行的 DSH 插件升级管理工具，不依赖 Agent 本体。支持扫描、更新、启用、禁用、卸载插件，内置官方插件市场与自动更新检测。

**简体中文 · [English](README.en.md)**

## 项目特性

- **独立运行**：不依赖 DSH Agent 本体进程，纯桌面工具
- **智能扫描**：自动识别插件目录下所有已安装插件和 Agent 本体
- **插件市场**：内置官方插件目录（2189+ 款插件），支持分类筛选、关键词搜索、Star/下载量/最新排序、一键安装
- **一键更新**：通过 npm registry 检测最新版本，快速下载更新
- **启停管理**：轻松启用/禁用插件，无需删除文件
- **安全卸载**：卸载前自动备份，支持随时回滚
- **目录直达**：一键打开插件所在文件夹
- **修复中心**：DSH 运行环境体检 + 常见报错双语修复指南
- **双语界面**：中文/英文一键切换，本地记忆选择
- **跨平台**：支持 Windows 和 Linux，单文件运行

## 项目结构

```
DSH-PluginUpdater/
├── src-tauri/              # Tauri Rust 后端
│   ├── src/
│   │   ├── main.rs         # 应用入口，Tauri 命令注册
│   │   ├── error.rs        # 错误类型和数据结构定义
│   │   ├── manifest.rs     # 插件清单读写
│   │   ├── plugin_scan.rs  # 插件目录扫描
│   │   ├── github_proxy.rs # GitHub 请求客户端
│   │   └── file_ops.rs     # 文件操作（更新/卸载/备份）
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── build.rs
├── src-vue/                # Vue3 前端界面
│   ├── src/
│   │   ├── components/     # UI 组件
│   │   ├── stores/         # Pinia 状态管理
│   │   ├── api/            # Tauri 调用封装
│   │   ├── types/          # TypeScript 类型定义
│   │   ├── styles/         # 全局样式
│   │   ├── App.vue
│   │   └── main.ts
│   ├── package.json
│   ├── vite.config.ts
│   └── index.html
├── website/                # Nuxt3 官方网站
│   ├── pages/              # 页面（首页/插件市场/下载/文档）
│   ├── components/         # 网站组件
│   ├── assets/css/         # 网站样式
│   ├── nuxt.config.ts
│   └── package.json
├── docs/                   # 项目文档
└── README.md
```

## 技术栈

### 桌面客户端
- **Tauri 2.0**：桌面应用框架，Rust 后端 + Web 前端
- **Vue 3**：前端框架，Composition API
- **TypeScript**：类型安全
- **Element Plus**：UI 组件库
- **Pinia**：状态管理
- **Rust**：后端核心逻辑
  - `reqwest`：HTTP 客户端
  - `semver`：语义化版本解析
  - `zip`：压缩包解压
  - `serde`：序列化/反序列化
  - `walkdir`：目录遍历

### 官方网站
- **Nuxt 3**：SSR 框架
- **Vue 3**：前端框架
- **Element Plus**：UI 组件库

## 快速开始

### 1. 克隆项目

```bash
git clone https://github.com/Frank-NF/DSH-PluginUpdater.git
cd DSH-PluginUpdater
```

### 2. 开发桌面客户端

```bash
# 安装前端依赖
cd src-vue
npm install

# 安装 Rust 依赖（自动）
cd ../src-tauri
cargo build

# 启动开发模式
cd ..
npm run tauri dev
```

### 3. 构建生产版本

```bash
cd src-tauri
cargo tauri build
```

构建产物位于 `src-tauri/target/release/bundle/` 目录下。

### 4. 启动官方网站

```bash
cd website
npm install
npm run dev
```

## 插件清单规范

每个插件目录下需要包含 `plugin.manifest.json` 文件：

```json
{
  "id": "dsh-plugin-example",
  "name": "示例插件",
  "description": "插件功能介绍",
  "github_repo": "owner/repo",
  "current_version": "1.0.0",
  "enabled": true,
  "type": "plugin",
  "author": "作者名称",
  "homepage": "https://example.com"
}
```

### 字段说明

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| id | string | 是 | 插件唯一标识符 |
| name | string | 是 | 插件显示名称 |
| description | string | 否 | 功能介绍 |
| github_repo | string | 否 | GitHub 仓库 (owner/repo) |
| current_version | string | 否 | 当前版本号 |
| enabled | boolean | 否 | 是否启用，默认 true |
| type | string | 否 | plugin 或 agent-core |
| author | string | 否 | 作者 |
| homepage | string | 否 | 主页地址 |

## 配置说明

在工具「设置」中可配置：

- **代理地址**：默认本地直连，留空合法；填写代理后所有 GitHub 请求经代理中转
- **安装源（npm Registry）**：官方源或国内镜像，自定义地址须以 http(s):// 开头
- **默认插件目录**：启动时自动扫描的目录
- **扫描后自动检查更新**：默认开启
- **更新前自动备份**：默认开启

## 常见问题

### Q: 工具提示"检查更新失败"？
A: 检查网络连接。更新检测走 npm registry，无需额外配置。

### Q: 更新时提示"文件被占用"？
A: 请先关闭 DSH Agent 本体，再执行更新操作。

### Q: 如何恢复误删的插件？
A: 工具在卸载和更新前都会自动备份，可在备份管理中恢复。

## 许可证

MIT License

## 联系方式

- 官网：https://dsh.huilinsh.cn
- GitHub：https://github.com/Frank-NF/DSH-PluginUpdater
