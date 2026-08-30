import { ref } from 'vue'

/**
 * 全局确认框（WeUI Dialog）
 * 以 Promise 方式调用：const ok = await confirm({ title, message })
 * 宿主组件：<WConfirmHost /> 挂在 App 根节点即可。
 */

export interface ConfirmOptions {
  /** 标题 */
  title: string
  /** 正文（支持 \n 换行） */
  message?: string
  /** 确认按钮文案 */
  confirmText?: string
  /** 取消按钮文案 */
  cancelText?: string
  /** 语义：default 普通 / warn 警告 / danger 危险 */
  type?: 'default' | 'warn' | 'danger'
  /** 点击遮罩是否关闭（默认 false，防误关） */
  closeOnMask?: boolean
}

interface ConfirmState extends ConfirmOptions {
  visible: boolean
  resolve?: (value: boolean) => void
}

const state = ref<ConfirmState | null>(null)

function confirm(options: ConfirmOptions): Promise<boolean> {
  // 若已有弹窗在等待，先以 false 结算，避免 Promise 悬挂
  state.value?.resolve?.(false)
  return new Promise<boolean>((resolve) => {
    state.value = { ...options, visible: true, resolve }
  })
}

function settle(value: boolean) {
  const s = state.value
  if (!s) return
  s.resolve?.(value)
  state.value = null
}

export function useConfirm() {
  return {
    state,
    confirm,
    /** 用户在弹窗上点了确定 */
    accept: () => settle(true),
    /** 用户在弹窗上点了取消 / 关闭 */
    cancel: () => settle(false),
  }
}
