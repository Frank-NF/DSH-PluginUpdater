# 部署指南

## 桌面客户端分发

### Windows 安装包

1. 构建 MSI 安装包

```bash
cd src-tauri
cargo tauri build
```

2. 产物位置：`src-tauri/target/release/bundle/msi/`

3. 可选：使用 Inno Setup 制作更友好的安装程序

### Linux AppImage

1. 构建 AppImage

```bash
cd src-tauri
cargo tauri build
```

2. 产物位置：`src-tauri/target/release/bundle/appimage/`

3. 分发前测试

```bash
chmod +x DSH-PluginUpdater_*.AppImage
./DSH-PluginUpdater_*.AppImage
```

## 香港中转代理服务部署

### 方案一：Docker Compose 部署（推荐）

#### 1. 服务器要求

- 香港地区云服务器（推荐：阿里云香港、腾讯云香港、AWS 香港）
- 配置：1核 2GB 起步，推荐 2核 4GB
- 操作系统：Ubuntu 22.04 LTS / Debian 12
- 开放端口：80, 443, 22

#### 2. 安装 Docker

```bash
# Ubuntu/Debian
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh
sudo usermod -aG docker $USER

# 安装 Docker Compose
sudo apt install -y docker-compose-plugin

# 验证
docker --version
docker compose version
```

#### 3. 部署服务

```bash
# 克隆项目
git clone https://github.com/DSH-Team/DSH-PluginUpdater.git
cd DSH-PluginUpdater/proxy-server

# 配置环境变量
cp .env.example .env
nano .env
```

编辑 `.env` 文件：

```env
PORT=8080
GITHUB_TOKEN=ghp_xxxxxxxxxxxxxxxxxxxx  # 可选，提高 API 限流
PROXY_TOKEN=your-secret-proxy-token     # 建议设置，防止滥用
REDIS_ADDR=redis:6379
REDIS_PASSWORD=
REDIS_DB=0
CACHE_TTL_MINUTES=30
ALLOWED_REPOS=DSH-Team/DSH-Agent,DSH-Team/dsh-plugin-example
```

#### 4. 启动服务

```bash
# 构建并启动
docker compose up -d --build

# 查看状态
docker compose ps

# 查看日志
docker compose logs -f proxy

# 测试
curl http://localhost:8080/health
```

#### 5. 配置 Nginx 反向代理 + HTTPS

安装 Nginx 和 Certbot：

```bash
sudo apt install -y nginx certbot python3-certbot-nginx
```

创建 Nginx 配置：

```bash
sudo nano /etc/nginx/sites-available/proxy.dsh-update.hk
```

配置内容：

```nginx
server {
    listen 80;
    server_name proxy.dsh-update.hk;

    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_read_timeout 300s;
        proxy_send_timeout 300s;
        client_max_body_size 500M;
    }
}
```

启用配置并申请 SSL 证书：

```bash
sudo ln -s /etc/nginx/sites-available/proxy.dsh-update.hk /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx

# 申请 Let's Encrypt 证书
sudo certbot --nginx -d proxy.dsh-update.hk
```

#### 6. 验证部署

```bash
# 健康检查
curl https://proxy.dsh-update.hk/health

# 测试 GitHub API 代理
curl -H "X-Proxy-Token: your-token" "https://proxy.dsh-update.hk/api/github/latest?repo=DSH-Team/DSH-Agent"
```

### 方案二：直接部署（无 Docker）

```bash
# 安装 Go
wget https://go.dev/dl/go1.21.0.linux-amd64.tar.gz
sudo tar -C /usr/local -xzf go1.21.0.linux-amd64.tar.gz
echo 'export PATH=$PATH:/usr/local/go/bin' >> ~/.bashrc
source ~/.bashrc

# 安装 Redis
sudo apt install -y redis-server
sudo systemctl enable redis-server
sudo systemctl start redis-server

# 构建服务
cd proxy-server
go build -o dsh-proxy .

# 创建 systemd 服务
sudo nano /etc/systemd/system/dsh-proxy.service
```

systemd 服务配置：

