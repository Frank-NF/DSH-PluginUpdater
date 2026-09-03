# git push via relay server (dynamic socks tunnel)
# 用法: .\scripts\push-via-relay.ps1 [remote] [refspec...]
# 默认: origin main --tags
param(
  [string]$Remote = "origin",
  [string[]]$Refspec = @("main", "--tags"),
  [int]$Port = 1789
)
$ErrorActionPreference = "Stop"

# 隧道若未在跑则隐藏启动（ssh -D 动态转发，经中转服务器出网）
$listening = Get-NetTCPConnection -LocalPort $Port -State Listen -ErrorAction SilentlyContinue
if (-not $listening) {
  Start-Process -WindowStyle Hidden ssh -ArgumentList "-D", "127.0.0.1:$Port", "-N", "-o", "ExitOnForwardFailure=yes", "-o", "ServerAliveInterval=30", "-i", "$env:USERPROFILE\.ssh\id_ed25519", "root@64.90.30.139"
  Start-Sleep -Seconds 3
  $listening = Get-NetTCPConnection -LocalPort $Port -State Listen -ErrorAction SilentlyContinue
  if (-not $listening) { Write-Error "relay tunnel failed to start on port $Port" }
}
Write-Host "relay tunnel up on 127.0.0.1:$Port"

git -c http.proxy="socks5h://127.0.0.1:$Port" -c https.proxy="socks5h://127.0.0.1:$Port" push $Remote @Refspec
Write-Host "push_exit=$LASTEXITCODE"
exit $LASTEXITCODE
