<template>
  <Teleport to="body">
    <div v-if="render" class="w-sheet-root">
      <div ref="maskEl" class="weui-mask" @click="close" />

      <div
        ref="sheetEl"
        class="weui-actionsheet"
        role="dialog"
        aria-modal="true"
        :aria-label="title || '操作'"
      >
        <div v-if="title || $slots.message" class="weui-actionsheet__title">
          <p v-if="title" class="weui-actionsheet__title-text">{{ title }}</p>
          <div v-if="$slots.message" class="w-sheet-msg">
            <slot name="message" />
          </div>
        </div>

        <div class="weui-actionsheet__menu">
          <a
            v-for="item in items"
            :key="item.value"
            class="weui-actionsheet__cell"
            :class="{ 'weui-actionsheet__cell_warn': item.type === 'warn' }"
            href="javascript:"
            @click="pick(item)"
          >
            <span>{{ item.label }}</span>
            <span v-if="item.desc" class="weui-actionsheet__cell__tips">{{ item.desc }}</span>
          </a>
        </div>

        <div class="weui-actionsheet__action">
          <a class="weui-actionsheet__cell" href="javascript:" @click="close">
            {{ cancelText }}
          </a>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, watch, nextTick, onMounted, onBeforeUnmount } from 'vue'
import { sheetIn, sheetOut, maskIn, maskOut } from '../composables/useMotion'
import type { SheetItem } from '../composables/useActionSheet'

const props = withDefaults(
  defineProps<{
    modelValue: boolean
    title?: string
    items: SheetItem[]
    cancelText?: string
  }>(),
  { cancelText: '取消' }
)

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
  select: [value: string]
}>()

const render = ref(props.modelValue)
const maskEl = ref<HTMLElement | null>(null)
const sheetEl = ref<HTMLElement | null>(null)
let closing = false

function open() {
  closing = false
  render.value = true
  nextTick(() => {
    maskIn(maskEl.value)
    sheetIn(sheetEl.value)
  })
}

function close() {
  if (!render.value || closing) return
  closing = true
  maskOut(maskEl.value)
  sheetOut(sheetEl.value, () => {
    render.value = false
    closing = false
  })
  emit('update:modelValue', false)
}

function pick(item: SheetItem) {
  emit('select', item.value)
  close()
}

function onKey(e: KeyboardEvent) {
  if (e.key === 'Escape' && render.value) {
    e.stopPropagation()
    close()
  }
}

watch(
  () => props.modelValue,
  (v) => {
    if (v) open()
    else if (render.value) close()
  }
)

onMounted(() => window.addEventListener('keydown', onKey, true))
onBeforeUnmount(() => window.removeEventListener('keydown', onKey, true))
</script>

<style scoped>
.w-sheet-root {
  position: fixed;
  inset: 0;
  z-index: 5000;
}

/* 交给 GSAP 控制进出场；必须覆盖 WeUI 类里的 translate(0,100%)，
   否则 GSAP clearProps 后面板回落到屏幕外（遮罩在、面板消失） */
.weui-actionsheet {
  transition: none;
  transform: none;
  max-height: 80vh;
  overflow-y: auto;
  padding-bottom: env(safe-area-inset-bottom, 0);
}

.w-sheet-msg {
  padding: 4px 16px 0;
  font-size: 13px;
  line-height: 1.6;
  color: var(--fg-1);
  text-align: left;
  white-space: pre-line;
}
</style>
