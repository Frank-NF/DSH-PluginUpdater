<template>
  <WDialog v-model="visible" :title="t('notes.title')">
    <div v-if="plugin" class="w-notes">
      <!-- 头部：插件 + 版本对比 -->
      <div class="w-notes-head">
        <div class="w-flex-1">
          <h3>{{ plugin.manifest.name }}</h3>
          <p class="w-notes-id mono">{{ plugin.manifest.id }}</p>
        </div>
        <div class="w-version" :class="{ 'is-update': plugin.update_available }">
          <span class="mono w-text-2">v{{ plugin.manifest.current_version }}</span>
          <WIcon name="arrowRight" :size="13" class="w-text-warn" />
          <span class="mono w-text-warn">v{{ plugin.latest_version }}</span>
        </div>
      </div>

      <!-- 日志正文 -->
      <div class="w-notes-body">
        <div v-if="plugin.release_notes" class="w-notes-text">{{ plugin.release_notes }}</div>
        <WEmpty
          v-else
          type="empty"
          icon="fileText"
          :title="t('notes.empty')"
          :icon-size="44"
          align-top
        />
      </div>

      <!-- GitHub 链接 -->
      <a
        v-if="plugin.release_url"
        class="weui-cell weui-cell_access w-notes-link"
        href="javascript:"
        @click="openRelease"
      >
        <div class="weui-cell__bd">{{ t('notes.viewOnGithub') }}</div>
        <div class="weui-cell__ft w-text-2">
          <WIcon name="external" :size="14" />
        </div>
      </a>
    </div>

    <template #footer>
      <WButton @click="visible = false">{{ t('common.close') }}</WButton>
      <WButton
        v-if="plugin?.update_available"
        type="primary"
        icon="upload"
        @click="handleUpdate"
      >
        {{ t('notes.updateNow') }}
      </WButton>
    </template>
  </WDialog>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import WDialog from './WDialog.vue'
import WButton from './WButton.vue'
import WIcon from './WIcon.vue'
import WEmpty from './WEmpty.vue'
import { pluginApi } from '../api'
import { t } from '../i18n'
import type { PluginInfo } from '../types'

const props = defineProps<{
  modelValue: boolean
  plugin: PluginInfo | null
}>()

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
  update: [plugin: PluginInfo]
}>()

const visible = ref(props.modelValue)

watch(
  () => props.modelValue,
  (val) => {
    visible.value = val
  }
)

watch(visible, (val) => emit('update:modelValue', val))

function handleUpdate() {
  if (props.plugin) {
    emit('update', props.plugin)
    visible.value = false
  }
}

function openRelease() {
  if (props.plugin?.release_url) {
    pluginApi.openExternal(props.plugin.release_url).catch(() => {})
  }
}
</script>

<style scoped>
.w-notes-head {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: var(--sp-3);
  flex-wrap: wrap;
  padding-bottom: var(--sp-3);
  border-bottom: 1px solid var(--border);
}

.w-notes-head h3 {
  font-size: 16px;
  font-weight: 600;
  color: var(--fg);
  line-height: 1.4;
}

.w-notes-id {
  font-size: 12px;
  color: var(--fg-2);
  margin-top: 2px;
  word-break: break-all;
}

.w-notes-body {
  margin: var(--sp-3) 0;
  padding: var(--sp-3);
  border-radius: var(--r-md);
  background: var(--bg-group);
  max-height: 46vh;
  overflow-y: auto;
}

.w-notes-text {
  white-space: pre-wrap;
  word-break: break-word;
  font-size: 13px;
  line-height: 1.75;
  color: var(--fg-1);
}

.w-notes-link {
  padding: var(--sp-2) 0;
  border-radius: var(--r-sm);
}

.w-notes-link::before {
  display: none;
}
</style>
