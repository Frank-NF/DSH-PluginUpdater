/**
 * GET /api/plugins
 * 返回插件市场数据（官网权威源，供桌面端/在线版/官网消费）
 *
 * 查询参数：
 * - q=xxx             关键词搜索（名称/描述/仓库/分类）
 * - sort=stars|name|hot|latest  排序（默认 stars；hot=热度；latest=最近更新）
 * - category=xxx      按分类筛选
 * - page=1&page_size=48  分页（默认 page_size=48）
 * - fields=basic|full  basic=精简；full=完整（默认 full 兼容旧客户端）
 *
 * 缓存：支持 ETag / If-None-Match 增量拉取
 */
export default defineEventHandler(async (event) => {
  const query = getQuery(event)
  const q = (query.q as string)?.trim() || ''
  const sort = (query.sort as string) || 'stars'
  const category = query.category as string | undefined
  const page = Math.max(1, Number(query.page) || 1)
  const pageSize = Math.min(200, Math.max(1, Number(query.page_size) || 48))
  const fields = (query.fields as string) || 'full'

  let plugins = await getPlugins()

  // 分类筛选
  if (category && category !== '全部') {
    plugins = plugins.filter((p) => p.category === category)
  }

  // 关键词搜索（名称/仓库/描述/分类）
  if (q) {
    const lower = q.toLowerCase()
    plugins = plugins.filter(
      (p) =>
        p.name.toLowerCase().includes(lower) ||
        p.repo.toLowerCase().includes(lower) ||
        p.description.toLowerCase().includes(lower) ||
        (p.category || '').toLowerCase().includes(lower) ||
        (p.github_description || '').toLowerCase().includes(lower)
    )
  }

  // 合并本地互动计数
  const db = getDB()
  const countStmt = db.prepare(
    'SELECT' +
    ' (SELECT COUNT(*) FROM favorites WHERE plugin_id = ?) AS favorites,' +
    ' (SELECT COUNT(*) FROM comments  WHERE plugin_id = ? AND deleted = 0) AS comments,' +
    ' (SELECT COUNT(*) FROM shares    WHERE plugin_id = ?) AS shares,' +
    ' (SELECT COUNT(*) FROM feedback  WHERE plugin_id = ?) AS feedback'
  )

  const enriched = plugins.map((p) => {
    const counts = countStmt.get(p.id, p.id, p.id, p.id) as {
      favorites: number
      comments: number
      shares: number
      feedback: number
    }
    return {
      ...p,
      favorite_count: counts.favorites,
      comment_count: counts.comments,
      share_count: counts.shares,
      feedback_count: counts.feedback,
      hot_score: counts.favorites * 3 + counts.comments * 2 + counts.shares + counts.feedback,
    }
  })

  // 排序
  if (sort === 'name') {
    enriched.sort((a, b) => a.name.localeCompare(b.name, 'zh-CN'))
  } else if (sort === 'hot') {
    enriched.sort((a, b) => b.hot_score - a.hot_score || b.stars - a.stars)
  } else if (sort === 'latest') {
    enriched.sort((a, b) => {
      const ta = a.pushed_at ? new Date(a.pushed_at).getTime() : 0
      const tb = b.pushed_at ? new Date(b.pushed_at).getTime() : 0
      return tb - ta
    })
  } else {
    enriched.sort((a, b) => b.stars - a.stars)
  }

  // 分页
  const total = enriched.length
  const start = (page - 1) * pageSize
  let paged = enriched.slice(start, start + pageSize)

  // 字段裁剪：basic 模式去掉长描述，减小负载
  if (fields === 'basic') {
    paged = paged.map((p) => ({
      id: p.id,
      repo: p.repo,
      name: p.name,
      category: p.category,
      type: p.type,
      stars: p.stars,
      github_url: p.github_url,
      pushed_at: p.pushed_at,
      favorite_count: p.favorite_count,
      comment_count: p.comment_count,
      share_count: p.share_count,
      feedback_count: p.feedback_count,
      hot_score: p.hot_score,
    }))
  }

  // ETag：基于数据变化（总数 + 最新更新时间），支持增量拉取
  const latestPush = enriched
    .map((p) => p.pushed_at)
    .filter(Boolean)
    .sort()
    .reverse()[0]
  const etag = '"' + total + '-' + (latestPush || 'none') + '"'
  setHeader(event, 'ETag', etag)
  setHeader(event, 'Cache-Control', 'public, max-age=600')

  // If-None-Match：命中返回 304，节省带宽
  const inm = getHeader(event, 'if-none-match')
  if (inm && inm === etag) {
    setResponseStatus(event, 304)
    return null
  }

  return {
    total,
    page,
    page_size: pageSize,
    sort,
    updated_at: new Date().toISOString(),
    plugins: paged,
  }
})
