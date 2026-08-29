/**
 * POST /api/auth/register
 * 邮箱注册：{ email, password, display_name }
 */
export default defineEventHandler(async (event) => {
  const body = await readBody<{
    email?: string
    password?: string
    display_name?: string
  }>(event)

  const email = body.email?.trim().toLowerCase()
  const password = body.password || ''
  const displayName = body.display_name?.trim() || email?.split('@')[0]

  if (!email || !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)) {
    throw createError({ statusCode: 400, statusMessage: '邮箱格式不正确' })
  }
  if (password.length < 8) {
    throw createError({ statusCode: 400, statusMessage: '密码至少 8 位' })
  }
  if (!displayName || displayName.length > 30) {
    throw createError({ statusCode: 400, statusMessage: '昵称需 1-30 个字符' })
  }

  const db = getDB()
  const exists = db.prepare('SELECT id FROM users WHERE email = ?').get(email)
  if (exists) {
    throw createError({ statusCode: 409, statusMessage: '该邮箱已注册，请直接登录' })
  }

  const result = db
    .prepare(
      'INSERT INTO users (email, password_hash, display_name, last_login_at) VALUES (?, ?, ?, datetime(\'now\'))'
    )
    .run(email, hashPassword(password), displayName)

  const user: AuthUser = {
    id: Number(result.lastInsertRowid),
    email,
    display_name: displayName,
    avatar_url: null,
    role: 'user',
  }

  setCookie(event, 'dsh_token', signToken(user), {
    httpOnly: true,
    sameSite: 'lax',
    maxAge: 7 * 24 * 3600,
    path: '/',
  })

  return { user }
})
