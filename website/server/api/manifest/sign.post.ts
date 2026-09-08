/**
 * POST /api/manifest/sign
 * 对原始 body 字符串作 Ed25519 签名，返回 hex 签名 + meta
 *
 * 安全：使用 node:crypto.sign(null, data, key) 原生 API（与桌面端 dalek::PublicKey
 *       verify_strict 对齐），不再使用 createSign('SHA256') —— Ed25519 密钥不兼容该 wrapper。
 */
import { getPlugins } from '~/server/utils/github'
import { sign } from 'node:crypto'
import { readFileSync } from 'node:fs'

const PRIVATE_KEY_PATH = process.env.DSH_SIGNING_KEY_PATH || '/var/www/dsh-updater/ed25519-private.pem'
const PUBLIC_KEY_PATH = process.env.DSH_SIGNING_PUB_KEY || '/var/www/dsh-updater/ed25519-public.pem'

export default defineEventHandler(async () => {
  const plugins = await getPlugins()

  // 构建与 /api/plugins 一致的 canonical 投影（便于跨接口校验）
  const canonical = JSON.stringify(
    plugins.map(p => ({ id: p.id, repo: p.repo, name: p.name, category: p.category })).sort((a, b) => a.id.localeCompare(b.id))
  )

  try {
    // Ed25519 原生签名：sign(null, data, privateKey) = Go crypto/ed25519.Sign
    const sigHex = sign(null, Buffer.from(canonical), readFileSync(PRIVATE_KEY_PATH)).toString('hex')
    // 附带公钥 PEM 用于第三方验证（无需本地文件系统读取）
    const pubPem = (() => { try { return readFileSync(PUBLIC_KEY_PATH, 'utf8').trim() } catch { return '' } })()
    return {
      signature: sigHex,
      count: plugins.length,
      signed_at: new Date().toISOString(),
      public_key_pem: pubPem,
    }
  } catch (e) {
    throw createError({ statusCode: 500, statusMessage: `Ed25519 签名失败: ${(e as Error).message}` })
  }
})
