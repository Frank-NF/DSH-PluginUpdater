<template>
  <WDialog
    v-model="visible"
    :title="t('serverPanel.title')"
    wide
  >
    <div class="sp-body">
      <!-- 状态卡 -->
      <div class="sp-status-card" :class="status?.running ? 'is-running' : 'is-stopped'">
        <div class="sp-status-indicator">
          <span class="sp-dot" :class="{ 'is-on': status?.running, 'is-busy': busy }"></span>
          <span class="sp-status-text">{{ statusText }}</span>
        </div>
        <div class="sp-status-detail">{{ status?.message || t('serverPanel.statusUnknown') }}</div>
        <div v-if="status?.running" class="sp-url-row" :class="{ 'is-auth': !!status.auth_url }">
          <WIcon name="link" :size="13" />
          <span class="sp-url">{{ status.auth_url || status.url }}</span>
          <WButton size="mini" icon="copy" :title="t('serverPanel.copyUrl')" @click="copyUrl">
            {{ t('serverPanel.copyUrl') }}
          </WButton>
        </div>
        <div v-if="status?.running && status.auth_url" class="sp-auth-hint">
          {{ t('serverPanel.authHint') }}
        </div>
      </div>

      <!-- 进程信息 -->
      <div v-if="status?.pid" class="sp-meta-row">
        <WIcon name="server" :size="13" />
        <span>PID {{ status.pid }} · {{ status.process_name || 'node' }}</span>
      </div>

      <!-- 操作按钮 -->
      <div class="sp-actions">
        <WButton
          type="primary"
          icon="zap"
          :loading="busy && busyAction === 'start'"
          :disabled="busy"
          @click="handleStart"
        >
          {{ t('serverPanel.start') }}
        </WButton>
        <WButton
          icon="power"
          :loading="busy && busyAction === 'stop'"
          :disabled="busy"
          @click="handleStop"
        >
          {{ t('serverPanel.stop') }}
        </WButton>
        <WButton
          icon="refresh"
          :loading="busy && busyAction === 'restart'"
          :disabled="busy"
          @click="handleRestart"
        >
          {{ t('serverPanel.restart') }}
        </WButton>
      </div>

      <!-- 使用提示 -->
      <div class="sp-tips">
        <WIcon name="info" :size="14" />
        <span>{{ tipsText }}</span>
      </div>
    </div>
  </WDialog>
</template>

<script setup lang="ts">
import { computed, onUnmounted, ref, watch } from 'vue'
import { t } from '../i18n'
import { serverApi, type ServerStatus } from '../api'
import { useToast } from '../composables/useToast'
import { fadeSlideIn } from '../composables/useMotion'
import WDialog from './WDialog.vue'
import WButton from './WButton.vue'
import WIcon from './WIcon.vue'

const props = defineProps<{ modelValue: boolean }>()
const emit = defineEmits<{ (e: 'update:modelValue', v: boolean): void }>()

const visible = computed({
  get: () => props.modelValue,
  set: (v: boolean) => emit('update:modelValue', v),
})

const toast = useToast()
const status = ref<ServerStatus | null>(null)
const busy = ref(false)
const busyAction = ref<'start' | 'stop' | 'restart' | ''>('')
let pollTimer: ReturnType<typeof setInterval> | null = null

const statusText = computed(() => {
  if (busy.value) return t('serverPanel.processing')
  if (!status.value) return t('serverPanel.statusUnknown')
  if (status.value.running && status.value.port_open) return t('serverPanel.running')
  if (status.value.running && !status.value.port_open) return t('serverPanel.starting')
  return t('serverPanel.stopped')
})

const tipsText = computed(() =>
  t('serverPanel.tips', { port: status.value?.port ?? 3081 })
)

async function refreshStatus() {
  try {
    status.value = await serverApi.status()
  } catch {
    status.value = null
  }
}

async function runAction(action: 'start' | 'stop' | 'restart') {
  busy.value = true
  busyAction.value = action
  try {
    // 启动/重启沿用当前探测到的端口（用户常用 3081；官方默认 3080；8787 为旧版遗留）
    const currentPort = status.value?.port_open ? status.value?.port : undefined
    const msg =
      action === 'stop'
        ? await serverApi.stop()
        : action === 'start'
          ? await serverApi.start(currentPort)
          : await serverApi.restart(currentPort)
    toast.success(msg || t(`serverPanel.${action}Ok`))
    // 立即刷新一次；启动后轮询等待端口就绪
    await refreshStatus()
    if (action !== 'stop' && status.value && !status.value.port_open) {
      startPolling()
    }
  } catch (e) {
    toast.error(e instanceof Error ? e.message : t(`serverPanel.${action}Fail`))
  } finally {
    busy.value = false
    busyAction.value = ''
  }
}

