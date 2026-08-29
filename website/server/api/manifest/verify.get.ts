/**
 * GET /api/manifest/verify?signature=xxx&hash=xxx
 * 验证插件目录签名
 */
import { createVerify } from 'node:crypto'
import { readFileSync } from 'node:fs'

const PUBLIC_KEY_PATH = process.env.DSH_SIGNING_PUB_KEY || '/var/www/dsh-updater/ed25519-public.pem'

export default defineEventHandler(async (event) => {
  const query = getQuery(event)
  const signature = (query.signature as string) || ''
  const hash = (query.hash as string) || ''
  
  if (!signature || !hash) {
    throw createError({ statusCode: 400, statusMessage: '缺少 signature 或 hash 参数' })
  }
  
  try {
    const verifier = createVerify('SHA256')
    verifier.update(Buffer.from(hash, 'hex'))
    const valid = verifier.verify(readFileSync(PUBLIC_KEY_PATH), signature, 'hex')
    return { valid, signature, hash }
  } catch (e) {
    return { valid: false, error: (e as Error).message }
  }
})
