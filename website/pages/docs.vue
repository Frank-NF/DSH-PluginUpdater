<template>
  <div>
    <section class="docs-section">
      <div class="container docs-container">
        <aside class="docs-sidebar">
          <h3>文档目录</h3>
          <ul>
            <li><a href="#quick-start" class="active">快速开始</a></li>
            <li><a href="#installation">安装指南</a></li>
            <li><a href="#basic-usage">基本使用</a></li>
            <li><a href="#bundles">行业组合包</a></li>
            <li><a href="#mcp-panel">MCP 服务管理</a></li>
            <li><a href="#snapshots">快照与离线部署</a></li>
            <li><a href="#server-panel">DSH Web 服务器面板</a></li>
            <li><a href="#security">安全设计</a></li>
            <li><a href="#self-update">自动更新</a></li>
            <li><a href="#plugin-manifest">插件清单规范</a></li>
            <li><a href="#proxy-setup">代理服务部署</a></li>
            <li><a href="#faq">常见问题</a></li>
          </ul>
        </aside>

        <div class="docs-content">
          <article id="quick-start">
            <h1>快速开始</h1>
            <p>DSH 插件升级管理工具是一个独立运行的桌面程序，不依赖 DSH Agent 本体。它可以扫描您的插件目录，管理所有已安装插件的更新、启用、禁用和卸载，并内置行业组合包、MCP 服务管理与离线部署能力。</p>

            <h2>系统要求</h2>
            <ul>
              <li><strong>Windows:</strong> Windows 10 1903+ / Windows 11（64位）</li>
              <li><strong>Linux:</strong> Ubuntu 20.04+ / Debian 11+（x86_64）</li>
              <li><strong>网络:</strong> 可访问 GitHub 与 npm 官方源（内置直连加速）</li>
            </ul>
          </article>

          <article id="installation">
            <h1>安装指南</h1>

            <h2>Windows 安装</h2>
            <ol>
              <li>从<a href="/download">下载页面</a>获取最新的 .exe 安装包（下载页展示 SHA256，可先校验再安装）</li>
              <li>双击运行安装程序</li>
              <li>按照安装向导完成安装</li>
              <li>从桌面快捷方式或开始菜单启动程序</li>
            </ol>

            <h2>Linux 安装</h2>
            <ol>
              <li>下载 .AppImage 文件</li>
              <li>打开终端，添加执行权限：
                <pre><code>chmod +x DSH-PluginUpdater_*.AppImage</code></pre>
              </li>
              <li>运行程序：
                <pre><code>./DSH-PluginUpdater_*.AppImage</code></pre>
              </li>
            </ol>
          </article>

          <article id="basic-usage">
            <h1>基本使用</h1>

            <h2>1. 指定插件目录</h2>
            <p>在顶部输入框中输入您的 DSH 插件目录路径，例如：</p>
            <pre><code>C:\DSH\plugins
