/**
 * GET /api/updater/latest
 * 桌面端自身更新通道（官网权威源）
 * 返回：最新版本号、各平台安装包地址、SHA256、changelog、是否强制更新
 *
 * 数据源：/var/www/dsh-updater/version.json（由发布脚本维护）
 * 兜底：默认返回当前已知版本，不抛错（桌面端静默降级）
 */
import { readFile } from 'node:fs/promises'

const VERSION_FILE = '/var/www/dsh-updater/version.json'
const FALLBACK = {
  version: '1.0.0',
  release_url: 'https://dsh.huilinsh.cn/download',
  is_mandatory: false,
}

export default defineEventHandler(async () => {
  try {
    const raw = await readFile(VERSION_FILE, 'utf8')
    const data = JSON.parse(raw)
    return {
      version: data.version || FALLBACK.version,
      platforms: data.platforms || {},
      release_url: data.release_url || FALLBACK.release_url,
      changelog: data.changelog || [],
      is_mandatory: !!data.is_mandatory,
      published_at: data.published_at || null,
    }
  } catch {
    return FALLBACK
  }
})
