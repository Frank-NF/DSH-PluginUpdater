/**
 * GET /api/stats
 * 站点汇总统计（插件总数 / 总 Star / 最近更新时间）
 */
export default defineEventHandler(async () => {
  const plugins = await getPlugins()

  const totalStars = plugins.reduce((sum, p) => sum + p.stars, 0)
  const latestPush = plugins
    .map((p) => p.pushed_at)
    .filter(Boolean)
    .sort()
    .reverse()[0] as string | undefined

  return {
    total_plugins: plugins.length,
    total_stars: totalStars,
    latest_push: latestPush ?? null,
    github_data_ok: plugins.some((p) => p.fetched),
  }
})
