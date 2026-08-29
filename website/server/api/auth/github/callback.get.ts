/**
 * GET /api/auth/github/callback
 * GitHub 授权回调：code 换 access_token → 拉用户信息 → upsert 本地用户 → 发 JWT
 */
export default defineEventHandler(async (event) => {
  const config = useRuntimeConfig()
  const clientId = config.public.githubClientId
  const clientSecret = config.githubClientSecret

  const query = getQuery(event)
  const code = query.code as string | undefined
  const state = query.state as string | undefined
  const savedState = getCookie(event, 'dsh_oauth_state')

  if (!code || !state || !savedState || state !== savedState) {
    throw createError({ statusCode: 400, statusMessage: '授权状态校验失败，请重试' })
  }
  if (!clientId || !clientSecret) {
    throw createError({ statusCode: 503, statusMessage: 'GitHub 登录未配置' })
  }

  // 1. code 换 access_token
  const tokenRes = await $fetch<any>('https://github.com/login/oauth/access_token', {
    method: 'POST',
    body: { client_id: clientId, client_secret: clientSecret, code },
    headers: { Accept: 'application/json' },
  })
  const accessToken = tokenRes?.access_token
  if (!accessToken) {
    throw createError({ statusCode: 401, statusMessage: 'GitHub 授权失败' })
  }

  // 2. 拉 GitHub 用户信息
  const ghUser = await $fetch<any>('https://api.github.com/user', {
    headers: { Authorization: `Bearer ${accessToken}`, 'User-Agent': 'DSH-Website' },
  })
  const emails = await $fetch<any[]>('https://api.github.com/user/emails', {
    headers: { Authorization: `Bearer ${accessToken}`, 'User-Agent': 'DSH-Website' },
  }).catch(() => [])
  const primaryEmail =
    emails.find((e) => e.primary && e.verified)?.email || null

  const githubId = String(ghUser.id)
  const displayName = ghUser.name || ghUser.login
  const avatarUrl = ghUser.avatar_url

  // 3. upsert 本地用户
  const db = getDB()
  let userId: number

  const existing = db
    .prepare('SELECT id FROM users WHERE github_id = ?')
    .get(githubId) as { id: number } | undefined

  if (existing) {
    db.prepare(
      'UPDATE users SET github_login = ?, display_name = ?, avatar_url = ?, last_login_at = datetime(\'now\') WHERE id = ?'
    ).run(ghUser.login, displayName, avatarUrl, existing.id)
    userId = existing.id
  } else {
    // 邮箱可能与已注册账号冲突 → 合并到该账号
    let mergedId: number | undefined
    if (primaryEmail) {
      const byEmail = db
        .prepare('SELECT id FROM users WHERE email = ?')
        .get(primaryEmail) as { id: number } | undefined
      mergedId = byEmail?.id
    }
    if (mergedId) {
      db.prepare(
        'UPDATE users SET github_id = ?, github_login = ?, avatar_url = ?, last_login_at = datetime(\'now\') WHERE id = ?'
      ).run(githubId, ghUser.login, avatarUrl, mergedId)
      userId = mergedId
    } else {
      const result = db
        .prepare(
          'INSERT INTO users (email, github_id, github_login, display_name, avatar_url, last_login_at) VALUES (?, ?, ?, ?, ?, datetime(\'now\'))'
        )
        .run(primaryEmail, githubId, ghUser.login, displayName, avatarUrl)
      userId = Number(result.lastInsertRowid)
    }
  }

  // 4. 发 JWT + 跳回首页
  const user: AuthUser = {
    id: userId,
    email: primaryEmail,
    display_name: displayName,
    avatar_url: avatarUrl,
    role: 'user',
  }

  setCookie(event, 'dsh_token', signToken(user), {
    httpOnly: true,
    sameSite: 'lax',
    maxAge: 7 * 24 * 3600,
    path: '/',
  })
  deleteCookie(event, 'dsh_oauth_state')

  return sendRedirect(event, '/?login=success')
})
