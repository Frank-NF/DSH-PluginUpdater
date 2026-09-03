/** DSH 插件/环境常见报错修复指南（来源：dsh-handbook FAQ、DSH Plugin Hub 排错指南、官方 Discussions） */
export interface RepairGuideItem {
  id: string
  keywords: string[]
  title: { zh: string; en: string }
  cause: { zh: string; en: string }
  steps: { zh: string[]; en: string[] }
}

export const REPAIR_GUIDES: RepairGuideItem[] = [
  {
    id: 'install-404',
    keywords: ['404', '安装', 'install', 'not found', '无法安装'],
    title: { zh: '插件安装报 404 / 找不到包', en: 'Plugin install 404 / package not found' },
    cause: {
      zh: '预览版（rc 线）之间插件依赖声明可能断裂；或插件包未发布到 npm。',
      en: 'Dependency declarations can break between preview (rc) releases, or the plugin package was never published to npm.',
    },
    steps: {
      zh: ['确认 DSH 本体与插件使用同一 rc 版本线（如 ^0.1.0-rc.6）', '更新 DSH 本体到最新版后重试', '确认插件包名拼写（npm view <包名> versions 可查）'],
      en: ['Verify DSH core and the plugin use the same rc line (e.g. ^0.1.0-rc.6)', 'Update DSH core to the latest version and retry', 'Verify the package name (npm view <pkg> versions)'],
    },
  },
  {
    id: 'github-no-dist',
    keywords: ['github', 'dist', '构建', 'build', '入口', '加载失败', 'crash'],
    title: { zh: 'GitHub 源插件装完不加载（缺构建产物）', en: 'GitHub-sourced plugin fails to load (missing dist)' },
    cause: {
      zh: 'git 分发的插件包可能没提交构建产物（dist/），装完入口文件缺失导致加载失败甚至后端崩溃。',
      en: 'Git-distributed plugins may not commit build output (dist/), so the entry file is missing after install, causing load failure or backend crash.',
    },
    steps: {
      zh: ['在插件详情页确认安装来源（npm 还是 github）', 'GitHub 源优先改装 npm 分发版', '或反馈插件作者补发构建产物/版本 tag'],
      en: ['Check install source on the plugin page (npm vs github)', 'Prefer the npm-distributed version for GitHub-sourced plugins', 'Or ask the author to publish build output / version tag'],
    },
  },
  {
    id: 'npx-timeout',
    keywords: ['npx', '超时', 'timeout', 'ETIMEDOUT', 'ECONNRESET', '慢', 'slow'],
    title: { zh: 'npx 启动慢 / 网络超时', en: 'npx startup slow / network timeout' },
    cause: {
      zh: 'npx 每次启动从 npm 官方源拉包，官方源慢或被墙导致超时（ETIMEDOUT / ECONNRESET）。',
      en: 'npx pulls packages from the npm registry on every start; the official registry can be slow or blocked, causing timeouts.',
    },
    steps: {
      zh: ['npm config get registry 查看当前源', '换国内镜像：npm config set registry https://registry.npmmirror.com', '或全局安装一次免 npx：npm i -g @deepseek-ai/dsh'],
      en: ['npm config get registry to check the current registry', 'Switch to a mirror: npm config set registry https://registry.npmmirror.com', 'Or install globally once: npm i -g @deepseek-ai/dsh'],
    },
  },
  {
    id: 'npm-cache',
    keywords: ['EINTEGRITY', 'EACCES', '缓存', 'cache', '权限'],
    title: { zh: 'npm 缓存损坏 / 权限错误', en: 'npm cache corruption / permission error' },
    cause: {
      zh: 'npm 缓存损坏（EINTEGRITY）或全局目录权限不足（EACCES）。',
      en: 'Corrupted npm cache (EINTEGRITY) or insufficient permission on the global prefix (EACCES).',
    },
    steps: {
      zh: ['npm cache clean --force 清缓存后重试', '若反复 EACCES：npm config get prefix 查看安装位置，修复目录权限'],
      en: ['Run npm cache clean --force and retry', 'If EACCES persists: npm config get prefix and fix directory permissions'],
    },
  },
  {
    id: 'node-version',
    keywords: ['node', '版本', 'version', '语法', 'syntax'],
    title: { zh: 'Node 版本过低', en: 'Node version too old' },
    cause: {
      zh: '过旧的 Node 在解析依赖时容易报语法类错误，这是预览版项目最常见的隐藏坑。',
      en: 'An outdated Node often fails with syntax errors when resolving dependencies — a common hidden pitfall in preview projects.',
    },
    steps: {
      zh: ['node -v 查看版本', '升级到 Node ≥ 18（推荐 LTS）后重试'],
      en: ['Check with node -v', 'Upgrade to Node ≥ 18 (LTS recommended) and retry'],
    },
  },
  {
    id: 'port-3080',
    keywords: ['3080', '端口', 'port', 'EADDRINUSE', '被占'],
    title: { zh: '端口 3080 被占用', en: 'Port 3080 already in use' },
    cause: {
      zh: '另一个进程占用了 DSH Web 默认端口 3080。',
      en: 'Another process is holding the default DSH Web port 3080.',
    },
    steps: {
      zh: ['netstat -ano | findstr 3080 找到占用进程 PID', 'taskkill /PID <PID> /F 结束进程后重启 DSH'],
      en: ['netstat -ano | findstr 3080 to find the occupying PID', 'taskkill /PID <PID> /F then restart DSH'],
    },
  },
  {
    id: 'port-reserved',
    keywords: ['EACCES', '保留端口', 'Hyper-V', 'WSL2', 'excludedportrange'],
    title: { zh: '端口 3080 报 EACCES 但无进程占用', en: 'Port 3080 EACCES with no process using it' },
    cause: {
      zh: 'Windows 上 3080 可能落在 Hyper-V / WSL2 / Docker Desktop 的保留端口区间内。',
      en: 'On Windows, port 3080 may fall inside Hyper-V / WSL2 / Docker Desktop reserved ranges.',
    },
    steps: {
      zh: ['netsh interface ipv4 show excludedportrange protocol=tcp 查看保留区间', '直接换端口：dsh web --port 13080'],
      en: ['netsh interface ipv4 show excludedportrange protocol=tcp to inspect ranges', 'Just use another port: dsh web --port 13080'],
    },
  },
  {
    id: 'hmr-expose',
    keywords: ['--expose-internals', 'HMR', 'hmr'],
    title: { zh: '启动报 --expose-internals is required for HMR service', en: 'Startup error: --expose-internals is required for HMR service' },
    cause: {
      zh: 'macOS arm64 / NixOS / 部分 Linux 上 cordis HMR loader 探测不到 Node 内部模块所致。',
      en: 'The cordis HMR loader cannot detect Node internal modules on macOS arm64 / NixOS / some Linux.',
    },
    steps: {
      zh: ['临时方案：node --expose-internals <bin> web 启动', '或等待官方修复'],
      en: ['Workaround: node --expose-internals <bin> web', 'Or wait for an official fix'],
    },
  },
  {
    id: 'unknown-tool',
    keywords: ['unknown tool', '工具调用', 'tool', '空', '流式'],
    title: { zh: '所有工具调用报 Error: unknown tool ""', en: 'All tool calls fail with Error: unknown tool ""' },
    cause: {
      zh: 'rc.6 流式解析 bug：SSE 分块覆盖赋值把工具名/ID 抹成空串。',
      en: 'rc.6 streaming bug: SSE chunk overwrites blank out the tool name/ID.',
    },
    steps: {
      zh: ['官方已定位（#725），等版本修复或自行打补丁', '及时中止模型反复重试'],
      en: ['Official root cause known (#725); wait for fix or patch manually', 'Stop the model from retrying in a loop'],
    },
  },
  {
    id: 'call-stack',
    keywords: ['Maximum call stack', 'call stack', '超长会话', '会话打不开'],
    title: { zh: '超长会话打不开：Maximum call stack size exceeded', en: 'Long session fails to open: Maximum call stack size exceeded' },
    cause: {
      zh: '超长回复（20 万+ token）的 sourceEventSeqs 数组展开成函数参数，超出 V8 参数上限。会话文件本身没坏。',
      en: 'Very long replies (>200k tokens) expand sourceEventSeqs into function arguments beyond V8 limits. The session file is not corrupted.',
    },
    steps: {
      zh: ['属于 rc 已知缺陷，等官方修复或找社区补丁'],
      en: ['Known rc defect; wait for an official fix or community patch'],
    },
  },
  {
    id: 'developer-role',
    keywords: ['unknown variant developer', 'developer', '400', '角色'],
    title: { zh: '400 unknown variant developer（网关不认 developer 角色）', en: '400 unknown variant developer (gateway rejects developer role)' },
    cause: {
      zh: '部分 OpenAI 兼容网关不识别 developer 角色消息。',
      en: 'Some OpenAI-compatible gateways do not recognize the developer role message.',
    },
    steps: {
      zh: ['在 ~/.dsh/settings.yaml 对应 provider 配 compat.supportsDeveloperRole: false'],
      en: ['In ~/.dsh/settings.yaml add compat.supportsDeveloperRole: false to the provider'],
    },
  },
  {
    id: 'plugin-update-fail',
    keywords: ['升级失败', 'update', '依赖不兼容', 'dependenc'],
    title: { zh: '插件升级失败（依赖不兼容）', en: 'Plugin update fails (dependency incompatible)' },
    cause: {
      zh: 'DSH 本体不是最新版时，预览版之间插件的依赖声明可能不兼容，update 直接失败。',
      en: 'When DSH core is not latest, plugin dependency declarations between previews may be incompatible, so update fails.',
    },
    steps: {
      zh: ['先更新 DSH 本体到最新版', '再到插件市场（设置 → 插件中心）重新安装该插件', '升级前核对插件详情页标注的兼容 DSH 版本'],
      en: ['Update DSH core to the latest version first', 'Reinstall the plugin from the plugin center', 'Check the compatible DSH version on the plugin page before upgrading'],
    },
  },
  {
    id: 'dsh-running-lock',
    keywords: ['运行', '锁定', 'lock', '更新失败', 'rename', '被锁'],
    title: { zh: '更新插件失败：DSH 运行时锁定插件目录', en: 'Plugin update fails: DSH locks the plugin directory' },
    cause: {
      zh: 'DSH 桌面端运行时持有插件目录句柄，文件重命名/写入被 Windows 锁拦截。',
      en: 'A running DSH desktop holds handles on the plugin directory, so file rename/write is blocked by Windows locks.',
    },
    steps: {
      zh: ['完全退出 DSH 桌面端（含托盘）', '再回来点击更新', '本工具已内置 DSH 运行检测防护，检测到运行时会先警告'],
      en: ['Fully quit DSH desktop (including tray)', 'Then retry the update', 'This tool already warns when DSH is detected running'],
    },
  },
  {
    id: 'reasoning-effort',
    keywords: ['reasoning effort', '推理档位', '适配器'],
    title: { zh: '报 does not support reasoning effort', en: 'Error: does not support reasoning effort' },
    cause: {
      zh: '适配器不支持当前推理档位（如 low），属适配器缺口。',
      en: 'The adapter does not support the current reasoning effort level (e.g. low) — an adapter gap.',
    },
    steps: {
      zh: ['降档前先确认 provider 支持能力表', '映射到最近可用档位（如 deepseek-official 支持 off/high/max）'],
      en: ['Check the provider capability table before lowering', 'Map to the nearest supported level (e.g. deepseek-official supports off/high/max)'],
    },
  },
]
