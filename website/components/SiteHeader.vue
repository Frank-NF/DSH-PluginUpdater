<template>
  <header class="site-header glass">
    <div class="container header-inner">
      <NuxtLink to="/" class="logo">
        <div class="logo-mark">DSH</div>
        <div class="logo-text">
          <span class="logo-title">插件升级管理<span class="logo-version">v{{ appVersion }}</span></span>
          <span class="logo-sub">插件市场 · 官方网站</span>
        </div>
      </NuxtLink>

      <nav class="nav-links">
        <NuxtLink to="/" class="nav-link" active-class="active">首页</NuxtLink>
        <NuxtLink to="/plugins" class="nav-link" active-class="active">插件市场</NuxtLink>
        <NuxtLink to="/download" class="nav-link" active-class="active">下载</NuxtLink>
        <NuxtLink to="/docs" class="nav-link" active-class="active">文档</NuxtLink>
      </nav>

      <div class="header-actions">
        <a
          href="http://64.90.30.139:8071/"
          target="_blank"
          rel="noopener"
          class="btn btn-outline btn-sm"
        >
          <span class="online-dot" aria-hidden="true"></span>
          在线版
        </a>
        <NuxtLink to="/download" class="btn btn-primary btn-sm download-btn">下载客户端</NuxtLink>

        <!-- 未登录 -->
        <button v-if="!user" class="btn btn-outline btn-sm login-btn" @click="openAuthDialog('login')">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
            <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" />
            <circle cx="12" cy="7" r="4" />
          </svg>
          登录
        </button>

        <!-- 已登录：用户菜单 -->
        <div v-else class="user-wrap" ref="userWrap">
          <button class="user-btn" aria-label="用户菜单" @click="menuOpen = !menuOpen">
            <img v-if="user.avatar_url" :src="user.avatar_url" :alt="user.display_name" class="user-avatar" referrerpolicy="no-referrer" />
            <span v-else class="user-avatar user-avatar-fallback">{{ avatarLetter }}</span>
            <span class="user-name">{{ user.display_name }}</span>
            <svg class="chev" :class="{ open: menuOpen }" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" aria-hidden="true">
              <path d="m6 9 6 6 6-6" />
            </svg>
          </button>

          <Transition name="menu-fade">
            <div v-if="menuOpen" class="user-menu glass">
              <div class="menu-head">
                <div class="menu-name">{{ user.display_name }}</div>
                <div class="menu-email">{{ user.email || 'GitHub 登录' }}</div>
              </div>
              <NuxtLink to="/plugins?tab=favorites" class="menu-item" @click="menuOpen = false">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><path d="m12 2 3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/></svg>
                我的收藏
              </NuxtLink>
              <NuxtLink v-if="user.role === 'admin'" to="/feedback" class="menu-item" @click="menuOpen = false">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/><path d="M8 9h8M8 13h5"/></svg>
                反馈管理
              </NuxtLink>
              <button class="menu-item" @click="handleLogout">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/><path d="m16 17 5-5-5-5"/><path d="M21 12H9"/></svg>
                退出登录
              </button>
            </div>
          </Transition>
        </div>
      </div>
    </div>
  </header>
</template>

<script setup lang="ts">
import { version as appVersion } from '~/package.json'

const { user, logout, openAuthDialog } = useAuth()

const menuOpen = ref(false)
const userWrap = ref<HTMLElement | null>(null)

const avatarLetter = computed(() => user.value?.display_name?.charAt(0).toUpperCase() || '?')

async function handleLogout() {
  menuOpen.value = false
  await logout()
}

function onDocClick(e: MouseEvent) {
  if (userWrap.value && !userWrap.value.contains(e.target as Node)) {
    menuOpen.value = false
  }
}

onMounted(() => document.addEventListener('click', onDocClick))
onUnmounted(() => document.removeEventListener('click', onDocClick))
</script>

<style scoped>
.site-header {
  position: sticky;
  top: 0;
  z-index: 100;
}

.header-inner {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 24px;
  height: 64px;
}

.logo {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-shrink: 0;
}

