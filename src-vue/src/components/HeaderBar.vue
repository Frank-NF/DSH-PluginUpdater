<template>
  <header class="header-shell">
    <div class="header-bar glass">
      <!-- 左：品牌标识 -->
      <div class="header-left">
        <div class="logo">
          <div class="logo-mark">DSH</div>
          <div class="logo-text">
            <h1>{{ t('header.title') }}</h1>
            <span class="subtitle">{{ t('header.subtitle') }}</span>
          </div>
        </div>
      </div>

      <!-- 中：目录输入 + 扫描 -->
      <div class="header-center">
        <div class="directory-input">
          <el-input
            v-model="localDirectory"
            :placeholder="t('header.dirPlaceholder')"
            clearable
            @keyup.enter="handleScan"
          >
            <template #prefix>
              <el-icon><Folder /></el-icon>
            </template>
          </el-input>
          <el-button type="primary" :loading="isScanning" @click="handleScan">
            <el-icon><Search /></el-icon>
            {{ t('header.scan') }}
          </el-button>
        </div>
      </div>

      <!-- 右：统计 + 操作 -->
      <div class="header-right">
        <div class="stats" v-if="pluginCount > 0">
          <div class="stat-pill">
            <span class="stat-value">{{ pluginCount }}</span>
            <span class="stat-label">{{ t('header.plugins') }}</span>
          </div>
          <div class="stat-pill update" v-if="updatableCount > 0">
            <span class="stat-value">{{ updatableCount }}</span>
            <span class="stat-label">{{ t('header.updatable') }}</span>
          </div>
        </div>

        <div class="action-group">
          <el-button :loading="autoScanning" @click="handleAutoScan">
            <el-icon><MagicStick /></el-icon> {{ t('header.autoScan') }} </el-button>

          <el-button
            type="primary"
            :loading="isCheckingUpdates"
            :disabled="pluginCount === 0"
            @click="$emit('check-updates')"
          >
            <el-icon><Refresh /></el-icon>
            {{ t('header.checkUpdates') }}
          </el-button>

          <el-button
            class="icon-only lang-switch"
            :title="locale === 'zh' ? 'Switch to English' : '切换到中文'"
            @click="toggleLocale"
          >
            <span class="lang-label">{{ t('header.langSwitch') }}</span>
          </el-button>

          <el-button
            class="icon-only"
            :title="t('repair.title')"
            :aria-label="t('repair.title')"
            @click="$emit('open-repair')"
          >
            <el-icon><FirstAidKit /></el-icon>
          </el-button>

          <el-button
            class="icon-only"
            :title="t('header.settings')"
            :aria-label="t('header.settings')"
            @click="$emit('open-settings')"
          >
            <el-icon><Tools /></el-icon>
          </el-button>

          <el-button
            class="icon-only"
            :title="t('header.website')"
            :aria-label="t('header.website')"
            @click="$emit('open-website')"
          >
            <el-icon><Link /></el-icon>
          </el-button>
        </div>
      </div>
    </div>

    <!-- 状态栏 -->
    <div class="status-bar" v-if="lastScanTime">
      <span class="scan-time">
        <el-icon><Clock /></el-icon>
        {{ t('header.lastScan') }} {{ lastScanTime }}
      </span>
      <span class="directory-path" :title="pluginDirectory">
        <el-icon><FolderOpened /></el-icon>
        {{ pluginDirectory }}
      </span>
    </div>
  </header>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import {
  Folder,
  Search,
  Refresh,
  Tools,
  Link,
  Clock,
  FolderOpened,
  MagicStick,
  FirstAidKit,
} from '@element-plus/icons-vue'
import { t, toggleLocale, locale } from '../i18n'

const props = defineProps<{
  pluginDirectory: string
  isScanning: boolean
  isCheckingUpdates: boolean
  autoScanning: boolean
  pluginCount: number
  updatableCount: number
  lastScanTime: string
}>()

const emit = defineEmits<{
  scan: [directory: string]
  'check-updates': []
  'auto-scan': []
  'open-settings': []
  'open-website': []
  'open-repair': []
}>()

const localDirectory = ref(props.pluginDirectory)

watch(
  () => props.pluginDirectory,
  (val) => {
    localDirectory.value = val
  }
)

