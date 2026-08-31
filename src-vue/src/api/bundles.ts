/**
 * 组合包（Bundle）API：V2 §3 事务安装
 * - Tauri 桌面端：invoke 后端命令，进度经 bundle_progress 事件（按 taskId 路由）
 * - 纯浏览器预览：列表走官网真实 API，预检/安装需要桌面端
 */
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import type {
  BundleDef,
  BundlePreview,
  BundleInstallResult,
  BundleProgress,
} from '../types'

const isTauri =
  typeof window !== 'undefined' &&
  ('__TAURI_INTERNALS__' in window || '__TAURI__' in window)

const WEBSITE_BASE = 'https://dsh.huilinsh.cn'

export const bundleApi = isTauri
  ? {
      listBundles: (): Promise<BundleDef[]> => invoke('list_bundles'),

      previewBundle: (id: string): Promise<BundlePreview> =>
        invoke('preview_bundle', { id }),

      installBundle: (id: string): Promise<BundleInstallResult> =>
        invoke('install_bundle', { id }),

      isCancelled: (taskId: string): Promise<boolean> =>
        invoke('is_cancelled', { taskId }),

      cancelInstall: (taskId: string): Promise<boolean> =>
        invoke('cancel_bundle_install', { taskId }),

      onBundleProgress: (callback: (p: BundleProgress) => void) =>
        listen<BundleProgress>('bundle_progress', (event) => {
          callback(event.payload)
        }),
    }
  : {
      listBundles: async (): Promise<BundleDef[]> => {
        try {
          const res = await fetch(`${WEBSITE_BASE}/api/bundles`)
          if (!res.ok) throw new Error(String(res.status))
          const data = await res.json()
          return (data.bundles ?? []) as BundleDef[]
        } catch {
          return []
        }
      },

      previewBundle: async (): Promise<BundlePreview> => {
        throw new Error('浏览器预览模式不支持组合包预检，请在桌面端使用')
      },

      installBundle: async (): Promise<BundleInstallResult> => {
        throw new Error('浏览器预览模式不支持组合包安装，请在桌面端使用')
      },

      isCancelled: async (): Promise<boolean> => false,

      cancelInstall: async (): Promise<boolean> => false,

      onBundleProgress: async (): Promise<() => void> => () => {},
    }
