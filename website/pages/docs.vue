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
            <li><a href="#plugin-manifest">插件清单规范</a></li>
            <li><a href="#proxy-setup">代理服务部署</a></li>
            <li><a href="#faq">常见问题</a></li>
          </ul>
        </aside>

        <div class="docs-content">
          <article id="quick-start">
            <h1>快速开始</h1>
            <p>DSH 插件升级管理工具是一个独立运行的桌面程序，不依赖 DSH Agent 本体。它可以扫描您的插件目录，管理所有已安装插件的更新、启用、禁用和卸载。</p>

            <h2>系统要求</h2>
            <ul>
              <li><strong>Windows:</strong> Windows 10 或更高版本（64位）</li>
              <li><strong>Linux:</strong> Ubuntu 20.04+ / Debian 11+（x86_64）</li>
              <li><strong>网络:</strong> 可访问代理服务器</li>
            </ul>
          </article>

          <article id="installation">
            <h1>安装指南</h1>

            <h2>Windows 安装</h2>
            <ol>
              <li>从<a href="/download">下载页面</a>获取最新的 .exe 安装包</li>
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
            <p>点击「检查更新」按钮，工具将通过网络代理访问 GitHub，检测每个插件的最新版本。有可用更新的插件将高亮显示。</p>

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
            <h1>代理服务部署</h1>
            <p>代理服务是工具能够正常访问 GitHub 的关键。以下是部署指南。</p>

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
            <p>A: 请检查网络连接是否正常，代理服务是否可访问。可以在设置中确认代理地址配置正确。</p>

            <h3>Q: 更新插件时提示"文件被占用"？</h3>
            <p>A: 这是因为 DSH Agent 正在运行并加载了该插件。请先关闭 DSH Agent 本体，再执行更新操作。</p>

            <h3>Q: 如何恢复被误删的插件？</h3>
            <p>A: 工具在卸载和更新前都会自动备份。可以在设置中查看备份列表，选择对应的备份进行恢复。</p>

            <h3>Q: 插件没有 GitHub 仓库能使用吗？</h3>
            <p>A: 可以。工具仍能扫描和管理这类插件的启用/禁用和卸载，但无法检测更新和在线升级。</p>

            <h3>Q: 支持哪些版本号格式？</h3>
            <p>A: 工具使用语义化版本（Semantic Versioning）规范，格式为 MAJOR.MINOR.PATCH，例如 1.2.3。也支持 v 前缀，如 v1.2.3。</p>

            <h3>Q: 工具会收集我的数据吗？</h3>
            <p>A: 不会。所有插件扫描和管理操作都在本地完成，仅在检查更新和下载时通过网络代理访问 GitHub，不收集任何用户数据。</p>
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
  padding: 0;
}

.docs-sidebar li {
  margin-bottom: 4px;
}

.docs-sidebar a {
  display: block;
  padding: 8px 12px;
  color: var(--text-secondary);
  font-size: 14px;
  border-radius: 6px;
  transition: all var(--dur) var(--ease);
}

.docs-sidebar a:hover,
.docs-sidebar a.active {
  background: rgba(99, 102, 241, 0.14);
  color: var(--primary-light);
}

.docs-content {
  flex: 1;
  min-width: 0;
}

.docs-content article {
  margin-bottom: 60px;
}

.docs-content h1 {
  font-size: 28px;
  font-weight: 700;
  color: var(--text-primary);
  margin-bottom: 20px;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--glass-border);
}

.docs-content h2 {
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 28px 0 16px;
}

.docs-content h3 {
  font-size: 16px;
  font-weight: 600;
  color: var(--primary-light);
  margin: 24px 0 12px;
}

.docs-content p {
  font-size: 15px;
  line-height: 1.8;
  color: var(--text-secondary);
  margin-bottom: 16px;
}

.docs-content ul,
.docs-content ol {
  padding-left: 24px;
  margin-bottom: 16px;
}

.docs-content li {
  font-size: 15px;
  line-height: 1.8;
  color: var(--text-secondary);
  margin-bottom: 8px;
}

.docs-content code {
  background: rgba(0, 0, 0, 0.3);
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 13px;
  color: var(--warning);
  font-family: 'JetBrains Mono', 'Consolas', monospace;
}

.docs-content pre {
  background: rgba(0, 0, 0, 0.35);
  border: 1px solid var(--glass-border);
  padding: 16px 20px;
  border-radius: 10px;
  overflow-x: auto;
  margin: 16px 0;
}

.docs-content pre code {
  background: none;
  color: var(--info);
  padding: 0;
  font-size: 13px;
  line-height: 1.6;
}

.warning-box {
  background: rgba(245, 158, 11, 0.1);
  border-left: 4px solid var(--warning);
  padding: 16px 20px;
  border-radius: 0 10px 10px 0;
  margin: 16px 0;
}

.warning-box strong {
  color: var(--warning);
}

.warning-box p {
  margin: 8px 0 0;
  color: var(--text-secondary);
}

.field-table {
  width: 100%;
  border-collapse: collapse;
  margin: 16px 0;
}

.field-table th,
.field-table td {
  padding: 10px 14px;
  text-align: left;
  border: 1px solid var(--glass-border);
  font-size: 14px;
}

.field-table th {
  background: rgba(255, 255, 255, 0.05);
  font-weight: 600;
  color: var(--text-primary);
}

.field-table td {
  color: var(--text-secondary);
}

@media (max-width: 900px) {
  .docs-container {
    flex-direction: column;
  }

  .docs-sidebar {
    width: 100%;
    position: static;
  }
}
</style>