function handleScan() {
  if (!localDirectory.value.trim()) {
    return
  }
  emit('scan', localDirectory.value.trim())
}

function handleAutoScan() {
  emit('auto-scan')
}
</script>

<style scoped>
.header-shell {
  position: sticky;
  top: 0;
  z-index: 100;
  flex-shrink: 0;
}

.header-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 20px;
  padding: 14px 24px;
  /* 玻璃拟态：由 .glass 提供背景/模糊/边框/阴影 */
}

/* ---------- 品牌区 ---------- */
.header-left {
  flex-shrink: 0;
}

.logo {
  display: flex;
  align-items: center;
  gap: 12px;
}

.logo-mark {
  width: 40px;
  height: 40px;
  border-radius: 12px;
  background: linear-gradient(135deg, var(--primary), var(--primary-light));
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 700;
  font-size: 13px;
  color: #fff;
  letter-spacing: 0.3px;
  box-shadow: 0 4px 14px rgba(99, 102, 241, 0.4);
}

.logo-text h1 {
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
  line-height: 1.25;
}

.subtitle {
  font-size: 11px;
  color: var(--text-muted);
}

/* ---------- 目录输入 ---------- */
.header-center {
  flex: 1;
  min-width: 0;
  max-width: 520px;
}

.directory-input {
  display: flex;
  gap: 8px;
}

.directory-input .el-input {
  flex: 1;
  min-width: 0;
}

.directory-input .el-input :deep(.el-input__wrapper) {
  border-radius: var(--radius-md);
}

.directory-input .el-button {
  flex-shrink: 0;
  border-radius: var(--radius-md);
}

/* ---------- 操作区 ---------- */
.header-right {
  display: flex;
  align-items: center;
  gap: 14px;
  flex-shrink: 0;
}

/* 统计胶囊 */
.stats {
  display: flex;
  gap: 8px;
}

.stat-pill {
  display: flex;
  flex-direction: column;
  align-items: center;
  min-width: 52px;
  padding: 4px 10px;
  border-radius: var(--radius-sm);
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid var(--glass-border);
}

.stat-value {
  font-size: 16px;
  font-weight: 700;
  line-height: 1.2;
  color: var(--primary-light);
}

.stat-label {
  font-size: 11px;
  color: var(--text-muted);
}

.stat-pill.update {
  background: rgba(245, 158, 11, 0.12);
  border-color: rgba(245, 158, 11, 0.3);
}

.stat-pill.update .stat-value {
  color: var(--warning);
}

/* 按钮组 */
.action-group {
  display: flex;
  align-items: center;
  gap: 8px;
}

.action-group .el-button {
  border-radius: var(--radius-md);
  font-weight: 500;
}

/* 非主按钮：玻璃描边风格 */
.action-group .el-button:not(.el-button--primary) {
  background: rgba(255, 255, 255, 0.04);
  border: 1px solid var(--glass-border);
  color: var(--text-secondary);
}

.action-group .el-button:not(.el-button--primary):hover {
  background: rgba(255, 255, 255, 0.1);
  border-color: rgba(255, 255, 255, 0.22);
  color: var(--text-primary);
}

/* 纯图标按钮 */
.icon-only {
  padding: 8px 10px;
}

/* ---------- 状态栏 ---------- */
.status-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  padding: 7px 24px;
  background: rgba(15, 23, 42, 0.85);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
  border-top: 1px solid rgba(255, 255, 255, 0.06);
  font-size: 12px;
  color: var(--text-muted);
}

.scan-time,
.directory-path {
  display: flex;
  align-items: center;
  gap: 5px;
  min-width: 0;
}

.directory-path {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 52%;
}

/* ---------- 窄屏适配（渐进增强：小屏隐藏次要元素） ---------- */
@media (max-width: 1280px) {
  .action-group .el-button :deep(span) {
    /* 保持文字，但收紧内边距 */
  }
  .action-group {
    gap: 6px;
  }
}

@media (max-width: 1080px) {
  .logo-text,
  .stats {
    display: none;
  }
  .header-center {
    max-width: none;
  }
}
.lang-label {
  font-size: 12px;
  font-weight: 600;
  letter-spacing: 0.5px;
}
</style>
