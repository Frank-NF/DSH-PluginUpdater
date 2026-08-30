<template>
  <span ref="wrapEl" class="w-menu-wrap" @click.stop="toggle">
    <slot name="trigger" />
  </span>
  <Teleport to="body">
    <div v-if="open" class="w-menu-overlay" @click.stop="close" />
    <div
      v-if="open"
      class="w-menu"
      :style="{ top: pos.top + 'px', left: pos.left + 'px', minWidth: menuWidth + 'px' }"
      role="menu"
    >
      <button
        v-for="item in items"
        :key="item.value"
        type="button"
        class="w-menu__item"
        :class="{ 'is-warn': item.type === 'warn', 'is-active': item.value === modelValue }"
        role="menuitem"
        @click.stop="pick(item.value)"
      >
        <span class="w-menu__label">{{ item.label }}</span>
        <WIcon v-if="item.value === modelValue" name="check" :size="14" class="w-menu__check" />
      </button>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { nextTick, onBeforeUnmount, ref } from 'vue'
import WIcon from './WIcon.vue'

export interface WMenuItem {
  label: string
  value: string
  type?: 'warn'
  desc?: string
}

const props = defineProps<{
  items: WMenuItem[]
  modelValue?: string
  align?: 'left' | 'right'
  width?: number
}>()

const emit = defineEmits<{ select: [value: string] }>()

const open = ref(false)
const wrapEl = ref<HTMLElement | null>(null)
const pos = ref({ top: 0, left: 0 })
const menuWidth = ref(168)

function toggle() {
  if (open.value) close()
  else show()
}

function show() {
  const el = wrapEl.value
  if (!el || !props.items.length) return
  const r = el.getBoundingClientRect()
  const width = props.width || 168
  menuWidth.value = width
  const height = props.items.length * 40 + 12
  let top = r.bottom + 6
  if (top + height > window.innerHeight - 8) top = Math.max(8, r.top - height - 6)
  let left = props.align === 'right' ? r.right - width : r.left
  left = Math.min(Math.max(8, left), Math.max(8, window.innerWidth - width - 8))
  pos.value = { top, left }
  open.value = true
  nextTick(addGlobal)
}

function close() {
  open.value = false
  removeGlobal()
}

function pick(value: string) {
  emit('select', value)
  close()
}

function onKey(e: KeyboardEvent) {
  if (e.key === 'Escape') close()
}
function onScrollOrResize() {
  if (open.value) close()
}

function addGlobal() {
  window.addEventListener('keydown', onKey)
  window.addEventListener('scroll', onScrollOrResize, true)
  window.addEventListener('resize', onScrollOrResize)
}
function removeGlobal() {
  window.removeEventListener('keydown', onKey)
  window.removeEventListener('scroll', onScrollOrResize, true)
  window.removeEventListener('resize', onScrollOrResize)
}

onBeforeUnmount(removeGlobal)
</script>
