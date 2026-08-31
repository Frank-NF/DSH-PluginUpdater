<template>
  <WDialog
    v-model="visible"
    :title="t('mcp.title')"
    wide
    :close-on-mask="false"
  >
    <details class="mcp-help">
      <summary>{{ t('mcp.helpTitle') }}</summary>
      <ol>
        <li>{{ t('mcp.help1') }}</li>
        <li>{{ t('mcp.help2') }}</li>
        <li>{{ t('mcp.help3') }}</li>
        <li>{{ t('mcp.help4') }}</li>
      </ol>
    </details>

    <WLoading v-if="loading" block :text="t('mcp.loading')" />
    <template v-else>
      <div v-if="!data?.configExists" class="w-callout w-callout_warn">{{ t('mcp.noConfig') }}</div>

      <!-- 启用中 -->
      <p v-if="data?.enabled.length" class="w-label">{{ t('mcp.enabled') }}</p>
      <div
        v-for="s in data?.enabled || []"
        :key="s.serverId"
        class="weui-cells w-cells mcp-card"
      >
        <div class="mcp-head">
          <div class="mcp-title">
            <strong>{{ s.name }}</strong>
            <span class="mono w-text-2">{{ s.serverId }}</span>
            <span class="w-tag w-tag_info">{{ s.transport }}</span>
          </div>
          <div class="mcp-ops">
            <WButton size="mini" icon="refresh" :loading="probing === s.serverId" @click="onProbe(s)">
              {{ t('mcp.probe') }}
            </WButton>
            <WButton size="mini" :title="t('mcp.disable')" @click="onToggle(s, false)">{{ t('mcp.disable') }}</WButton>
          </div>
        </div>
        <p class="w-cell-desc mono">
          {{ s.transport === 'stdio' ? [s.command, ...(s.args || [])].join(' ') : s.url }}
        </p>
        <p v-if="probeResult?.serverId === s.serverId" class="mcp-probe" :class="probeResult.ok ? 'is-ok' : 'is-fail'">
          {{ probeResult.ok ? '✓' : '✗' }} {{ probeResult.detail }} · {{ probeResult.latencyMs }}ms
        </p>
        <!-- env 键值 -->
        <div v-if="s.envKeys.length" class="mcp-env">
          <div v-for="k in s.envKeys" :key="k.key" class="mcp-env-row">
            <span class="mono mcp-env-key">{{ k.key }}</span>
            <input
              v-model="envInputs[s.serverId + '/' + k.key]"
              class="weui-input mono"
              type="password"
              autocomplete="off"
              :placeholder="k.hasSecret ? t('mcp.maskedStored') : t('mcp.enterValue')"
            />
            <WButton
              size="mini"
              type="primary"
              :disabled="!envInputs[s.serverId + '/' + k.key]"
              @click="onSaveEnv(s, k.key)"
            >
              {{ t('mcp.save') }}
            </WButton>
            <span v-if="k.hasSecret" class="w-tag w-tag_success">{{ t('mcp.storedTag') }}</span>
          </div>
          <p class="w-cell-desc">{{ t('mcp.envTip') }}</p>
        </div>
      </div>

      <!-- 已禁用 -->
      <template v-if="data?.disabled.length">
        <p class="w-label" style="margin-top: 16px">{{ t('mcp.disabled') }}</p>
        <div
          v-for="s in data.disabled"
          :key="s.serverId"
          class="weui-cells w-cells mcp-card is-disabled"
        >
          <div class="mcp-head">
            <div class="mcp-title">
              <strong>{{ s.name }}</strong>
              <span class="mono w-text-2">{{ s.serverId }}</span>
            </div>
            <WButton size="mini" type="primary" @click="onToggle(s, true)">{{ t('mcp.enable') }}</WButton>
          </div>
        </div>
      </template>

      <div v-if="data?.enabled.length" class="mcp-footer">
        <p class="w-cell-desc">{{ t('mcp.applyTip') }}</p>
        <WButton type="primary" :loading="applying" @click="onApply">{{ t('mcp.apply') }}</WButton>
      </div>
      <p v-if="applyNote" class="w-cell-desc mcp-apply-note">{{ applyNote }}</p>
    </template>
  </WDialog>
