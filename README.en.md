# DSH Plugin Updater

> A standalone DSH plugin management tool that does not depend on the Agent core. Scan, update, enable, disable and uninstall plugins, with a built-in official plugin marketplace and automatic update detection.

**[简体中文](README.md) · English**

## Features

- **Standalone** — pure desktop tool, does not depend on the DSH Agent process
- **Smart Scan** — auto-detects installed plugins and the Agent core in plugin directories
- **Plugin Marketplace** — built-in official catalog (2189+ plugins) with category filters, keyword search, Star/downloads/latest sorting and one-click install
- **One-Click Updates** — checks the latest versions via npm registry, fast downloads
- **Enable/Disable** — toggle plugins on/off without deleting files
- **Safe Uninstall** — automatic backup before uninstall, restore anytime
- **Folder Access** — one click to open the plugin folder
- **Repair Center** — DSH environment check + bilingual repair guide for common errors
- **Bilingual UI** — Chinese/English switch, remembers your choice
- **Cross-Platform** — Windows and Linux, single-file runtime

## Project Structure

```
DSH-PluginUpdater/
├── src-tauri/              # Tauri Rust backend
│   ├── src/
│   │   ├── main.rs         # App entry, Tauri command registration
│   │   ├── error.rs        # Error types and data structures
│   │   ├── manifest.rs     # Plugin manifest read/write
│   │   ├── plugin_scan.rs  # Plugin directory scanning
│   │   ├── github_proxy.rs # GitHub request client
│   │   └── file_ops.rs     # File operations (update/uninstall/backup)
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── build.rs
├── src-vue/                # Vue3 frontend
│   ├── src/
│   │   ├── components/     # UI components
│   │   ├── stores/         # Pinia state management
│   │   ├── api/            # Tauri invocation wrappers
│   │   ├── types/          # TypeScript type definitions
│   │   ├── styles/         # Global styles
│   │   ├── App.vue
│   │   └── main.ts
│   ├── package.json
│   ├── vite.config.ts
│   └── index.html
├── website/                # Nuxt3 official website
│   ├── pages/              # Pages (home/marketplace/download/docs)
│   ├── components/         # Website components
│   ├── assets/css/         # Website styles
│   ├── nuxt.config.ts
│   └── package.json
├── docs/                   # Project documentation
└── README.md
```

## Tech Stack

### Desktop Client
- **Tauri 2.0** — desktop app framework, Rust backend + Web frontend
- **Vue 3** — frontend framework, Composition API
- **TypeScript** — type safety
- **Element Plus** — UI component library
- **Pinia** — state management
- **Rust** — backend core logic
  - `reqwest`: HTTP client
  - `semver`: semantic version parsing
  - `zip`: archive extraction
  - `serde`: serialization/deserialization
  - `walkdir`: directory traversal

### Official Website
- **Nuxt 3** — SSR framework
- **Vue 3** — frontend framework
- **Element Plus** — UI component library

## Quick Start

### 1. Clone

```bash
git clone https://github.com/Frank-NF/DSH-PluginUpdater.git
cd DSH-PluginUpdater
```

### 2. Develop the Desktop Client

```bash
# Install frontend dependencies
cd src-vue
npm install

# Install Rust dependencies (automatic)
cd ../src-tauri
cargo build

# Start development mode
cd ..
npm run tauri dev
```

### 3. Build Production

```bash
cd src-tauri
cargo tauri build
```

Build artifacts are in `src-tauri/target/release/bundle/`.

### 4. Run the Official Website

```bash
cd website
npm install
npm run dev
```

## Plugin Manifest Specification

Each plugin directory must contain a `plugin.manifest.json` file:

```json
{
  "id": "dsh-plugin-example",
  "name": "Example Plugin",
  "description": "Plugin description",
  "github_repo": "owner/repo",
  "current_version": "1.0.0",
  "enabled": true,
  "type": "plugin",
  "author": "Author Name",
  "homepage": "https://example.com"
}
```

### Field Reference

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| id | string | Yes | Unique plugin identifier |
| name | string | Yes | Display name |
| description | string | No | Description |
| github_repo | string | No | GitHub repository (owner/repo) |
| current_version | string | No | Current version |
| enabled | boolean | No | Enabled by default true |
| type | string | No | plugin or agent-core |
| author | string | No | Author |
| homepage | string | No | Homepage URL |

## Configuration

Configurable in the app "Settings":

- **Proxy URL** — direct connection by default, empty is valid; when set, all GitHub requests go through the proxy
- **Install Registry (npm)** — official or mirror registry; custom URLs must start with http(s)://
- **Default Plugin Directory** — directory scanned at startup
- **Auto-check Updates After Scan** — on by default
- **Backup Before Update** — on by default

## FAQ

### Q: "Update check failed"?
A: Check your network connection. Update checks use the npm registry and need no extra configuration.

### Q: "File in use" when updating?
A: Close the DSH Agent core first, then retry the update.

### Q: How to restore accidentally deleted plugins?
A: The tool auto-backs up before uninstall and update; restore from the backup manager.

## License

MIT License

## Contact

- Website: https://dsh.huilinsh.cn
- GitHub: https://github.com/Frank-NF/DSH-PluginUpdater