/home/user/dsh/plugins</code></pre>

            <h2>2. 扫描插件</h2>
            <p>点击「扫描」按钮，工具将遍历目录下的所有子文件夹，读取每个插件的 <code>plugin.manifest.json</code> 文件，列出所有已安装插件。</p>

            <h2>3. 检查更新</h2>
            <p>点击「检查更新」按钮，工具将通过 npm / GitHub 双源检测每个插件的最新版本。有可用更新的插件将高亮显示。</p>

            <h2>4. 更新插件</h2>
            <p>对于有可用更新的插件，点击「更新」按钮即可开始更新。更新前会自动备份旧版本，更新过程中显示进度条。</p>
            <div class="warning-box">
              <strong>注意：</strong>更新插件前建议关闭 DSH Agent 本体，避免文件占用导致更新失败。
            </div>

            <h2>5. 启用/禁用插件</h2>
            <p>点击「启用」或「禁用」按钮可以切换插件状态。禁用不会删除插件文件，只是修改配置，重启 Agent 后生效。</p>

            <h2>6. 卸载插件</h2>
            <p>在「更多」菜单中选择「卸载」可以删除插件。卸载前会自动备份，可在备份管理中恢复。</p>

            <h2>7. 打开插件目录</h2>
            <p>点击「目录」按钮可以直接在文件管理器中打开插件所在文件夹，方便手动查看或修改插件文件。</p>
          </article>

          <article id="bundles">
            <h1>行业组合包</h1>
            <p>组合包按行业场景把「插件 + MCP 服务模板 + Skill」打包成一套，一键安装即得到完整能力栈，无需逐个挑选。入口：客户端「组合包」标签页，或官网 <a href="/bundles">组合包页</a> 浏览（安装动作在客户端内完成）。</p>

            <h2>安装流程</h2>
            <ol>
              <li>在组合包列表按行业标签筛选，查看每个包的插件清单与 MCP 服务清单</li>
              <li>点击「安装」后自动执行冲突预检（依赖冲突 / MCP 端口占用等）</li>
              <li>预检通过后逐项安装，任一项失败自动回滚已装部分</li>
              <li>安装完成后 MCP 模板进入「MCP 服务管理」面板，密钥留待填写</li>
            </ol>

            <h2>常见组合包内容</h2>
            <ul>
              <li><strong>插件清单：</strong>必装项与可选项分级，可按需跳过可选项</li>
              <li><strong>MCP 服务模板：</strong>预置服务名、命令、参数与环境变量键名（不含密值）</li>
              <li><strong>Skill 集：</strong>行业相关的技能描述文件</li>
            </ul>
          </article>

          <article id="mcp-panel">
            <h1>MCP 服务管理</h1>
            <p>MCP 面板用于集中管理本地 MCP 配置（<code>~/.dsh/dsh-mcp.json</code>）。入口：<strong>设置 → 底部「MCP 服务管理」</strong>（仅桌面客户端可用）。</p>

            <h2>1. 生成配置</h2>
            <p>首次使用时配置文件尚不存在。到客户端「组合包」标签页安装含 MCP 模板的组合包，会自动生成配置文件；你手工添加的条目始终保留。</p>

            <h2>2. 填写环境变量</h2>
            <p>每个服务卡片列出所需的环境变量（只显示键名，不显示值）。填入 Token/密钥后点「保存」，密值立即加密存入系统凭据库（Windows 凭据管理器），配置文件中不留明文。</p>

            <h2>3. 连通性探活</h2>
            <p>点击「探活」检测服务是否可用：stdio 服务会试拉起进程；streamable-http 服务会发起探测请求。返回原因与耗时；HTTP 401/403 表示服务可达但需要配置鉴权。</p>

            <h2>4. 写入配置</h2>
            <p>点击「写入配置」把已保存的密值写入运行配置（这一步之后 DSH 运行时才能读到真实密值），重启 DSH 生效。面板中的密值始终以掩码显示。</p>

            <h2>5. 启用/禁用服务</h2>
            <p>「禁用」会把该服务条目安全移出运行配置并暂存（其余条目与你手工添加的内容零改动），需要时一键恢复。</p>
          </article>

          <article id="snapshots">
            <h1>快照与离线部署</h1>
            <p>「快照与离线打包」面向两类场景：环境迁移（快照导出/比对）与无外网环境（离线包）。入口：客户端「快照与离线打包」标签页。</p>

            <h2>快照导出与比对</h2>
            <ol>
              <li>「导出快照」记录当前插件目录的完整状态（插件清单、版本、启用状态、MCP 配置结构）</li>
              <li>把快照文件带到另一台机器，或留作基线</li>
              <li>「导入快照」时与目标机当前状态逐项比对，列出差异后按需补齐</li>
            </ol>

            <h2>离线部署（两步完成）</h2>
            <ol>
              <li><strong>联网机生成离线包：</strong>选择要带走的插件，生成自包含 <code>.zip</code>（含全部插件文件与依赖清单、校验信息）</li>
              <li><strong>离线机导入：</strong>目标机安装客户端后「导入离线包」，逐项校验完整性并还原</li>
            </ol>
            <p>详见官网<a href="/offline">离线部署指引</a>。</p>
          </article>

          <article id="server-panel">
            <h1>DSH Web 服务器面板</h1>
            <p>客户端内置本地 DSH Web 服务器的启停面板。入口：顶栏服务器图标（闪电按钮）。</p>

            <h2>功能</h2>
            <ul>
              <li><strong>状态查看：</strong>运行状态、监听地址与端口（带 token 的访问地址一键复制/直达）</li>
              <li><strong>启停控制：</strong>启动 / 停止 / 重启本地 DSH Web 服务器</li>
              <li><strong>路径自动探测：</strong>自动定位本机 DSH 安装目录（apps/cli）与 Node 运行时；也可用环境变量 <code>DSH_WEB_DIR</code>、<code>DSH_NODE_PATH</code> 显式指定</li>
            </ul>
            <div class="warning-box">
              <strong>注意：</strong>面板只做进程级启停，不修改 DSH 自身配置；服务器崩溃后可从面板一键重启。
            </div>
          </article>

          <article id="security">
            <h1>安全设计</h1>
            <p>v1.14.0 起全链路启用 Ed25519 签名验证，防止目录数据与更新包在传输途中被篡改。</p>

            <h2>验证范围</h2>
            <table class="field-table">
              <thead>
                <tr><th>数据流</th><th>机制</th><th>失败行为</th></tr>
              </thead>
              <tbody>
                <tr><td>插件目录（<code>/api/plugins</code>）</td><td>响应体 Ed25519 验签（X-DSH-SIGNATURE 头）</td><td>拒绝消费 + 降级本地缓存（标注 sig-fallback）</td></tr>
                <tr><td>自更新清单（<code>/api/updater/latest</code>）</td><td>同上</td><td>fail-closed，不提示更新</td></tr>
                <tr><td>更新包下载</td><td>SHA256 完整性校验</td><td>终止安装</td></tr>
                <tr><td>MCP 密钥</td><td>加密存系统凭据库</td><td>—（本地存储，不经网络）</td></tr>
              </tbody>
            </table>

            <h2>密钥管理</h2>
            <ul>
              <li>签名私钥仅存在于官网服务器（环境变量注入路径），从不入库、不打安装包</li>
              <li>客户端只内置公钥（32 字节），密钥已历经三次轮换，历史泄漏密钥均已作废</li>
              <li>签名失败时界面顶部出现红色告警条，可手动重试验证</li>
            </ul>

            <h2>过渡语义（v1.14.0 之前的老客户端）</h2>
            <p>老版本客户端内置旧公钥，看到新签名会得到「验证失败」结果，自动降级本地缓存目录数据，仍可正常自更新到 1.14.0；官网过渡期对未签名响应 fail-open，不拦截老版本。升级到 1.14.0 后即恢复完整验签。</p>
          </article>

          <article id="self-update">
            <h1>自动更新</h1>
            <p>客户端启动时后台检查新版本（可关闭）。发现新版本后：</p>
            <ol>
              <li>从官网拉取清单并验证 Ed25519 签名（失败则放弃本次更新）</li>
              <li>下载对应平台安装包并校验 SHA256</li>
              <li>提示用户确认后执行安装</li>
            </ol>
            <p>清单中的 <code>changelog</code> 会在更新提示里逐条展示；「跳过此版本」的记录保存在本地。</p>
          </article>

          <article id="plugin-manifest">
            <h1>插件清单规范</h1>
            <p>每个插件目录下需要包含一个 <code>plugin.manifest.json</code> 文件，用于描述插件的基本信息。</p>

            <h2>字段说明</h2>
            <table class="field-table">
              <thead>
                <tr>
                  <th>字段</th>
                  <th>类型</th>
                  <th>必填</th>
                  <th>说明</th>
                </tr>
              </thead>
              <tbody>
                <tr>
                  <td><code>id</code></td>
                  <td>string</td>
                  <td>是</td>
                  <td>插件唯一标识符，建议使用小写字母和连字符</td>
                </tr>
                <tr>
                  <td><code>name</code></td>
                  <td>string</td>
                  <td>是</td>
                  <td>插件显示名称</td>
                </tr>
                <tr>
                  <td><code>description</code></td>
                  <td>string</td>
                  <td>否</td>
                  <td>插件功能介绍</td>
                </tr>
                <tr>
                  <td><code>github_repo</code></td>
                  <td>string</td>
                  <td>否</td>
                  <td>GitHub 仓库地址，格式: owner/repo</td>
                </tr>
                <tr>
                  <td><code>current_version</code></td>
                  <td>string</td>
                  <td>否</td>
                  <td>当前版本号，语义化版本格式</td>
                </tr>
                <tr>
                  <td><code>enabled</code></td>
                  <td>boolean</td>
                  <td>否</td>
                  <td>是否启用，默认 true</td>
                </tr>
                <tr>
                  <td><code>type</code></td>
                  <td>string</td>
                  <td>否</td>
                  <td>类型: plugin 或 agent-core，默认 plugin</td>
                </tr>
                <tr>
                  <td><code>author</code></td>
                  <td>string</td>
                  <td>否</td>
                  <td>作者名称</td>
                </tr>
                <tr>
                  <td><code>homepage</code></td>
                  <td>string</td>
                  <td>否</td>
                  <td>插件主页地址</td>
                </tr>
              </tbody>
            </table>

            <h2>示例</h2>
            <pre><code>{
  "id": "dsh-plugin-example",
  "name": "示例插件",
  "description": "这是一个示例插件，展示 manifest 格式",
  "github_repo": "Frank-NF/dsh-plugin-example",
  "current_version": "1.0.0",
  "enabled": true,
  "type": "plugin",
  "author": "DSH Team",
  "homepage": "https://dsh.huilinsh.cn"
}</code></pre>
          </article>

          <article id="proxy-setup">
            <h1>代理服务部署（自建可选）</h1>
            <p>客户端默认直连 GitHub 与 npm 官方源。如果你的网络环境无法直连，可自建代理服务。以下是部署指南。</p>

            <h2>使用 Docker 部署（推荐）</h2>
            <ol>
              <li>克隆项目
                <pre><code>git clone https://github.com/Frank-NF/DSH-PluginUpdater.git
