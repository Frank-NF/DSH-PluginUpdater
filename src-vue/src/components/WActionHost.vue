<template>
  <WSheet
    v-if="state"
    :model-value="true"
    :title="state.title"
    :items="state.items"
    :cancel-text="state.cancelText"
    @update:model-value="onVisibleChange"
    @select="onSelect"
  >
    <template v-if="state.message" #message>
      <p class="w-action-msg">{{ state.message }}</p>
    </template>
  </WSheet>
</template>

<script setup lang="ts">
import WSheet from './WSheet.vue'
import { useActionSheet } from '../composables/useActionSheet'

const { state, pick, cancel } = useActionSheet()

function onSelect(value: string) {
  pick(value)
}

/** 遮罩 / Esc 关闭 → 视为取消 */
function onVisibleChange(v: boolean) {
  if (!v) cancel()
}
</script>

<style scoped>
.w-action-msg {
  margin: 0;
}
</style>
