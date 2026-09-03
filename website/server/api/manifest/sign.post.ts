/**
 * POST /api/manifest/sign
 * 对插件目录数据签名，返回 Ed25519 签名（供桌面端验证）
 */
import { getPlugins } from '~/server/utils/github'
import { createHash, createSign } from 'node:crypto'
import { readFileSync } from 'node:fs'

const PRIVATE_KEY_PATH = process.env.DSH_SIGNING_KEY_PATH || '/var/www/dsh-updater/ed25519-private.pem'

export default defineEventHandler(async () => {
  const plugins = await getPlugins()
  
  const canonical = JSON.stringify(
    plugins.map(p => ({ id: p.id, repo: p.repo, name: p.name, category: p.category })).sort((a, b) => a.id.localeCompare(b.id))
  )
  
  const hash = createHash('sha256').update(canonical).digest()
  const signer = createSign('SHA256')
  signer.update(hash)
  const sig = signer.sign(readFileSync(PRIVATE_KEY_PATH), 'hex')
  
  return { signature: sig, hash: hash.toString('hex'), count: plugins.length, signed_at: new Date().toISOString() }
})