cd DSH-PluginUpdater/proxy-server</code></pre>
              </li>
              <li>复制环境变量配置文件
                <pre><code>cp .env.example .env</code></pre>
              </li>
              <li>编辑 .env 文件，配置 Token 和其他参数</li>
              <li>启动服务
                <pre><code>docker-compose up -d</code></pre>
              </li>
              <li>服务将在 8080 端口运行，配置 Nginx 反向代理并启用 HTTPS</li>
            </ol>

            <h2>环境变量说明</h2>
            <ul>
              <li><code>PORT</code>: 服务监听端口，默认 8080</li>
              <li><code>GITHUB_TOKEN</code>: GitHub Personal Access Token，提高 API 限流</li>
              <li><code>PROXY_TOKEN</code>: 代理访问 Token，配置后客户端必须携带</li>
              <li><code>REDIS_ADDR</code>: Redis 地址，用于缓存 API 响应</li>
              <li><code>CACHE_TTL_MINUTES</code>: 缓存过期时间，默认 30 分钟</li>
              <li><code>ALLOWED_REPOS</code>: 允许的仓库白名单，逗号分隔</li>
            </ul>

            <h2>Nginx 反向代理配置示例</h2>
            <pre><code>server {
    listen 443 ssl http2;
    server_name dsh.huilinsh.cn;

    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;

    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_read_timeout 300s;
    }
}</code></pre>
          </article>

          <article id="faq">
            <h1>常见问题</h1>

            <h3>Q: 工具提示"检查更新失败"怎么办？</h3>
            <p>A: 请检查网络连接是否正常。可在设置中确认网络配置；自建代理用户请确认代理服务可访问。</p>

            <h3>Q: 顶部出现红色"目录签名验证失败"告警？</h3>
            <p>A: 说明官网返回的插件目录数据未通过 Ed25519 验签。工具已自动降级到本地缓存数据（功能不受影响）。点击告警条上的重试可重新验证；若持续出现请通过问题反馈渠道上报。</p>

            <h3>Q: 更新插件时提示"文件被占用"？</h3>
            <p>A: 这是因为 DSH Agent 正在运行并加载了该插件。请先关闭 DSH Agent 本体，再执行更新操作。</p>

            <h3>Q: 如何恢复被误删的插件？</h3>
            <p>A: 工具在卸载和更新前都会自动备份。可以在设置中查看备份列表，选择对应的备份进行恢复。</p>

            <h3>Q: 插件没有 GitHub 仓库能使用吗？</h3>
            <p>A: 可以。工具仍能扫描和管理这类插件的启用/禁用和卸载，但无法检测更新和在线升级。</p>

            <h3>Q: 支持哪些版本号格式？</h3>
            <p>A: 工具使用语义化版本（Semantic Versioning）规范，格式为 MAJOR.MINOR.PATCH，例如 1.2.3。也支持 v 前缀，如 v1.2.3。</p>

            <h3>Q: 工具会收集我的数据吗？</h3>
            <p>A: 不会。所有插件扫描和管理操作都在本地完成，仅在检查更新和下载时访问 GitHub/npm 与官网清单，不收集任何用户数据。MCP 密钥加密保存在本机系统凭据库，不上传。</p>

            <h3>Q: 之前的"在线版"去哪了？</h3>
            <p>A: 在线版已于 v1.13.15 下线，插件管理能力统一收敛到桌面客户端（功能更全：组合包、MCP 面板、快照与离线部署均为客户端独占）。插件市场浏览仍可在官网完成。</p>
          </article>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
