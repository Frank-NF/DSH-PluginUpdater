# DSH 插件升级管理工具

> 独立运行的 DSH 插件升级管理工具，不依赖 Agent 本体。支持扫描、更新、启用、禁用、卸载插件，通过香港中转代理访问 GitHub。

## 项目特性

- **独立运行**：不依赖 DSH Agent 本体进程，纯桌面工具
- **智能扫描**：自动识别插件目录下所有已安装插件和 Agent 本体
- **一键更新**：通过香港中转代理快速检测和下载 GitHub 最新版本
- **启停管理**：轻松启用/禁用插件，无需删除文件
- **安全卸载**：卸载前自动备份，支持随时回滚
- **目录直达**：一键打开插件所在文件夹
- **香港加速**：所有 GitHub 请求通过香港服务器中转，解决访问限制
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
│   │   ├── github_proxy.rs # GitHub 代理客户端
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
├── proxy-server/           # Go 香港中转代理服务
│   ├── main.go
│   ├── go.mod
│   ├── Dockerfile
│   ├── docker-compose.yml
│   └── .env.example
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

### 香港中转代理服务
- **Go 1.21**：高性能后端语言
- **Gin**：Web 框架
- **Redis**：API 响应缓存
- **Docker**：容器化部署

### 官方网站
- **Nuxt 3**：SSR 框架
- **Vue 3**：前端框架
- **Element Plus**：UI 组件库

## 快速开始

### 1. 克隆项目

```bash
git clone https://github.com/DSH-Team/DSH-PluginUpdater.git
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

### 4. 部署香港中转代理

```bash
cd proxy-server

# 复制配置
cp .env.example .env
# 编辑 .env 配置 Token 等参数

# Docker 部署
docker-compose up -d
```

### 5. 启动官方网站

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

## 代理服务 API

### 获取最新 Release

```
GET /api/github/latest?repo=owner/repo
```

### 获取 Release 列表

```
GET /api/github/releases?repo=owner/repo&per_page=10
```

### 下载 Release 资产

```
GET /api/github/download?repo=owner/repo&tag=v1.0.0&asset=plugin.zip
```

### 获取插件列表

```
GET /api/plugins/list
```

## 配置说明

### 客户端配置

在工具「设置」中可配置：

- **香港中转代理地址**：默认 `https://proxy.dsh-update.hk`
- **默认插件目录**：启动时自动扫描的目录
- **扫描后自动检查更新**：默认开启
- **更新前自动备份**：默认开启

### 代理服务环境变量

参见 `proxy-server/.env.example`。

## 常见问题

### Q: 工具提示"检查更新失败"？
A: 检查网络连接和代理地址配置，确保香港中转代理服务可访问。

### Q: 更新时提示"文件被占用"？
A: 请先关闭 DSH Agent 本体，再执行更新操作。

### Q: 如何恢复误删的插件？
A: 工具在卸载和更新前都会自动备份，可在备份管理中恢复。

## 许可证

MIT License

## 联系方式

- 官网：https://dsh-update.hk
- 邮箱：support@dsh-update.hk
- GitHub：https://github.com/DSH-Team
