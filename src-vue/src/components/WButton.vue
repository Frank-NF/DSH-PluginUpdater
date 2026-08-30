<template>
  <button
    ref="btnEl"
    type="button"
    class="weui-btn"
    :class="[typeCls, sizeCls, { 'weui-btn_disabled': disabled || loading, 'w-btn_block': block }]"
    :disabled="disabled || loading"
    :aria-busy="loading || undefined"
    @pointerdown="onDown"
    @pointerup="onUp"
    @pointerleave="onUp"
    @pointercancel="onUp"
    @click="$emit('click', $event)"
  >
    <i v-if="loading" class="weui-loading w-btn-loading" aria-hidden="true" />
    <WIcon v-else-if="icon" :name="icon" :size="iconSize" />
    <span v-if="$slots.default" class="w-btn-text"><slot /></span>
  </button>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import WIcon from './WIcon.vue'
import { pressIn, pressOut } from '../composables/useMotion'

const props = withDefaults(
  defineProps<{
    /** primary 主行动 / default 次行动 / warn 危险 / plain 描边弱化 */
    type?: 'primary' | 'default' | 'warn' | 'plain'
    /** mini 小按钮（卡片内操作）/ inline 行内紧凑 */
    size?: 'normal' | 'mini' | 'inline'
    icon?: string
    iconSize?: number
    loading?: boolean
    disabled?: boolean
    block?: boolean
  }>(),
  { type: 'default', size: 'normal', iconSize: 16, loading: false, disabled: false, block: false }
)

defineEmits<{ click: [e: MouseEvent] }>()

const btnEl = ref<HTMLElement | null>(null)

const typeCls = computed(() => {
  switch (props.type) {
    case 'primary':
      return 'weui-btn_primary'
    case 'warn':
      return 'weui-btn_warn'
    case 'plain':
      return 'weui-btn_plain-default'
    default:
      return 'weui-btn_default'
  }
})

const sizeCls = computed(() => {
  switch (props.size) {
    case 'mini':
      return 'weui-btn_mini'
    case 'inline':
      return 'weui-btn_inline weui-btn_mini'
    default:
      return ''
  }
})

function onDown() {
  if (props.disabled || props.loading) return
  pressIn(btnEl.value)
}

function onUp() {
  if (props.disabled || props.loading) return
  pressOut(btnEl.value)
}
</script>

<style scoped>
.weui-btn {
  /* 结构由全局 main.css 统一定义，这里只做组件级微调 */
}

.w-btn_block {
  display: flex;
  width: 100%;
}

.w-btn-loading {
  width: 15px;
  height: 15px;
}

.w-btn-text:empty {
  display: none;
}
</style>
