import { invoke } from '@tauri-apps/api/core'
import type { OfflinePackSummary, SnapshotApplyItem, SnapshotDiff, SnapshotSummary } from '../types'

function requireTauri(): void {
  if (!('__TAURI_INTERNALS__' in window)) {
    throw new Error('快照与离线打包功能仅在桌面客户端可用')
  }
}

/** 弹出系统文件选择框（filters 形如 ['json','zip']） */
export async function pickFile(title: string, filters: string[]): Promise<string | null> {
  requireTauri()
  return invoke<string | null>('pick_file', { title, filters })
}

/** 弹出系统「另存为」对话框（导出快照/离线包） */
export async function pickSaveFile(title: string, fileName: string, filters: string[]): Promise<string | null> {
  requireTauri()
  return invoke<string | null>('pick_save_file', { title, fileName, filters })
}

export async function snapshotExport(path: string): Promise<SnapshotSummary> {
  requireTauri()
  return invoke<SnapshotSummary>('snapshot_export', { path })
}

export async function snapshotPreview(path: string): Promise<SnapshotDiff> {
  requireTauri()
  return invoke<SnapshotDiff>('snapshot_preview', { path })
}

export async function snapshotApply(path: string): Promise<SnapshotApplyItem[]> {
  requireTauri()
  return invoke<SnapshotApplyItem[]>('snapshot_apply', { path })
}

export async function offlinePack(path: string): Promise<OfflinePackSummary> {
  requireTauri()
  return invoke<OfflinePackSummary>('offline_pack', { path })
}

export async function offlineApply(path: string): Promise<number> {
  requireTauri()
  return invoke<number>('offline_apply', { path })
}
