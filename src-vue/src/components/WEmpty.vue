<template>
  <div ref="rootEl" class="weui-msg" :class="{ 'weui-msg_align-top': alignTop }">
    <!-- 图标区 -->
    <div ref="iconEl" class="weui-msg__icon-area">
      <!-- 内置语义图标（成功 / 警告 / 错误 / 等待） -->
      <i
        v-if="iconClass"
        class="weui-icon_msg"
        :class="iconClass"
        aria-hidden="true"
      />
      <!-- 空状态：自定义线性图标 -->
      <span v-else class="w-empty-custom" aria-hidden="true">
        <WIcon :name="customIcon" :size="iconSize" :stroke-width="1.5" />
      </span>
    </div>

    <!-- 文本区 -->
    <div class="weui-msg__text-area">
      <h2 class="weui-msg__title">{{ title }}</h2>
      <p v-if="desc" class="weui-msg__desc">{{ desc }}</p>
      <slot />
    </div>

    <!-- 主操作 -->
    <div v-if="$slots.action" class="weui-msg__opr-area">
      <p class="weui-btn-area">
        <slot name="action" />
      </p>
    </div>

    <!-- 次要操作 / 提示 -->
    <div v-if="$slots.tips" class="weui-msg__tips-area">
      <p class="weui-msg__tips">
        <slot name="tips" />
      </p>
    </div>

    <div v-if="$slots.extra" class="weui-msg__extra-area">
      <slot name="extra" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import WIcon from './WIcon.vue'
import { popIn } from '../composables/useMotion'

/**
 * 结果页 / 空状态（WeUI .weui-msg）
 * 覆盖：空状态、成功、失败、错误、加载等待
 */
const props = withDefaults(
  defineProps<{
    /** 语义类型 */
    type?: 'empty' | 'success' | 'error' | 'warn' | 'info' | 'loading'
    title: string
    desc?: string
    /** type=empty 时的自定义图标名 */
    icon?: string
    iconSize?: number
    alignTop?: boolean
  }>(),
  { type: 'empty', icon: 'inbox', iconSize: 56, alignTop: false }
)

const rootEl = ref<HTMLElement | null>(null)
const iconEl = ref<HTMLElement | null>(null)

const iconClass = computed(() => {
  switch (props.type) {
    case 'success':
      return 'weui-icon-success weui-icon_msg-primary'
    case 'error':
    case 'warn':
      return 'weui-icon-warn'
    case 'info':
      return 'weui-icon-info'
    case 'loading':
      return 'weui-icon-waiting'
    default:
      return ''
  }
})

const customIcon = computed(() => props.icon)

function play() {
  popIn(iconEl.value)
}

onMounted(play)
// 内容切换（如从错误重试回正常）时重新播一次
watch(() => [props.type, props.title], () => play())
</script>

<style scoped>
/* 空状态图标：弱化，不与结果图标抢视觉 */
.w-empty-custom {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 80px;
  height: 80px;
  border-radius: 50%;
  background: var(--bg-group);
  color: var(--fg-2);
  border: 1px solid var(--border);
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.06);
}

.weui-msg__desc {
  white-space: pre-line;
  color: var(--fg-2);
}

.weui-msg__title {
  color: var(--fg);
  font-weight: 700;
}

/* 桌面端收窄，避免标题在超宽屏上过于分散 */
.weui-msg__text-area {
  max-width: 560px;
  margin: 0 auto;
}

.weui-btn-area {
  display: flex;
  gap: 12px;
  justify-content: center;
  flex-wrap: wrap;
  margin: 0;
}
</style>
