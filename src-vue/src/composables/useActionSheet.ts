import { ref } from 'vue'

/** 动作项（与 WeUI Actionsheet 对应） */
export interface SheetItem {
  label: string
  value: string
  /** warn = 危险操作（标红） */
  type?: 'default' | 'warn'
  /** 次要说明 */
  desc?: string
}

export interface ActionSheetOptions {
  title?: string
  items: SheetItem[]
  cancelText?: string
  /** 正文说明（显示在标题下方） */
  message?: string
}

interface ActionState extends ActionSheetOptions {
  visible: boolean
  resolve?: (value: string | null) => void
}

const state = ref<ActionState | null>(null)

/** 打开动作面板，返回所选 value；取消返回 null */
function actionSheet(options: ActionSheetOptions): Promise<string | null> {
  // 已有等待中的面板先结算，避免 Promise 悬挂
  state.value?.resolve?.(null)
  return new Promise<string | null>((resolve) => {
    state.value = { ...options, visible: true, resolve }
  })
}

function settle(value: string | null) {
  const s = state.value
  if (!s) return
  s.resolve?.(value)
  state.value = null
}

export function useActionSheet() {
  return {
    state,
    actionSheet,
    /** 选中某项 */
    pick: (value: string) => settle(value),
    /** 取消 / 关闭 */
    cancel: () => settle(null),
  }
}
