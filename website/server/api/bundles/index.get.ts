/**
 * GET /api/bundles
 * 组合包全量索引（官网权威源，供桌面端 / 官网消费）
 *
 * 查询参数：
 * - q=xxx          关键词搜索（名称 / 描述 / 标签）
 * - sort=create_time|name  排序（默认 create_time 倒序）
 * - page=1&page_size=50    分页（page_size<=200）
 *
 * 缓存：支持 ETag / If-None-Match 增量拉取
 */
import { seedBundles, buildBundle, type BundleRow } from '../../utils/bundles'

export default defineEventHandler(async (event) => {
  const db = getDB()
  seedBundles(db)

  const rows = db.prepare('SELECT * FROM bundles').all() as unknown as BundleRow[]
  let list = rows.map((row) => buildBundle(db, row))

  const query = getQuery(event)
  const q = ((query.q as string) || '').trim().toLowerCase()
  if (q) {
    list = list.filter(
      (b) =>
        b.name.toLowerCase().includes(q) ||
        b.description.toLowerCase().includes(q) ||
        b.tags.some((t) => t.toLowerCase().includes(q))
    )
  }

  const sort = (query.sort as string) || 'create_time'
  if (sort === 'name') {
    list.sort((a, b) => a.name.localeCompare(b.name, 'zh-CN'))
  } else {
    list.sort((a, b) => b.createTime.localeCompare(a.createTime) || a.id.localeCompare(b.id))
  }

  const page = Math.max(1, Number(query.page) || 1)
  const pageSize = Math.min(200, Math.max(1, Number(query.page_size) || 50))
  const total = list.length
  const paged = list.slice((page - 1) * pageSize, page * pageSize)

  // ETag：基于总数 + 最新创建时间，支持增量拉取（与 /api/plugins 同风格）
  const latestCreate = rows
    .map((r) => r.create_time)
    .filter(Boolean)
    .sort()
    .reverse()[0]
  const etag = '"' + total + '-' + (latestCreate || 'none') + '"'
  setHeader(event, 'ETag', etag)
  setHeader(event, 'Cache-Control', 'public, max-age=600')
  const inm = getHeader(event, 'if-none-match')
  if (inm && inm === etag) {
    setResponseStatus(event, 304)
    return null
  }

  return {
    total,
    page,
    page_size: pageSize,
    updated_at: new Date().toISOString(),
    bundles: paged
  }
})
