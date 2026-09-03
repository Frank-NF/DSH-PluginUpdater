/**
 * MCP 面板 API（V2 §8 P1）：dsh-mcp.json 读写合并、env 密值加密存储、探活、启停。
 * 仅桌面端可用；浏览器预览时所有方法拒绝由调用方兜底。
 */
import { invoke } from '@tauri-apps/api/core'
import type { McpListResult, McpProbeResult } from '../types'

const isTauri =
  typeof window !== 'undefined' &&
  ('__TAURI_INTERNALS__' in window || '__TAURI__' in window)

function requireTauri<T>(): never {
  throw new Error('MCP 面板需要桌面端')
}

export const mcpApi = isTauri
  ? {
      list: (): Promise<McpListResult> => invoke('mcp_list'),
      saveEnv: (serverId: string, key: string, value: string): Promise<boolean> =>
        invoke('mcp_save_env', { serverId, key, value }),
      applyConfig: (): Promise<number> => invoke('mcp_apply_config'),
      probe: (serverId: string): Promise<McpProbeResult> => invoke('mcp_probe', { serverId }),
      toggle: (serverId: string, enable: boolean): Promise<boolean> =>
        invoke('mcp_toggle', { serverId, enable }),
    }
  : {
      list: (): Promise<McpListResult> => requireTauri(),
      saveEnv: (): Promise<boolean> => requireTauri(),
      applyConfig: (): Promise<number> => requireTauri(),
      probe: (): Promise<McpProbeResult> => requireTauri(),
      toggle: (): Promise<boolean> => requireTauri(),
    }