useHead({
  title: '使用文档 - DSH 插件升级管理工具',
})
</script>

<style scoped>
.docs-section {
  padding: 40px 0;
  min-height: calc(100vh - 64px);
}

.docs-container {
  display: flex;
  gap: 40px;
  align-items: flex-start;
}

.docs-sidebar {
  width: 240px;
  flex-shrink: 0;
  position: sticky;
  top: 100px;
}

.docs-sidebar h3 {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 16px;
  padding-bottom: 12px;
  border-bottom: 2px solid var(--primary);
}

.docs-sidebar ul {
  list-style: none;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.docs-sidebar a {
  display: block;
  padding: 7px 12px;
  border-radius: 8px;
  font-size: 13.5px;
  color: var(--text-secondary);
  transition: all 0.18s ease;
}

.docs-sidebar a:hover,
.docs-sidebar a.active {
  color: var(--primary-light);
  background: rgba(99, 102, 241, 0.1);
}

.docs-content {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 56px;
}

.docs-content article {
  scroll-margin-top: 90px;
}

.docs-content h1 {
  font-size: 28px;
  font-weight: 700;
  color: var(--text-primary);
  margin-bottom: 18px;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--border);
}

.docs-content h2 {
  font-size: 19px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 28px 0 12px;
}

.docs-content h3 {
  font-size: 15.5px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 20px 0 8px;
}

