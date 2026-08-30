import { gsap } from 'gsap'

/**
 * 动效基础层（GSAP）
 * 设计原则：
 * 1. 克制 —— 只动 transform / opacity，时长控制在 0.12~0.38s
 * 2. 统一 —— 所有动效走这里，便于全局降级与调参
 * 3. 可降级 —— 尊重系统「减少动态效果」，降级时直接落到终态，不做补间
 */

/** 系统是否要求减少动效（WCAG 2.3.3 / prefers-reduced-motion） */
export function prefersReducedMotion(): boolean {
  if (typeof window === 'undefined' || typeof window.matchMedia !== 'function') return false
  try {
    return window.matchMedia('(prefers-reduced-motion: reduce)').matches
  } catch {
    return false
  }
}

/** 动效时长（秒） */
export const DUR = {
  tap: 0.12, // 按钮按下反馈
  fast: 0.18, // 小元素进出
  base: 0.26, // 常规过渡
  slow: 0.38, // 页面 / 弹窗
} as const

/** 缓动曲线 */
export const EASE = {
  out: 'power2.out',
  inOut: 'power2.inOut',
  soft: 'power1.out',
  back: 'back.out(1.5)',
} as const

type Targets = Element | Element[] | NodeListOf<Element> | string | null | undefined

/** 归一化目标，过滤空值 */
function norm(t: Targets, root?: Element | null): Element[] {
  const list: Element[] = []
  const push = (v: unknown) => {
    if (!v) return
    if (typeof v === 'string') {
      Array.from((root ?? document).querySelectorAll(v)).forEach((el) => list.push(el))
    } else if (v instanceof Element) {
      list.push(v)
    } else if (Array.isArray(v)) {
      v.forEach(push)
    } else if (v instanceof NodeList) {
      Array.from(v).forEach(push)
    } else if ((v as { $el?: Element })?.$el instanceof Element) {
      // Vue 组件实例
      list.push((v as { $el: Element }).$el)
    }
  }
  push(t)
  return list
}

/** 降级时把元素还原为「无内联样式」的终态 */
function settle(els: Element[]) {
  if (els.length) gsap.set(els, { clearProps: 'all' })
}

/**
 * 进场：淡入 + 轻微上移（列表/卡片/区块通用）
 * stagger > 0 时做交错进场
 */
export function fadeSlideIn(
  target: Targets,
  opts: {
    y?: number
    delay?: number
    duration?: number
    stagger?: number
    root?: Element | null
    onComplete?: () => void
  } = {}
) {
  const els = norm(target, opts.root)
  if (!els.length) return null
  if (prefersReducedMotion()) {
    settle(els)
    opts.onComplete?.()
    return null
  }
  return gsap.fromTo(
    els,
    { opacity: 0, y: opts.y ?? 10 },
    {
      opacity: 1,
      y: 0,
      duration: opts.duration ?? DUR.base,
      delay: opts.delay ?? 0,
      stagger: opts.stagger ?? 0,
      ease: EASE.out,
      clearProps: 'transform,opacity',
      onComplete: opts.onComplete,
    }
  )
}

/** 页面 / 面板切换：横向淡入（用于 Tab 切换） */
export function panelIn(
  target: Targets,
  opts: { x?: number; duration?: number; root?: Element | null } = {}
) {
  const els = norm(target, opts.root)
  if (!els.length) return null
  if (prefersReducedMotion()) {
    settle(els)
    return null
  }
  return gsap.fromTo(
    els,
    { opacity: 0, x: opts.x ?? 12 },
    {
      opacity: 1,
      x: 0,
      duration: opts.duration ?? DUR.base,
      ease: EASE.out,
      clearProps: 'transform,opacity',
    }
  )
}

/** 列表交错进场（插件卡片网格） */
export function staggerIn(
  target: Targets,
  opts: { y?: number; stagger?: number; duration?: number; root?: Element | null } = {}
) {
  return fadeSlideIn(target, {
    y: opts.y ?? 14,
    stagger: opts.stagger ?? 0.035,
    duration: opts.duration ?? DUR.slow,
    root: opts.root,
  })
}

/** 按钮按下反馈：轻微缩放 */
export function pressIn(el: Element | null | undefined) {
  if (!el || prefersReducedMotion()) return
  gsap.to(el, { scale: 0.96, duration: DUR.tap, ease: EASE.out })
}

/** 按钮松开回弹 */
export function pressOut(el: Element | null | undefined) {
  if (!el || prefersReducedMotion()) return
  gsap.to(el, { scale: 1, duration: DUR.fast, ease: EASE.out, clearProps: 'transform' })
}

/** 图标 / 徽标弹出（结果状态：成功、失败、空状态） */
export function popIn(
  target: Targets,
  opts: { delay?: number; duration?: number; root?: Element | null } = {}
) {
  const els = norm(target, opts.root)
  if (!els.length) return null
  if (prefersReducedMotion()) {
    settle(els)
    return null
  }
  return gsap.fromTo(
    els,
    { opacity: 0, scale: 0.6 },
    {
      opacity: 1,
      scale: 1,
      duration: opts.duration ?? DUR.slow,
      delay: opts.delay ?? 0,
      ease: EASE.back,
      clearProps: 'transform,opacity',
    }
  )
}

