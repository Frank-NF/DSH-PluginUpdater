import { ref, watch } from 'vue'

/**
 * 主题：复用 WeUI 原生主题机制（data-weui-theme / data-weui-mode）
 * - light / dark：强制
 * - auto：跟随系统（WeUI 的 prefers-color-scheme 规则自动生效）
 * 无需自行维护深色变量表，完全走 WeUI 官方主题通道。
 */
export type ThemeMode = 'light' | 'dark' | 'auto'

const THEME_KEY = 'dsh-updater-theme'

function loadTheme(): ThemeMode {
  const saved = localStorage.getItem(THEME_KEY)
  return saved === 'light' || saved === 'dark' || saved === 'auto' ? saved : 'dark'
}

const theme = ref<ThemeMode>(loadTheme())

function applyTheme(mode: ThemeMode) {
  if (typeof document === 'undefined') return
  const body = document.body
  // care = WeUI 官方「关注系统主题」标记
  body.setAttribute('data-weui-mode', 'care')
  if (mode === 'auto') {
    body.removeAttribute('data-weui-theme')
  } else {
    body.setAttribute('data-weui-theme', mode)
  }
  // 同步给自定义样式使用（如背景光晕配色）
  document.documentElement.setAttribute('data-theme', mode)
}

watch(theme, applyTheme, { immediate: true })

export function useTheme() {
  function setTheme(mode: ThemeMode) {
    theme.value = mode
    localStorage.setItem(THEME_KEY, mode)
  }

  function toggleTheme() {
    // 在 light / dark 间切换；auto 时按当前系统表现取反
    const current = theme.value
    if (current === 'auto') {
      const sysDark =
        typeof window !== 'undefined' &&
        window.matchMedia?.('(prefers-color-scheme: dark)').matches
      setTheme(sysDark ? 'light' : 'dark')
    } else {
      setTheme(current === 'dark' ? 'light' : 'dark')
    }
  }

  return { theme, setTheme, toggleTheme, applyTheme }
}
