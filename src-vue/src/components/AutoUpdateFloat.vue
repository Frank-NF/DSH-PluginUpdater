<template>
  <Teleport to="body">
    <Transition name="float-slide">
      <div v-if="show" class="auto-update-float" @click.stop>
        <!-- 关闭按钮 -->
        <button class="float-close" @click="handleClose">×</button>

        <!-- 内容区 -->
        <div class="float-content">
          <!-- 版本信息 -->
          <div class="float-header">
            <WIcon name="refresh" :size="16" />
            <span class="float-title">发现新版本</span>
          </div>

          <div class="float-version">
            <span class="version-current">v{{ currentVersion }}</span>
            <WIcon name="arrow-right" :size="12" />
            <span class="version-latest">v{{ latestVersion }}</span>
          </div>

          <!-- 下载进度 -->
          <div v-if="state.download_phase !== 'idle' && state.download_phase !== 'done'" class="float-progress">
            <div class="progress-bar">
              <div class="progress-fill" :style="{ width: state.download_percent + '%' }"></div>
            </div>
            <span class="progress-text">{{ state.download_message || '正在下载...' }}</span>
          </div>

          <!-- 完成状态 -->
          <div v-else-if="state.is_downloaded" class="float-done">
            <WIcon name="check-circle" :size="20" />
            <span>下载完成，准备安装</span>
          </div>

          <!-- 操作按钮 -->
          <div class="float-actions">
            <WButton
              v-if="state.is_downloaded && installPath"
              type="primary"
              icon="download"
              @click="handleInstall"
            >
              立即安装
            </WButton>
            <WButton
              v-else
              size="mini"
              @click="handleClose"
            >
              稍后提醒
            </WButton>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { usePluginStore } from '../stores/pluginStore'
import WIcon from './WIcon.vue'
import WButton from './WButton.vue'

const pluginStore = usePluginStore()

const show = computed(() => pluginStore.showAutoUpdateFloat && pluginStore.autoUpdateState?.available)
const state = computed(() => pluginStore.autoUpdateState!)
const currentVersion = computed(() => state.value?.current_version || '')
const latestVersion = computed(() => state.value?.latest_version || '')
const installPath = computed(() => pluginStore.autoUpdateInstallPath)

function handleClose() {
  pluginStore.showAutoUpdateFloat = false
}

async function handleInstall() {
  if (installPath.value) {
    try {
      await pluginStore.launchAutoUpdate(installPath.value)
    } catch (e) {
      console.error('安装失败:', e)
    }
  }
}
</script>

<style scoped>
.auto-update-float {
  position: fixed;
  bottom: 24px;
  right: 24px;
  width: 320px;
  background: rgba(15, 23, 42, 0.95);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
  border-radius: 16px;
  border: 1px solid rgba(99, 102, 241, 0.3);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4), 0 0 0 1px rgba(255, 255, 255, 0.05);
  z-index: 1000;
  overflow: hidden;
}

.float-close {
  position: absolute;
  top: 12px;
  right: 12px;
  width: 24px;
  height: 24px;
  border: none;
  background: rgba(255, 255, 255, 0.1);
  color: rgba(255, 255, 255, 0.6);
  border-radius: 50%;
  cursor: pointer;
  font-size: 16px;
  line-height: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.float-close:hover {
  background: rgba(255, 255, 255, 0.2);
  color: #fff;
}

.float-content {
  padding: 16px;
}

.float-header {
  display: flex;
  align-items: center;
  gap: 8px;
  color: #6366F1;
  font-size: 13px;
  font-weight: 600;
  margin-bottom: 12px;
}

.float-title {
  color: #e2e8f0;
}

.float-version {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 16px;
  font-size: 14px;
}

.version-current {
  color: rgba(255, 255, 255, 0.5);
  text-decoration: line-through;
}

.version-latest {
  color: #10b981;
  font-weight: 600;
}

.float-progress {
  margin-bottom: 16px;
}

.progress-bar {
  height: 4px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 2px;
  overflow: hidden;
  margin-bottom: 8px;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, #6366F1, #8b5cf6);
  border-radius: 2px;
  transition: width 0.3s ease;
}

.progress-text {
  font-size: 12px;
  color: rgba(255, 255, 255, 0.5);
}

.float-done {
  display: flex;
  align-items: center;
  gap: 8px;
  color: #10b981;
  font-size: 13px;
  margin-bottom: 16px;
}

.float-actions {
  display: flex;
  gap: 8px;
}

/* 动画 */
.float-slide-enter-active,
.float-slide-leave-active {
  transition: all 0.3s ease;
}

.float-slide-enter-from,
.float-slide-leave-to {
  opacity: 0;
  transform: translateY(20px) scale(0.95);
}
</style>
