import { ref } from 'vue'
import { getVersion } from '@tauri-apps/api/app'

const cached = ref('')

let started = false

/**
 * 应用自身版本号（Tauri 环境；浏览器预览返回空串，调用方据此隐藏徽章）。
 * 需要 capabilities 授权 core:default（含 core:app:default → allow-version）。
 */
export function useAppVersion() {
  if (!started) {
    started = true
    getVersion()
      .then((v) => {
        cached.value = v
      })
      .catch(() => {
        /* 未授权或非 Tauri 环境：留空，调用方隐藏 */
      })
  }
  return { appVersion: cached }
}
