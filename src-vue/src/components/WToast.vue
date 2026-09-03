<template>
  <Teleport to="body">
    <TransitionGroup
      tag="div"
      class="w-toast-host"
      @enter="onEnter"
      @leave="onLeave"
      appear
    >
      <div v-for="item in toasts" :key="item.id" class="w-toast-item">
        <!-- 图标型 -->
        <div v-if="item.type !== 'text'" class="weui-toast">
          <i
            v-if="item.type === 'loading'"
            class="weui-loading weui-icon_toast"
            aria-hidden="true"
          />
          <i
            v-else
            class="weui-icon_toast"
            :class="item.type === 'success' ? 'weui-icon-success' : 'weui-icon-warn'"
            aria-hidden="true"
          />
          <p class="weui-toast__content">{{ item.text }}</p>
        </div>

        <!-- 纯文字型 -->
        <div v-else class="weui-toast weui-toast_text">
          <p class="weui-toast__content">{{ item.text }}</p>
        </div>
      </div>
    </TransitionGroup>
  </Teleport>
</template>

<script setup lang="ts">
import { useToast } from '../composables/useToast'
import { toastIn, toastOut } from '../composables/useMotion'

const { toasts } = useToast()

function onEnter(el: Element, done: () => void) {
  const tw = toastIn(el)
  if (tw) tw.eventCallback('onComplete', done)
  else done()
}

function onLeave(el: Element, done: () => void) {
  const tw = toastOut(el)
  if (tw) tw.eventCallback('onComplete', done)
  else done()
}
</script>

<style scoped>
/* 容器：顶部居中纵向堆叠；WeUI 视觉交给 .weui-toast */
.w-toast-host {
  position: fixed;
  top: 12%;
  left: 0;
  right: 0;
  z-index: 5000;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
  pointer-events: none;
}

.w-toast-item {
  pointer-events: none;
}

/* Toast 文案换行友好 */
.weui-toast__content {
  max-width: 78vw;
  word-break: break-word;
}
</style>