/** 遮罩淡入 / 淡出 */
export function maskIn(el: Element | null | undefined) {
  if (!el) return null
  if (prefersReducedMotion()) {
    gsap.set(el, { clearProps: 'opacity' })
    return null
  }
  return gsap.fromTo(el, { opacity: 0 }, { opacity: 1, duration: DUR.fast, ease: EASE.out })
}

/** 弹窗进出场 */
export function dialogIn(el: Element | null | undefined) {
  if (!el) return null
  if (prefersReducedMotion()) {
    gsap.set(el, { clearProps: 'all' })
    return null
  }
  return gsap.fromTo(
    el,
    { opacity: 0, scale: 0.94, y: 12 },
    {
      opacity: 1,
      scale: 1,
      y: 0,
      duration: DUR.slow,
      ease: EASE.out,
      clearProps: 'transform,opacity',
    }
  )
}

export function dialogOut(el: Element | null | undefined, done?: () => void) {
  if (!el) {
    done?.()
    return null
  }
  if (prefersReducedMotion()) {
    gsap.set(el, { clearProps: 'all' })
    done?.()
    return null
  }
  return gsap.to(el, {
    opacity: 0,
    scale: 0.96,
    y: 8,
    duration: DUR.fast,
    ease: EASE.out,
    onComplete: () => {
      gsap.set(el, { clearProps: 'all' })
      done?.()
    },
  })
}

/** Toast 进出场 */
export function toastIn(el: Element | null | undefined) {
  if (!el) return null
  if (prefersReducedMotion()) {
    gsap.set(el, { clearProps: 'all' })
    return null
  }
  return gsap.fromTo(
    el,
    { opacity: 0, y: -12, scale: 0.96 },
    {
      opacity: 1,
      y: 0,
      scale: 1,
      duration: DUR.fast,
      ease: EASE.out,
      clearProps: 'transform,opacity',
    }
  )
}

export function toastOut(el: Element | null | undefined, done?: () => void) {
  if (!el) {
    done?.()
    return null
  }
  if (prefersReducedMotion()) {
    gsap.set(el, { clearProps: 'all' })
    done?.()
    return null
  }
  return gsap.to(el, {
    opacity: 0,
    y: -8,
    duration: DUR.fast,
    ease: EASE.out,
    onComplete: () => {
      gsap.set(el, { clearProps: 'all' })
      done?.()
    },
  })
}

/** 遮罩淡出 */
export function maskOut(el: Element | null | undefined, done?: () => void) {
  if (!el) {
    done?.()
    return null
  }
  if (prefersReducedMotion()) {
    gsap.set(el, { clearProps: 'opacity' })
    done?.()
    return null
  }
  return gsap.to(el, {
    opacity: 0,
    duration: DUR.fast,
    ease: EASE.out,
    onComplete: () => {
      gsap.set(el, { clearProps: 'opacity' })
      done?.()
    },
  })
}

/** 底部动作面板进出场 */
export function sheetIn(el: Element | null | undefined) {
  if (!el) return null
  if (prefersReducedMotion()) {
    gsap.set(el, { clearProps: 'all' })
    return null
  }
  return gsap.fromTo(
    el,
    { yPercent: 100 },
    { yPercent: 0, duration: DUR.slow, ease: EASE.out, clearProps: 'transform' }
  )
}

export function sheetOut(el: Element | null | undefined, done?: () => void) {
  if (!el) {
    done?.()
    return null
  }
  if (prefersReducedMotion()) {
    gsap.set(el, { clearProps: 'all' })
    done?.()
    return null
  }
  return gsap.to(el, {
    yPercent: 100,
    duration: DUR.base,
    ease: EASE.inOut,
    onComplete: () => {
      gsap.set(el, { clearProps: 'all' })
      done?.()
    },
  })
}

/**
 * 数字滚动（统计值变化）
 */
export function countTo(
  el: Element | null | undefined,
  value: number,
  opts: { duration?: number; decimals?: number } = {}
) {
  if (!el) return null
  const decimals = opts.decimals ?? 0
  if (prefersReducedMotion()) {
    el.textContent = value.toFixed(decimals)
    return null
  }
  const obj = { v: Number(el.textContent?.replace(/[^\d.-]/g, '')) || 0 }
  return gsap.to(obj, {
    v: value,
    duration: opts.duration ?? DUR.base,
    ease: EASE.out,
    onUpdate: () => {
      el.textContent = obj.v.toFixed(decimals)
    },
  })
}

/** 进度条平滑推进 */
export function progressTo(el: Element | null | undefined, percent: number) {
  if (!el) return null
  const p = Math.max(0, Math.min(100, percent))
  if (prefersReducedMotion()) {
    gsap.set(el, { width: `${p}%` })
    return null
  }
  return gsap.to(el, { width: `${p}%`, duration: DUR.base, ease: EASE.out })
}
