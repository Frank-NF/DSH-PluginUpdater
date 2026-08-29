<template>
  <el-dialog
    v-model="visible"
    :title="t('repair.title')"
    width="760px"
    :close-on-click-modal="true"
  >
    <div class="repair-container">
      <!-- 环境体检 -->
      <div class="repair-section">
        <div class="section-head">
          <h4>
            <el-icon><FirstAidKit /></el-icon>
            {{ t('repair.envTitle') }}
          </h4>
          <el-button
            size="small"
            type="primary"
            plain
            :loading="checking"
            :icon="Refresh"
            @click="runCheck"
          >{{ t('repair.runCheck') }}</el-button>
        </div>

        <div v-if="checking" class="check-loading">
          <el-icon class="is-loading"><Loading /></el-icon>
          {{ t('repair.checking') }}
        </div>

        <ul v-else-if="envItems.length" class="env-list">
          <li v-for="item in envItems" :key="item.id" class="env-item" :class="item.status">
            <span class="env-status">
              <el-icon v-if="item.status === 'ok'"><CircleCheck /></el-icon>
              <el-icon v-else-if="item.status === 'warn'"><WarningFilled /></el-icon>
              <el-icon v-else><CircleClose /></el-icon>
            </span>
            <div class="env-body">
              <div class="env-name">
                {{ item.name }}
                <span class="env-badge" :class="item.status">{{ statusText(item.status) }}</span>
              </div>
              <div class="env-message">{{ item.message }}</div>
              <div v-if="item.fix_hint" class="env-fix">
                <el-icon><InfoFilled /></el-icon>
                {{ item.fix_hint }}
              </div>
            </div>
          </li>
        </ul>
        <div v-else class="check-loading">{{ t('repair.noData') }}</div>
      </div>

      <el-divider />

      <!-- 常见报错修复 -->
      <div class="repair-section">
        <div class="section-head">
          <h4>
            <el-icon><Tools /></el-icon>
            {{ t('repair.guideTitle') }}
          </h4>
          <el-input
            v-model="search"
            size="small"
            :placeholder="t('repair.searchPlaceholder')"
            clearable
            class="guide-search"
          />
        </div>

        <div class="guide-list">
          <div
            v-for="g in filteredGuides"
            :key="g.id"
            class="guide-item"
          >
            <div class="guide-head" @click="toggleGuide(g.id)">
              <span class="guide-arrow" :class="{ open: expanded === g.id }">
                <el-icon><ArrowRight /></el-icon>
              </span>
              <span class="guide-title">{{ locale === 'zh' ? g.title.zh : g.title.en }}</span>
            </div>
            <div v-if="expanded === g.id" class="guide-detail">
              <div class="guide-cause">
                <span class="guide-label">{{ t('repair.cause') }}:</span>
                {{ locale === 'zh' ? g.cause.zh : g.cause.en }}
              </div>
              <ol class="guide-steps">
                <li v-for="(s, i) in (locale === 'zh' ? g.steps.zh : g.steps.en)" :key="i">{{ s }}</li>
              </ol>
            </div>
          </div>
          <div v-if="filteredGuides.length === 0" class="guide-empty">{{ t('repair.noMatch') }}</div>
        </div>
      </div>
    </div>

    <template #footer>
      <el-button @click="visible = false">{{ t('common.close') }}</el-button>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import {
  FirstAidKit,
  Refresh,
  Loading,
  CircleCheck,
  CircleClose,
  WarningFilled,
  InfoFilled,
  Tools,
  ArrowRight,
} from '@element-plus/icons-vue'
import { t, locale } from '../i18n'
import { pluginApi } from '../api'
import { REPAIR_GUIDES } from '../data/repairGuide'
import type { EnvCheckItem } from '../types'

const props = defineProps<{ modelValue: boolean }>()
const emit = defineEmits<{ 'update:modelValue': [value: boolean] }>()

const visible = ref(props.modelValue)
watch(() => props.modelValue, (v) => { visible.value = v })

