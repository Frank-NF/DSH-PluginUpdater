/**
 * POST /api/compat/check
 * Admin 维护兼容规则：{ plugin_id, dsh_version, compatible, note }
 */
export default defineEventHandler(async (event) => {
  const user = getAuthUser(event)
  if (!user || user.role !== 'admin') {
    throw createError({ statusCode: 403, statusMessage: '需要管理员权限' })
  }
  const db = getDB()
  const body = await readBody<{ plugin_id?: string; dsh_version?: string; compatible?: boolean; note?: string }>(event)
  const pluginId = body.plugin_id?.trim()
  const dshVersion = body.dsh_version?.trim() || '*'
  if (!pluginId) {
    throw createError({ statusCode: 400, statusMessage: '缺少 plugin_id' })
  }
  db.prepare(
    `INSERT INTO plugin_compat (plugin_id, dsh_version, compatible, note, updated_at)
     VALUES (?, ?, ?, ?, datetime('now'))
     ON CONFLICT(plugin_id, dsh_version) DO UPDATE SET
       compatible = excluded.compatible,
       note = excluded.note,
       updated_at = datetime('now')`
  ).run(pluginId, dshVersion, body.compatible === false ? 0 : 1, body.note?.trim() || null)
  return { ok: true }
})
