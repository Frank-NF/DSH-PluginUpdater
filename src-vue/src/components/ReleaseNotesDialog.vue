<template>
  <el-dialog
    v-model="visible"
    :title="t('notes.title')"
    width="660px"
    :close-on-click-modal="true"
  >
    <div v-if="plugin" class="release-notes">
      <!-- 头部：插件 + 版本对比 -->
      <div class="plugin-header">
        <div class="plugin-info">
          <h3>{{ plugin.manifest.name }}</h3>
          <span class="plugin-id">{{ plugin.manifest.id }}</span>
        </div>
        <div class="version-compare">
          <span class="ver-from">v{{ plugin.manifest.current_version }}</span>
          <el-icon class="ver-arrow"><Right /></el-icon>
          <span class="ver-to">v{{ plugin.latest_version }}</span>
        </div>
      </div>

      <!-- 日志内容 -->
      <div class="notes-body">
        <div v-if="plugin.release_notes" class="notes-text">{{ plugin.release_notes }}</div>
        <div v-else class="empty-notes">
          <el-icon :size="40"><Document /></el-icon>
          <p>{{ t('notes.empty') }}</p>
        </div>
      </div>

      <!-- GitHub 链接 -->
      <div class="release-links" v-if="plugin.release_url">
        <el-link type="primary" :href="plugin.release_url" target="_blank">
          <el-icon><Link /></el-icon>
          {{ t('notes.viewOnGithub') }}
        </el-link>
      </div>
    </div>

    <template #footer>
      <el-button @click="visible = false">{{ t('common.close') }}</el-button>
      <el-button
        v-if="plugin?.update_available"
        type="primary"
        @click="handleUpdate"
      >
        <el-icon><Upload /></el-icon>
        {{ t('notes.updateNow') }}
      </el-button>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { Document, Link, Upload, Right } from '@element-plus/icons-vue'
import type { PluginInfo } from '../types'
import { t } from '../i18n'

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

watch(visible, (val) => {
  emit('update:modelValue', val)
})

function handleUpdate() {
  if (props.plugin) {
    emit('update', props.plugin)
    visible.value = false
  }
}
</script>

<style scoped>
.release-notes {
  max-height: 62vh;
  overflow-y: auto;
}

/* ---------- 头部 ---------- */
.plugin-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 16px;
  padding-bottom: 16px;
  border-bottom: 1px solid var(--glass-border);
}

.plugin-info {
  min-width: 0;
}

.plugin-info h3 {
  margin: 0 0 4px;
  font-size: 17px;
  font-weight: 600;
  color: var(--text-primary);
}

.plugin-id {
  font-size: 12px;
  color: var(--text-muted);
  font-family: 'JetBrains Mono', 'Consolas', monospace;
}

/* 版本对比 */
.version-compare {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-shrink: 0;
  padding: 6px 14px;
  border-radius: 20px;
  background: rgba(245, 158, 11, 0.12);
  border: 1px solid rgba(245, 158, 11, 0.3);
  font-family: 'JetBrains Mono', 'Consolas', monospace;
  font-size: 13px;
}

.ver-from {
  color: var(--text-muted);
}

.ver-arrow {
  color: var(--warning);
  font-size: 13px;
}

.ver-to {
  color: var(--warning);
  font-weight: 700;
}

/* ---------- 日志内容 ---------- */
.notes-body {
  margin: 16px 0;
  padding: 16px;
  border-radius: var(--radius-md);
  background: rgba(0, 0, 0, 0.25);
  border: 1px solid var(--glass-border);
  min-height: 140px;
  max-height: 320px;
  overflow-y: auto;
}

.notes-text {
  white-space: pre-wrap;
  word-break: break-word;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Microsoft YaHei', sans-serif;
  font-size: 13px;
  line-height: 1.75;
  color: var(--text-secondary);
  margin: 0;
}

.empty-notes {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 32px;
  color: var(--text-muted);
  gap: 10px;
}

.empty-notes p {
  margin: 0;
  font-size: 13px;
}

/* ---------- 底部链接 ---------- */
.release-links {
  padding-top: 14px;
  border-top: 1px solid var(--glass-border);
  text-align: center;
}

.release-links :deep(.el-link) {
  font-size: 13px;
}
</style>
