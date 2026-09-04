import { createApp } from 'vue'
import { createPinia } from 'pinia'

// WeUI 官方样式（package main = dist/style/weui.css）
import 'weui'

import App from './App.vue'
// 业务样式必须在 weui 之后加载，才能覆盖其变量与定位
import './styles/main.css'
import { useTheme } from './composables/useTheme'

// 尽早应用主题，避免首屏闪烁
const { applyTheme, theme } = useTheme()
applyTheme(theme.value)

// 全局错误捕获
if (typeof window !== 'undefined') {
  window.addEventListener('error', (e) => {
    console.error('[Global Error]', e.error)
  })
  window.addEventListener('unhandledrejection', (e) => {
    console.error('[Unhandled Rejection]', e.reason)
  })
}

const app = createApp(App)
app.use(createPinia())
app.mount('#app')
console.log('[DSH] App mounted')
