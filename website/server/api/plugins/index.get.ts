/**
 * GET /api/plugins
 * 返回插件市场数据（GitHub 实时数据 + 注册表合并，10 分钟缓存）
 * 并合并每个插件的收藏数 / 评论数 / 分享数（来自本地 SQLite）
 *
 * 查询参数：
 * - sort=stars|name|hot   排序（默认 stars；hot = 收藏+评论+分享热度）
 * - category=xxx          按分类筛选
 */
export default defineEventHandler(async (event) => {
  const query = getQuery(event)
  const sort = (query.sort as string) || 'stars'
  const category = query.category as string | undefined

  let plugins = await getPlugins()

  if (category && category !== '全部') {
    plugins = plugins.filter((p) => p.category === category)
  }

  // 合并本地互动计数
  const db = getDB()
  const countStmt = db.prepare(
    `SELECT
       (SELECT COUNT(*) FROM favorites WHERE plugin_id = ?) AS favorites,
       (SELECT COUNT(*) FROM comments  WHERE plugin_id = ? AND deleted = 0) AS comments,
       (SELECT COUNT(*) FROM shares    WHERE plugin_id = ?) AS shares`
  )

  const enriched = plugins.map((p) => {
    const counts = countStmt.get(p.id, p.id, p.id) as {
      favorites: number
      comments: number
      shares: number
    }
    return {
      ...p,
      favorite_count: counts.favorites,
      comment_count: counts.comments,
      share_count: counts.shares,
      hot_score: counts.favorites * 3 + counts.comments * 2 + counts.shares,
    }
  })

  if (sort === 'name') {
    enriched.sort((a, b) => a.name.localeCompare(b.name, 'zh-CN'))
  } else if (sort === 'hot') {
    enriched.sort((a, b) => b.hot_score - a.hot_score || b.stars - a.stars)
  } else {
    enriched.sort((a, b) => b.stars - a.stars)
  }

  return {
    total: enriched.length,
    updated_at: new Date().toISOString(),
    plugins: enriched,
  }
})