</template>

<script setup lang="ts">
import { ref, reactive, watch } from 'vue'
import WButton from './WButton.vue'
import WDialog from './WDialog.vue'
import WLoading from './WLoading.vue'
import { mcpApi } from '../api/mcp'
import { t } from '../i18n'
import type { McpListResult, McpEntryInfo } from '../types'

const visible = defineModel<boolean>({ default: false })

const loading = ref(false)
const data = ref<McpListResult | null>(null)
const probing = ref('')
const probeResult = ref<{ serverId: string; ok: boolean; detail: string; latencyMs: number } | null>(null)
const applying = ref(false)
const applyNote = ref('')
const envInputs = reactive<Record<string, string>>({})

async function load() {
  loading.value = true
  try {
    data.value = await mcpApi.list()
  } catch (e) {
    data.value = { enabled: [], disabled: [], configExists: false }
    applyNote.value = String(e)
  } finally {
    loading.value = false
  }
}

watch(visible, (v) => {
  if (v) {
    load()
  } else {
    probeResult.value = null
    applyNote.value = ''
  }
})

async function onProbe(s: McpEntryInfo) {
  probing.value = s.serverId
  probeResult.value = null
  try {
    const r = await mcpApi.probe(s.serverId)
    probeResult.value = { serverId: s.serverId, ok: r.ok, detail: r.detail, latencyMs: r.latencyMs }
  } catch (e) {
    probeResult.value = { serverId: s.serverId, ok: false, detail: String(e), latencyMs: 0 }
  } finally {
    probing.value = ''
  }
}

async function onSaveEnv(s: McpEntryInfo, key: string) {
  const value = envInputs[s.serverId + '/' + key]
  if (!value) return
  try {
    await mcpApi.saveEnv(s.serverId, key, value)
    envInputs[s.serverId + '/' + key] = ''
    await load()
  } catch (e) {
    applyNote.value = String(e)
  }
}

async function onToggle(s: McpEntryInfo, enable: boolean) {
  try {
    await mcpApi.toggle(s.serverId, enable)
    await load()
  } catch (e) {
    applyNote.value = String(e)
  }
}

async function onApply() {
  applying.value = true
  applyNote.value = ''
  try {
    const n = await mcpApi.applyConfig()
    applyNote.value = t('mcp.appliedNote').replace('{n}', String(n))
  } catch (e) {
    applyNote.value = String(e)
  } finally {
    applying.value = false
  }
}
</script>

<style scoped>
.mcp-help {
  margin-bottom: 12px;
  padding: 8px 12px;
  border: 1px solid var(--border);
  border-radius: var(--r-sm);
  font-size: 12px;
  color: var(--fg-2);
}

.mcp-help summary {
  cursor: pointer;
  font-weight: 600;
  color: var(--fg);
}

.mcp-help ol {
  margin: 8px 0 0;
  padding-left: 18px;
  display: grid;
  gap: 4px;
}

.mcp-card {
  margin-bottom: 12px;
  padding: 12px 14px;
}

.mcp-card.is-disabled {
  opacity: 0.6;
}

.mcp-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  flex-wrap: wrap;
}

.mcp-title {
  display: flex;
  align-items: center;
  gap: 8px;
}

.mcp-ops {
  display: flex;
  gap: 6px;
}

.mcp-probe {
  font-size: 12px;
  margin-top: 6px;
}

.mcp-probe.is-ok {
  color: #10b981;
}

.mcp-probe.is-fail {
  color: #f87171;
}

.mcp-env {
  margin-top: 10px;
  padding-top: 10px;
  border-top: 1px dashed var(--border);
}

.mcp-env-row {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
}

.mcp-env-key {
  min-width: 120px;
  font-size: 12px;
}

.mcp-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  margin-top: 16px;
}

.mcp-apply-note {
  margin-top: 8px;
}
</style>
