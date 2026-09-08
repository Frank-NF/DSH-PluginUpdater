/**
 * GET /api/updater/latest
 * 桌面端自身更新通道（官网权威源）
 * 返回：最新版本号、各平台安装包地址、SHA256、changelog、是否强制更新
 *
 * 数据源：/var/www/dsh-updater/version.json（由发布脚本维护）
 * 兜底：version='0.0.0'（客户端 semver 比较 0.0.0 < any >=1.0.0，不会触发误降级）
 *
 * 安全：若私钥可用，对原始 body 字节作 Ed25519 签名并通过 X-DSH-SIGNATURE 响应头
 *       返回。桌面端 check_self_update / check_auto_update_background 会验签，
 *       签名失败则拒绝更新（防御中间人篡改自更新包）。
 */
import { readFile } from 'node:fs/promises'

const VERSION_FILE = '/var/www/dsh-updater/version.json'
const FALLBACK = {
  version: '0.0.0',
  release_url: 'https://dsh.huilinsh.cn/download',
  is_mandatory: false,
}

export default defineEventHandler(async (event) => {
  let payload: Record<string, unknown>
  try {
    const raw = await readFile(VERSION_FILE, 'utf8')
    const data = JSON.parse(raw)
    payload = {
      version: data.version || FALLBACK.version,
      platforms: data.platforms || {},
      release_url: data.release_url || FALLBACK.release_url,
      changelog: data.changelog || [],
      is_mandatory: !!data.is_mandatory,
      published_at: data.published_at || null,
    }
  } catch {
    payload = FALLBACK
  }

  // Ed25519 签名：仅当私钥可用且 VERSION_FILE 可读取时注入 X-DSH-SIGNATURE
  try {
    const { sign } = await import('node:crypto')
    const { readFileSync } = await import('node:fs')
    const keyPath = process.env.DSH_SIGNING_KEY_PATH || '/var/www/dsh-updater/ed25519-private.pem'
    const bodyStr = JSON.stringify(payload)
    const sigHex = sign(null, Buffer.from(bodyStr), readFileSync(keyPath)).toString('hex')
    event.node.res.setHeader('X-DSH-SIGNATURE', sigHex)
    event.node.res.setHeader('Content-Type', 'application/json')
    event.node.res.end(bodyStr)
    return
  } catch {
    // 私钥不可用 → 不发签名头，客户端 fail-open（过渡期兼容旧客户端）
  }

  return payload
})
