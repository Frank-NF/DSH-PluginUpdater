/**
 * GET /api/plugins/:id
 * 返回单个插件详情（GitHub 实时数据）
 */
export default defineEventHandler(async (event) => {
  const id = getRouterParam(event, 'id')
  const plugins = await getPlugins()
  const plugin = plugins.find((p) => p.id === id)

  if (!plugin) {
    throw createError({
      statusCode: 404,
      statusMessage: 'Plugin Not Found',
    })
  }

  return plugin
})
