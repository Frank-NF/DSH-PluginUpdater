<template>
  <header class="w-header">
    <div class="w-header__inner">
      <!-- 品牌 -->
      <div class="w-header__brand">
        <div class="w-logo" aria-hidden="true">
          <WIcon name="package" :size="20" />
        </div>
        <div class="w-header__titles w-hide-mobile">
          <h1>{{ t('header.title') }}</h1>
          <p>{{ t('header.subtitle') }}</p>
        </div>
      </div>

      <!-- 插件目录输入 + 扫描 -->
      <div class="w-header__search">
        <div class="weui-search-bar weui-search-bar_focusing">
          <div class="weui-search-bar__form">
            <div class="weui-search-bar__box">
              <i class="weui-icon-search" aria-hidden="true" />
              <input
                v-model="localDirectory"
                type="search"
                class="weui-search-bar__input"
                :placeholder="t('header.dirPlaceholder')"
                :aria-label="t('header.dirPlaceholder')"
                @keyup.enter="handleScan"
              />
              <a
                v-if="localDirectory"
                href="javascript:"
                class="weui-icon-clear"
                :aria-label="t('common.clear')"
                @click="clearDir"
              />
            </div>
          </div>
        </div>

        <WButton
          type="primary"
          icon="search"
          :loading="isScanning"
          @click="handleScan"
        >
          {{ t('header.scan') }}
        </WButton>
      </div>

      <!-- 操作区 -->
      <div class="w-header__actions">
        <!-- 统计（桌面） -->
        <div v-if="pluginCount > 0" class="w-header__stats w-hide-mobile">
          <div class="w-header__stat">
            <span class="w-header__stat-value">{{ pluginCount }}</span>
            <span class="w-header__stat-label">{{ t('header.plugins') }}</span>
          </div>
          <div v-if="updatableCount > 0" class="w-header__stat is-warn">
            <span class="w-header__stat-value">{{ updatableCount }}</span>
            <span class="w-header__stat-label">{{ t('header.updatable') }}</span>
          </div>
        </div>

        <!-- 主要操作 -->
        <WButton
          icon="wand"
          :loading="autoScanning"
          :title="t('header.autoScan')"
          @click="$emit('auto-scan')"
        >
          <span class="w-hide-mobile">{{ t('header.autoScan') }}</span>
        </WButton>

        <WButton
          type="primary"
          icon="refresh"
          :loading="isCheckingUpdates"
          :disabled="pluginCount === 0"
          :title="t('header.checkUpdates')"
          @click="$emit('check-updates')"
        >
          <span class="w-hide-mobile">{{ t('header.checkUpdates') }}</span>
        </WButton>

        <!-- 次要操作：桌面直接展示图标按钮 -->
        <div class="w-header__icon-group w-hide-mobile">
          <WButton
            size="mini"
            :icon="themeIcon"
            :title="t('header.theme')"
            :aria-label="t('header.theme')"
            @click="$emit('toggle-theme')"
          />
          <WButton
            size="mini"
            icon="wrench"
            :title="t('repair.title')"
            :aria-label="t('repair.title')"
            @click="$emit('open-repair')"
          />
          <WButton
            size="mini"
            icon="settings"
            :title="t('header.settings')"
            :aria-label="t('header.settings')"
            @click="$emit('open-settings')"
          />
          <WButton
            size="mini"
            icon="globe"
            :title="t('header.website')"
            :aria-label="t('header.website')"
            @click="$emit('open-website')"
          />
          <WButton size="mini" :title="t('header.langSwitch')" @click="$emit('toggle-locale')">
            {{ t('header.langSwitch') }}
          </WButton>
        </div>

        <!-- 移动端：更多 -->
        <WButton
          class="w-hide-desktop"
          icon="more"
          :title="t('common.more')"
          :aria-label="t('common.more')"
          @click="sheetOpen = true"
        />
      </div>
    </div>

    <!-- 状态栏 -->
    <div v-if="lastScanTime" class="w-header__status">
      <span class="w-status-item">
        <WIcon name="clock" :size="13" />
        <span class="w-hide-mobile">{{ t('header.lastScan') }}</span>
        {{ lastScanTime }}
      </span>
      <span class="w-status-item w-truncate" :title="pluginDirectory">
        <WIcon name="folder" :size="13" />
        <span class="w-truncate">{{ pluginDirectory }}</span>
      </span>
    </div>

    <!-- 移动端更多操作 -->
    <WSheet
      v-model="sheetOpen"
      :title="t('common.more')"
      :items="sheetItems"
      @select="onSheetSelect"
    />
  </header>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import WButton from './WButton.vue'
import WIcon from './WIcon.vue'
import WSheet from './WSheet.vue'
import { t } from '../i18n'
import type { ThemeMode } from '../composables/useTheme'

const props = defineProps<{
  pluginDirectory: string
  isScanning: boolean
  isCheckingUpdates: boolean
  autoScanning: boolean
  pluginCount: number
  updatableCount: number
  lastScanTime: string
  theme: ThemeMode
}>()

const emit = defineEmits<{
  scan: [directory: string]
  'check-updates': []
  'auto-scan': []
  'open-settings': []
  'open-website': []
  'open-repair': []
  'toggle-locale': []
  'toggle-theme': []
}>()

const localDirectory = ref(props.pluginDirectory)
const sheetOpen = ref(false)

watch(
  () => props.pluginDirectory,
  (val) => {
    localDirectory.value = val
  }
)

const themeIcon = computed(() => (props.theme === 'dark' ? 'sun' : 'moon'))

