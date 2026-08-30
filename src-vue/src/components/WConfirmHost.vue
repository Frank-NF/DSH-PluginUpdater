<template>
  <!-- 复用 WeUI Dialog：外观与动效与全局一致 -->
  <WDialog
    v-if="state"
    :model-value="true"
    :title="state.title"
    :close-on-mask="state.closeOnMask ?? false"
    :closable="true"
    @update:model-value="onVisibleChange"
  >
    <p v-if="state.message" class="w-confirm-msg">{{ state.message }}</p>

    <template #footer>
      <a
        class="weui-btn weui-btn_default w-confirm-btn"
        @click="cancel"
      >
        {{ state.cancelText || '取消' }}
      </a>
      <a
        class="weui-btn w-confirm-btn"
        :class="confirmCls"
        @click="accept"
      >
        {{ state.confirmText || '确定' }}
      </a>
    </template>
  </WDialog>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import WDialog from './WDialog.vue'
import { useConfirm } from '../composables/useConfirm'

const { state, accept, cancel } = useConfirm()

const confirmCls = computed(() => {
  switch (state.value?.type) {
    case 'danger':
      return 'weui-btn_warn'
    case 'warn':
      return 'weui-btn_warn'
    default:
      return 'weui-btn_primary'
  }
})

/** 弹窗被外部关闭（遮罩 / 关闭按钮 / Esc）→ 视为取消 */
function onVisibleChange(v: boolean) {
  if (!v) cancel()
}
</script>

<style scoped>
.w-confirm-msg {
  margin: 0;
  white-space: pre-line;
  color: var(--weui-FG-1);
  font-size: 14px;
  line-height: 1.7;
}

.w-confirm-btn {
  min-width: 92px;
  margin: 0;
  line-height: 40px;
  font-size: 15px;
}
</style>