const envItems = ref<EnvCheckItem[]>([])
const checking = ref(false)
const search = ref('')
const expanded = ref<string | null>(null)

async function runCheck() {
  checking.value = true
  try {
    envItems.value = await pluginApi.checkEnvironment()
  } catch (e) {
    envItems.value = []
  } finally {
    checking.value = false
  }
}

function toggleGuide(id: string) {
  expanded.value = expanded.value === id ? null : id
}

function statusText(s: string): string {
  if (s === 'ok') return t('repair.statusOk')
  if (s === 'warn') return t('repair.statusWarn')
  return t('repair.statusError')
}

const filteredGuides = computed(() => {
  const q = search.value.trim().toLowerCase()
  if (!q) return REPAIR_GUIDES
  return REPAIR_GUIDES.filter((g) => {
    const haystack = [
      g.title.zh, g.title.en, g.cause.zh, g.cause.en, ...g.keywords,
    ].join(' ').toLowerCase()
    return haystack.includes(q)
  })
})
</script>

<style scoped>
.repair-container {
  max-height: 62vh;
  overflow-y: auto;
  padding-right: 4px;
}

.section-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 12px;
}

.section-head h4 {
  display: flex;
  align-items: center;
  gap: 6px;
  margin: 0;
  font-size: 14px;
}

.guide-search {
  width: 220px;
}

.env-list {
  list-style: none;
  margin: 0;
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.env-item {
  display: flex;
  gap: 10px;
  padding: 10px 12px;
  border-radius: 8px;
  background: rgba(255, 255, 255, 0.04);
  border: 1px solid rgba(255, 255, 255, 0.08);
}

.env-item.ok .env-status { color: var(--el-color-success); }
.env-item.warn .env-status { color: var(--el-color-warning); }
.env-item.error .env-status { color: var(--el-color-danger); }

.env-status {
  flex-shrink: 0;
  font-size: 16px;
  margin-top: 2px;
}

.env-body { flex: 1; min-width: 0; }

.env-name {
  font-weight: 600;
  font-size: 13px;
  display: flex;
  align-items: center;
  gap: 8px;
}

.env-badge {
  font-size: 11px;
  font-weight: 500;
  padding: 1px 8px;
  border-radius: 10px;
}
.env-badge.ok { background: rgba(103, 194, 58, 0.15); color: var(--el-color-success); }
.env-badge.warn { background: rgba(230, 162, 60, 0.15); color: var(--el-color-warning); }
.env-badge.error { background: rgba(245, 108, 108, 0.15); color: var(--el-color-danger); }

.env-message {
  font-size: 12px;
  color: var(--text-secondary, #8b93a7);
  margin-top: 2px;
  word-break: break-all;
}

.env-fix {
  display: flex;
  align-items: flex-start;
  gap: 4px;
  font-size: 12px;
  color: var(--el-color-warning);
  margin-top: 4px;
}

.check-loading {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 16px;
  color: var(--text-secondary, #8b93a7);
  font-size: 13px;
}

.guide-list {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.guide-item {
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 8px;
  background: rgba(255, 255, 255, 0.03);
  overflow: hidden;
}

.guide-head {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 12px;
  cursor: pointer;
  font-size: 13px;
  font-weight: 500;
  transition: background 0.15s;
}
.guide-head:hover { background: rgba(255, 255, 255, 0.05); }

.guide-arrow {
  display: inline-flex;
  transition: transform 0.15s;
  color: var(--text-secondary, #8b93a7);
}
.guide-arrow.open { transform: rotate(90deg); }

.guide-detail {
  padding: 0 12px 12px 34px;
  font-size: 12px;
}

.guide-cause {
  color: var(--text-secondary, #8b93a7);
  margin-bottom: 8px;
  line-height: 1.6;
}

.guide-label {
  color: var(--el-color-warning);
  font-weight: 600;
}

.guide-steps {
  margin: 0;
  padding-left: 18px;
  line-height: 1.8;
  color: var(--text-primary);
}

.guide-empty {
  padding: 20px;
  text-align: center;
  color: var(--text-secondary, #8b93a7);
}
</style>
