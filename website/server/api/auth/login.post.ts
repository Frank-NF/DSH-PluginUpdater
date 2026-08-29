/**
 * POST /api/auth/login
 * 邮箱登录：{ email, password }
 */
export default defineEventHandler(async (event) => {
  const body = await readBody<{ email?: string; password?: string }>(event)

  const email = body.email?.trim().toLowerCase()
  const password = body.password || ''

  if (!email || !password) {
    throw createError({ statusCode: 400, statusMessage: '请输入邮箱和密码' })
  }

  const db = getDB()
  const row = db
    .prepare('SELECT * FROM users WHERE email = ? AND password_hash IS NOT NULL')
    .get(email) as
    | { id: number; email: string; display_name: string; avatar_url: string | null; role: string; password_hash: string }
    | undefined

  if (!row || !checkPassword(password, row.password_hash)) {
    throw createError({ statusCode: 401, statusMessage: '邮箱或密码错误' })
  }

  db.prepare('UPDATE users SET last_login_at = datetime(\'now\') WHERE id = ?').run(row.id)

  const user: AuthUser = {
    id: row.id,
    email: row.email,
    display_name: row.display_name,
    avatar_url: row.avatar_url,
    role: row.role,
  }

  setCookie(event, 'dsh_token', signToken(user), {
    httpOnly: true,
    sameSite: 'lax',
    maxAge: 7 * 24 * 3600,
    path: '/',
  })

  return { user }
})