```ini
[Unit]
Description=DSH Plugin Proxy Server
After=network.target redis-server.service

[Service]
Type=simple
User=www-data
WorkingDirectory=/opt/dsh-proxy
ExecStart=/opt/dsh-proxy/dsh-proxy
Restart=always
RestartSec=5
EnvironmentFile=/opt/dsh-proxy/.env

[Install]
WantedBy=multi-user.target
```

启动服务：

```bash
sudo cp dsh-proxy /opt/dsh-proxy/
sudo cp .env /opt/dsh-proxy/
sudo systemctl daemon-reload
sudo systemctl enable dsh-proxy
sudo systemctl start dsh-proxy
sudo systemctl status dsh-proxy
```

## 官方网站部署

### 方案一：Node.js 服务器

```bash
cd website
npm install
npm run build

# 使用 PM2 管理进程
npm install -g pm2
pm2 start .output/server/index.mjs --name dsh-website
pm2 save
pm2 startup
```

### 方案二：静态站点托管

```bash
cd website
npm run generate
# 产物在 .output/public/
# 上传到 Vercel / Netlify / Cloudflare Pages / 阿里云 OSS
```

### Nginx 配置（Node.js 模式）

```nginx
server {
    listen 443 ssl http2;
    server_name dsh-update.hk www.dsh-update.hk;

    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;

    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # 静态资源缓存
    location /_nuxt/ {
        proxy_pass http://127.0.0.1:3000;
        expires 1y;
        add_header Cache-Control "public, immutable";
    }
}
```

## 域名与 DNS 配置

### 推荐域名结构

| 域名 | 用途 |
|------|------|
| `dsh-update.hk` | 官方网站 |
| `proxy.dsh-update.hk` | 香港中转代理服务 |
| `download.dsh-update.hk` | 下载资源（可选，可使用 CDN） |

### DNS 配置示例

```
A       dsh-update.hk          <网站服务器IP>
A       proxy.dsh-update.hk    <香港代理服务器IP>
CNAME   www.dsh-update.hk      dsh-update.hk
```

## 监控与维护

### 代理服务监控

```bash
# 查看服务状态
docker compose ps

# 查看日志
docker compose logs -f --tail=100 proxy

# 重启服务
docker compose restart proxy

# 更新服务
git pull
docker compose up -d --build
```

### Redis 维护

```bash
# 进入 Redis 容器
docker compose exec redis redis-cli

# 查看缓存键
KEYS *

# 清空缓存
FLUSHALL

# 查看内存使用
INFO memory
```

### 日志轮转

Docker 容器日志配置（docker-compose.yml）：

```yaml
services:
  proxy:
    logging:
      driver: "json-file"
      options:
        max-size: "100m"
        max-file: "3"
```

## 安全建议

1. **设置 PROXY_TOKEN**：防止代理服务被滥用
2. **配置 ALLOWED_REPOS**：限制只能访问指定的 GitHub 仓库
3. **启用 HTTPS**：所有通信加密传输
4. **定期更新**：保持 Go 依赖和 Docker 镜像最新
5. **防火墙配置**：只开放必要端口（80, 443, 22）
6. **GitHub Token 权限**：使用最小权限的 Token，只需要 public_repo 权限

## 备份策略

### 代理服务配置备份

```bash
# 备份 .env 和 docker-compose.yml
tar -czf dsh-proxy-backup-$(date +%Y%m%d).tar.gz .env docker-compose.yml
```

### Redis 数据备份

```bash
docker compose exec redis redis-cli BGSAVE
docker cp dsh-proxy-redis:/data/dump.rdb ./backup-$(date +%Y%m%d).rdb
```

## 故障排查

### 服务无法启动

```bash
# 查看详细日志
docker compose logs proxy --tail=50

# 检查端口占用
sudo netstat -tlnp | grep 8080

# 检查 .env 配置
cat .env
```

### GitHub API 限流

- 配置 `GITHUB_TOKEN` 提高限流额度
- 增加 Redis 缓存时间（`CACHE_TTL_MINUTES`）
- 监控 API 调用量

### 下载速度慢

- 确认服务器在香港地区
- 检查服务器带宽
- 考虑使用 CDN 加速大文件下载
