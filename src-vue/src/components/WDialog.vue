<template>
  <Teleport to="body">
    <div v-if="render" class="w-dialog-root" role="presentation">
      <!-- 遮罩 -->
      <div ref="maskEl" class="weui-mask" @click="onMaskClick" />

      <!-- 对话框容器：负责居中，便于 GSAP 安全动画 transform -->
      <div ref="wrapEl" class="w-dialog-wrap">
        <div
          ref="dialogEl"
          class="weui-dialog"
          :class="{ 'w-dialog_wide': wide }"
          role="dialog"
          aria-modal="true"
          :aria-label="title || '对话框'"
        >
          <div v-if="title" class="weui-dialog__hd">
            <strong class="weui-dialog__title">{{ title }}</strong>
            <button
              v-if="closable"
              type="button"
              class="w-dialog-close"
              :aria-label="closeText"
              @click="close"
            >
              <WIcon name="close" :size="20" />
            </button>
          </div>

          <div class="weui-dialog__bd">
            <slot />
          </div>

          <div v-if="$slots.footer" class="weui-dialog__ft">
            <slot name="footer" />
          </div>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, watch, nextTick, onMounted, onBeforeUnmount } from 'vue'
import WIcon from './WIcon.vue'
import { dialogIn, dialogOut, maskIn, maskOut } from '../composables/useMotion'

const props = withDefaults(
  defineProps<{
    modelValue: boolean
    title?: string
    /** 点遮罩关闭 */
    closeOnMask?: boolean
    /** 显示右上角关闭按钮 */
    closable?: boolean
    closeText?: string
    /** 宽版（设置、修复中心等复杂内容） */
    wide?: boolean
    /** 加载中：禁止遮罩关闭与关闭按钮 */
    busy?: boolean
  }>(),
  { closeOnMask: true, closable: true, closeText: '关闭', wide: false, busy: false }
)

const emit = defineEmits<{ 'update:modelValue': [value: boolean] }>()

const render = ref(props.modelValue)
const maskEl = ref<HTMLElement | null>(null)
const wrapEl = ref<HTMLElement | null>(null)
const dialogEl = ref<HTMLElement | null>(null)
let closing = false

function open() {
  closing = false
  render.value = true
  nextTick(() => {
    maskIn(maskEl.value)
    dialogIn(dialogEl.value)
    // 焦点移入对话框（无障碍）
    dialogEl.value?.focus?.()
  })
}

function finishClose() {
  render.value = false
  closing = false
}

function close() {
  if (!render.value || closing) return
  closing = true
  maskOut(maskEl.value)
  dialogOut(dialogEl.value, finishClose)
  emit('update:modelValue', false)
}

function onMaskClick() {
  if (props.closeOnMask && !props.busy) close()
}

function onKey(e: KeyboardEvent) {
  if (e.key === 'Escape' && render.value && !props.busy) {
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

// Esc 关闭：捕获阶段，避免被多个弹窗同时响应
onMounted(() => window.addEventListener('keydown', onKey, true))
onBeforeUnmount(() => window.removeEventListener('keydown', onKey, true))
</script>

<style scoped>
.w-dialog-root {
  position: fixed;
  inset: 0;
  z-index: 5000;
}

/* 居中容器 */
.w-dialog-wrap {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 16px;
  pointer-events: none;
}

/* 覆盖 WeUI 自带的 fixed 定位：改为由容器居中，让 GSAP 自由动画 transform */
.weui-dialog {
  position: relative;
  top: auto;
  left: auto;
  right: auto;
  transform: none;
  width: 100%;
  max-width: 360px;
  max-height: 86vh;
  display: flex;
  flex-direction: column;
  text-align: left;
  pointer-events: auto;
  outline: none;
  background: var(--bg-card);
  backdrop-filter: var(--glass-blur);
  -webkit-backdrop-filter: var(--glass-blur);
  border: 1px solid var(--border);
  border-radius: var(--r-xl);
  box-shadow: var(--shadow-lg);
}

.w-dialog_wide {
  max-width: 640px;
}

.weui-dialog__hd {
  position: relative;
  padding: 22px 26px 14px;
}

.weui-dialog__title {
  color: var(--fg);
  font-size: 17px;
  font-weight: 700;
}

.weui-dialog__bd {
  padding: 8px 26px 24px;
  overflow-y: auto;
  flex: 1;
  min-height: 0;
  font-size: 14px;
  line-height: 1.65;
  color: var(--fg-1);
}

.weui-dialog__ft {
  display: flex;
  gap: 10px;
  padding: 14px 26px 22px;
  justify-content: flex-end;
  flex-wrap: wrap;
}

.w-dialog-close {
  position: absolute;
  top: 16px;
  right: 16px;
  width: 32px;
  height: 32px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: var(--bg-group);
  color: var(--fg-2);
  border-radius: 50%;
  cursor: pointer;
  transition: background-color 0.2s var(--ease-out, ease), color 0.2s var(--ease-out, ease);
}

.w-dialog-close:hover {
  background: var(--bg-hover);
  color: var(--fg);
}

@media (max-width: 480px) {
  .weui-dialog {
    max-width: none;
    border-radius: var(--r-lg);
  }
}
</style>