const sheetItems = computed(() => [
  { label: t('header.autoScan'), value: 'auto', desc: '' },
  { label: t('header.theme'), value: 'theme' },
  { label: t('repair.title'), value: 'repair' },
  { label: t('header.settings'), value: 'settings' },
  { label: t('header.website'), value: 'website' },
  { label: t('header.langSwitch'), value: 'locale' },
])

function handleScan() {
  const dir = localDirectory.value.trim()
  if (!dir) return
  emit('scan', dir)
}

function clearDir() {
  localDirectory.value = ''
}

function onSheetSelect(value: string) {
  switch (value) {
    case 'auto':
      emit('auto-scan')
      break
    case 'theme':
      emit('toggle-theme')
      break
    case 'repair':
      emit('open-repair')
      break
    case 'settings':
      emit('open-settings')
      break
    case 'website':
      emit('open-website')
      break
    case 'locale':
      emit('toggle-locale')
      break
  }
}
</script>

<style scoped>
.w-header {
  position: sticky;
  top: 0;
  z-index: 900;
  flex-shrink: 0;
  background: var(--bg-card);
  backdrop-filter: var(--glass-blur);
  -webkit-backdrop-filter: var(--glass-blur);
  border-bottom: 1px solid var(--border);
  box-shadow: var(--shadow);
}

.w-header__inner {
  display: flex;
  align-items: center;
  gap: var(--sp-4);
  flex-wrap: wrap;
  padding: 14px var(--sp-5);
}

/* ---------- 品牌 ---------- */
.w-header__brand {
  display: flex;
  align-items: center;
  gap: var(--sp-3);
  flex-shrink: 0;
}

.w-logo {
  width: 40px;
  height: 40px;
  border-radius: var(--r-md);
  background: linear-gradient(135deg, var(--brand) 0%, var(--brand-3) 100%);
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 4px 14px var(--brand-glow);
}

.w-header__titles h1 {
  font-size: 16px;
  font-weight: 700;
  line-height: 1.25;
  color: var(--fg);
  letter-spacing: 0.2px;
}

.w-header__titles p {
  font-size: 11px;
  color: var(--fg-2);
  line-height: 1.3;
}

/* ---------- 目录输入 ---------- */
.w-header__search {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
  flex: 1 1 280px;
  min-width: 0;
}

.w-header__search .weui-search-bar {
  flex: 1;
  min-width: 0;
  padding: 0;
  background: transparent;
  border-radius: var(--r-md);
}

.w-header__search .weui-search-bar::after,
.w-header__search .weui-search-bar::before {
  display: none;
}

.w-header__search .weui-search-bar__form {
  border-radius: var(--r-md);
  background: transparent;
}

.w-header__search .weui-search-bar__box {
  height: 40px;
  background: var(--bg-input);
  border: 1px solid var(--border);
  border-radius: var(--r-md);
}

.w-header__search .weui-search-bar__input {
  font-size: 13px;
  color: var(--fg);
}

/* ---------- 操作区 ---------- */
.w-header__actions {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
  flex-wrap: wrap;
  margin-left: auto;
}

.w-header__stats {
  display: flex;
  gap: var(--sp-2);
  padding-right: var(--sp-2);
  border-right: 1px solid var(--border);
}

.w-header__stat {
  display: flex;
  align-items: baseline;
  gap: 4px;
  padding: 4px 10px;
  border-radius: var(--r-sm);
  background: var(--bg-group);
  border: 1px solid var(--border);
}

.w-header__stat.is-warn {
  background: rgba(245, 158, 11, 0.12);
  border-color: rgba(245, 158, 11, 0.25);
}

.w-header__stat-value {
  font-size: 14px;
  font-weight: 700;
  color: var(--fg);
}

.w-header__stat.is-warn .w-header__stat-value {
  color: var(--c-warn);
}

.w-header__stat-label {
  font-size: 11px;
  color: var(--fg-2);
}

.w-header__icon-group {
  display: flex;
  align-items: center;
  gap: var(--sp-1);
  padding: 3px;
  border-radius: var(--r-md);
  background: var(--bg-group);
  border: 1px solid var(--border);
}

/* 组内按钮（图标/EN 文字）统一 ghost 风格，消除双重视觉噪音 */
.w-header__icon-group .weui-btn {
  min-width: 32px;
  height: 32px;
  padding: 0 8px;
  font-size: 12px;
  font-weight: 700;
  letter-spacing: 0.5px;
  background: transparent;
  border: none;
  color: var(--fg-2);
  border-radius: var(--r-sm);
  box-shadow: none;
}

.w-header__icon-group .weui-btn:not(:disabled):hover {
  background: var(--bg-hover);
  color: var(--brand-2);
  box-shadow: none;
}

/* ---------- 状态栏 ---------- */
.w-header__status {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--sp-4);
  padding: 7px var(--sp-5);
  background: rgba(0, 0, 0, 0.12);
  border-top: 1px solid var(--border);
  font-size: 11px;
  color: var(--fg-2);
}

[data-theme='light'] .w-header__status {
  background: rgba(255, 255, 255, 0.35);
}

.w-status-item {
  display: inline-flex;
  align-items: center;
  gap: 5px;
  min-width: 0;
}

.w-status-item:last-child {
  max-width: 52%;
}

@media (max-width: 767px) {
  .w-header__inner {
    padding: 10px var(--sp-4);
    gap: var(--sp-3);
  }

  .w-header__search {
    order: 3;
    flex-basis: 100%;
  }

  .w-header__actions {
    margin-left: auto;
  }

  .w-header__status {
    padding: 6px var(--sp-4);
  }
}
</style>
