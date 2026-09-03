import { ref } from 'vue'

/**
 * 全局轻提示（WeUI Toast）
 * 单例：模块级状态，任意组件调用同一份队列
 */

export type ToastType = 'success' | 'warn' | 'error' | 'loading' | 'text'

export interface ToastItem {
  id: number
  type: ToastType
  text: string
  /** 持续时长（毫秒）；0 = 不自动关闭（用于 loading，需手动 hide） */
  duration: number
}

const toasts = ref<ToastItem[]>([])
let seed = 0
const timers = new Map<number, ReturnType<typeof setTimeout>>()

function push(item: Omit<ToastItem, 'id'>): number {
  const id = ++seed
  toasts.value.push({ ...item, id })

  if (item.duration > 0) {
    timers.set(
      id,
      setTimeout(() => hide(id), item.duration)
    )
  }
  // 最多同时保留 3 条，超出移除最早的
  while (toasts.value.length > 3) {
    const first = toasts.value[0]
    if (first) hide(first.id)
    else break
  }
  return id
}

function hide(id: number) {
  const timer = timers.get(id)
  if (timer) {
    clearTimeout(timer)
    timers.delete(id)
  }
  const idx = toasts.value.findIndex((t) => t.id === id)
  if (idx !== -1) toasts.value.splice(idx, 1)
}

function clear() {
  timers.forEach((t) => clearTimeout(t))
  timers.clear()
  toasts.value = []
}

/** 成功：默认 2s */
function success(text: string, duration = 2000) {
  return push({ type: 'success', text, duration })
}

/** 警告：默认 2.4s */
function warn(text: string, duration = 2400) {
  return push({ type: 'warn', text, duration })
}

/** 错误：默认 3s */
function error(text: string, duration = 3000) {
  return push({ type: 'error', text, duration })
}

/** 纯文本提示：默认 2s */
function text(msg: string, duration = 2000) {
  return push({ type: 'text', text: msg, duration })
}

/** 加载中：不会自动关闭，返回 id，需手动 hide */
function loading(msg: string): number {
  return push({ type: 'loading', text: msg, duration: 0 })
}

export function useToast() {
  return {
    toasts,
    success,
    warn,
    error,
    text,
    info: text,
    loading,
    hide,
    clear,
  }
}
