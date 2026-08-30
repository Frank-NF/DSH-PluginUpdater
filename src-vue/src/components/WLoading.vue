<template>
  <!-- 区块型：整行居中（WeUI loadmore） -->
  <div v-if="block" class="weui-loadmore" :class="{ 'weui-loadmore_line': line }">
    <i class="weui-loading" aria-hidden="true" />
    <span class="weui-loadmore__tips">{{ text }}</span>
  </div>

  <!-- 行内型：跟随文字 -->
  <span v-else class="w-loading-inline">
    <i class="weui-loading" aria-hidden="true" />
    <span v-if="text" class="w-loading-text">{{ text }}</span>
  </span>
</template>

<script setup lang="ts">
withDefaults(
  defineProps<{
    text?: string
    /** 区块型（独占一行并居中） */
    block?: boolean
    /** 带分隔线 */
    line?: boolean
  }>(),
  { text: '加载中', block: false, line: false }
)
</script>

<style scoped>
.w-loading-inline {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  color: var(--weui-FG-2);
}

.w-loading-text {
  line-height: 1;
}

/* WeUI .weui-loading 为固定尺寸，这里统一缩小以贴合 13px 文案 */
.w-loading-inline .weui-loading {
  width: 14px;
  height: 14px;
}
</style>
