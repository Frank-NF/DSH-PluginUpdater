<template>
  <div class="w-progress">
    <div class="weui-progress">
      <div class="weui-progress__bar">
        <div ref="barEl" class="weui-progress__inner-bar" style="width: 0%" />
      </div>
    </div>
    <span class="w-progress-text">
      <span ref="numEl">0</span>%<template v-if="message"> · {{ message }}</template>
    </span>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref, watch } from 'vue'
import { progressTo, countTo } from '../composables/useMotion'

const props = withDefaults(
  defineProps<{ percent: number; message?: string }>(),
  { message: '' }
)

const barEl = ref<HTMLElement | null>(null)
const numEl = ref<HTMLElement | null>(null)

function play() {
  progressTo(barEl.value, props.percent)
  countTo(numEl.value, props.percent)
}

onMounted(play)
watch(() => props.percent, play)
</script>

<style scoped>
.w-progress {
  flex: 1;
  min-width: 140px;
}

.weui-progress__inner-bar {
  min-width: 0;
}
</style>