function startPolling() {
  stopPolling()
  let tries = 0
  pollTimer = setInterval(async () => {
    tries += 1
    await refreshStatus()
    if (status.value?.port_open || tries >= 10) {
      stopPolling()
    }
  }, 1000)
}

function stopPolling() {
  if (pollTimer) {
    clearInterval(pollTimer)
    pollTimer = null
  }
}

function handleStart() {
  runAction('start')
}
function handleStop() {
  runAction('stop')
}
function handleRestart() {
  runAction('restart')
}

async function copyUrl() {
  if (!status.value?.running) return
  // 优先复制带 token 的完整地址（裸地址会被 401 拦截）
  const url = status.value.auth_url || status.value.url
  if (!url) return
  try {
    await navigator.clipboard.writeText(url)
    toast.success(t('serverPanel.copied'))
  } catch {
    toast.error(t('serverPanel.copyFail'))
  }
}

// 打开时进场动效 + 轮询
watch(
  () => props.modelValue,
  (open) => {
    if (open) {
      refreshStatus()
      setTimeout(() => fadeSlideIn('.sp-actions .w-btn', { stagger: 0.035, delay: 0.1 }), 120)
      // 打开期间持续刷新状态（每 5 秒）
      stopPolling()
      pollTimer = setInterval(refreshStatus, 5000)
    } else {
      stopPolling()
    }
  }
)

onUnmounted(stopPolling)
</script>

<style scoped>
.sp-body {
  display: flex;
  flex-direction: column;
  gap: 14px;
}

/* 状态卡 */
.sp-status-card {
  padding: 18px 20px;
  border-radius: var(--radius-lg, 12px);
  border: 1px solid var(--glass-border, rgba(148, 163, 184, 0.2));
  background: var(--bg-card, rgba(30, 41, 59, 0.5));
  transition: border-color 0.24s ease;
}

.sp-status-card.is-running {
  border-color: rgba(16, 185, 129, 0.4);
}

.sp-status-card.is-stopped {
  border-color: rgba(100, 116, 139, 0.35);
}

.sp-status-indicator {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 6px;
}

.sp-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: #64748b;
  flex-shrink: 0;
  transition: background 0.24s ease;
}

.sp-dot.is-on {
  background: #10b981;
  box-shadow: 0 0 8px rgba(16, 185, 129, 0.7);
}

.sp-dot.is-busy {
  background: #f59e0b;
  box-shadow: 0 0 8px rgba(245, 158, 11, 0.6);
}

.sp-status-text {
  font-size: 16px;
  font-weight: 700;
  color: var(--fg-1, #e8edf5);
}

.sp-status-detail {
  font-size: 12px;
  color: var(--fg-3, #8da0b8);
  margin-bottom: 10px;
}

.sp-url-row {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  border-radius: 8px;
  background: rgba(16, 185, 129, 0.08);
  border: 1px solid rgba(16, 185, 129, 0.2);
}

.sp-url {
  flex: 1;
  font-size: 13px;
  font-family: 'JetBrains Mono', 'Consolas', monospace;
  color: var(--fg-2, #b6c2d4);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.sp-url-row.is-auth {
  background: rgba(99, 102, 241, 0.08);
  border-color: rgba(99, 102, 241, 0.3);
}

.sp-auth-hint {
  margin-top: 6px;
  font-size: 11px;
  line-height: 1.5;
  color: var(--fg-3, #8da0b8);
}

/* 进程信息行 */
.sp-meta-row {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
  color: var(--fg-3, #8da0b8);
}

/* 操作按钮 */
.sp-actions {
  display: flex;
  gap: 10px;
}

.sp-actions .w-btn {
  flex: 1;
}

/* 提示 */
.sp-tips {
  display: flex;
  gap: 8px;
  align-items: flex-start;
  padding: 10px 12px;
  border-radius: 8px;
  background: rgba(99, 102, 241, 0.08);
  font-size: 12px;
  line-height: 1.6;
  color: var(--fg-2, #b6c2d4);
}

.sp-tips .w-icon {
  margin-top: 2px;
  color: var(--primary, #6366f1);
}
</style>