.docs-content p {
  font-size: 14.5px;
  color: var(--text-secondary);
  line-height: 1.85;
  margin-bottom: 12px;
}

.docs-content ul,
.docs-content ol {
  padding-left: 24px;
  color: var(--text-secondary);
  line-height: 1.9;
  font-size: 14.5px;
  margin-bottom: 14px;
}

.docs-content a {
  color: var(--primary-light);
}

.docs-content code {
  font-size: 13px;
  color: var(--primary-light);
  background: rgba(0, 0, 0, 0.3);
  padding: 2px 7px;
  border-radius: 5px;
}

.docs-content pre {
  background: rgba(0, 0, 0, 0.35);
  border: 1px solid var(--border);
  border-radius: 10px;
  padding: 16px 18px;
  overflow-x: auto;
  margin: 10px 0 16px;
}

.docs-content pre code {
  background: none;
  padding: 0;
  font-size: 13px;
  line-height: 1.7;
}

.field-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13.5px;
  margin: 12px 0 18px;
}

.field-table th,
.field-table td {
  padding: 9px 12px;
  border: 1px solid var(--border);
  text-align: left;
  color: var(--text-secondary);
  vertical-align: top;
}

.field-table th {
  color: var(--text-primary);
  background: rgba(99, 102, 241, 0.08);
  font-weight: 600;
}

.warning-box {
  background: rgba(245, 158, 11, 0.1);
  border: 1px solid rgba(245, 158, 11, 0.3);
  border-radius: 10px;
  padding: 12px 16px;
  font-size: 13.5px;
  color: var(--text-secondary);
  line-height: 1.7;
  margin: 12px 0 16px;
}

@media (max-width: 900px) {
  .docs-container {
    flex-direction: column;
  }
  .docs-sidebar {
    width: 100%;
    position: static;
  }
  .docs-sidebar ul {
    flex-direction: row;
    flex-wrap: wrap;
    gap: 4px;
  }
}
</style>