.logo-mark {
  width: 38px;
  height: 38px;
  border-radius: 11px;
  background: linear-gradient(135deg, var(--primary), var(--primary-light));
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 700;
  font-size: 12px;
  color: #fff;
  letter-spacing: 0.3px;
  box-shadow: 0 4px 14px rgba(99, 102, 241, 0.4);
}

.logo-text {
  display: flex;
  flex-direction: column;
  line-height: 1.25;
}

.logo-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.logo-version {
  display: inline-block;
  margin-left: 6px;
  padding: 1px 7px;
  border-radius: 999px;
  font-size: 10px;
  font-weight: 600;
  line-height: 1.5;
  vertical-align: 1px;
  color: var(--primary-light);
  background: rgba(99, 102, 241, 0.12);
}

.logo-sub {
  font-size: 11px;
  color: var(--text-muted);
}

.nav-links {
  display: flex;
  align-items: center;
  gap: 28px;
}

.nav-link {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-secondary);
  transition: color var(--dur) var(--ease);
  position: relative;
}

.nav-link:hover {
  color: var(--text-primary);
}

.nav-link.active {
  color: var(--primary-light);
}

.nav-link.active::after {
  content: '';
  position: absolute;
  left: 0;
  right: 0;
  bottom: -6px;
  height: 2px;
  border-radius: 1px;
  background: linear-gradient(90deg, var(--primary), var(--primary-light));
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 10px;
  flex-shrink: 0;
}

.online-dot {
  width: 7px;
  height: 7px;
  border-radius: 50%;
  background: var(--accent);
  box-shadow: 0 0 6px rgba(16, 185, 129, 0.8);
}

/* ---------- 用户菜单 ---------- */
.user-wrap {
  position: relative;
}

.user-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 5px 10px 5px 5px;
  border-radius: 24px;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid var(--glass-border);
  cursor: pointer;
  color: var(--text-primary);
  transition: all var(--dur) var(--ease);
}

.user-btn:hover {
  background: rgba(255, 255, 255, 0.09);
  border-color: rgba(99, 102, 241, 0.4);
}

.user-avatar {
  width: 28px;
  height: 28px;
  border-radius: 50%;
  object-fit: cover;
}

.user-avatar-fallback {
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, var(--primary), var(--primary-light));
  color: #fff;
  font-size: 13px;
  font-weight: 700;
}

.user-name {
  font-size: 13px;
  font-weight: 500;
  max-width: 90px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.chev {
  color: var(--text-muted);
  transition: transform var(--dur) var(--ease);
}
.chev.open {
  transform: rotate(180deg);
}

.user-menu {
  position: absolute;
  top: calc(100% + 10px);
  right: 0;
  width: 220px;
  padding: 8px;
  border-radius: var(--radius-lg);
  background: var(--bg-secondary);
  box-shadow: 0 12px 40px rgba(0, 0, 0, 0.5);
}

.menu-fade-enter-active,
.menu-fade-leave-active {
  transition: all 0.18s var(--ease);
}
.menu-fade-enter-from,
.menu-fade-leave-to {
  opacity: 0;
  transform: translateY(-6px);
}

.menu-head {
  padding: 10px 12px;
  border-bottom: 1px solid var(--glass-border);
  margin-bottom: 6px;
}

.menu-name {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.menu-email {
  font-size: 12px;
  color: var(--text-muted);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.menu-item {
  display: flex;
  align-items: center;
  gap: 10px;
  width: 100%;
  padding: 9px 12px;
  border-radius: var(--radius-md);
  background: none;
  border: none;
  color: var(--text-secondary);
  font-size: 13px;
  cursor: pointer;
  text-align: left;
  transition: all var(--dur) var(--ease);
}

.menu-item:hover {
  background: rgba(99, 102, 241, 0.12);
  color: var(--text-primary);
}

@media (max-width: 900px) {
  .nav-links {
    gap: 16px;
  }
  .logo-sub,
  .header-actions .btn-outline,
  .user-name,
  .chev {
    display: none;
  }
}

@media (max-width: 640px) {
  .nav-links {
    display: none;
  }
  .download-btn {
    display: none;
  }
}
</style>
