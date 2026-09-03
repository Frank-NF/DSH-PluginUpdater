<script setup lang="ts">
import { ref } from 'vue'
import WDialog from './WDialog.vue'
import WButton from './WButton.vue'
import WLoading from './WLoading.vue'
import { t } from '../i18n'
import { useToast } from '../composables/useToast'
import { offlineApply, offlinePack, pickFile, pickSaveFile, snapshotApply, snapshotExport, snapshotPreview } from '../api/snapshot'
import type { SnapshotApplyItem, SnapshotDiff } from '../types'

const model = defineModel<boolean>({ default: false })
const toast = useToast()
const busy = ref('')
const diff = ref<SnapshotDiff | null>(null)
const applied = ref<SnapshotApplyItem[] | null>(null)

async function guard(fn: () => Promise<void>, key: string) {
  if (busy.value) return
  busy.value = key
  try {
    await fn()
  } catch (e) {
    toast.error(String(e))
  } finally {
    busy.value = ''
  }
}

const doExport = () => guard(async () => {
  const path = await pickSaveFile(t('snapshot.exportTitle'), 'dsh-snapshot.json', ['json'])
  if (!path) return
  const s = await snapshotExport(path)
  toast.success(t('snapshot.exported', { n: String(s.count), p: s.path }))
}, 'export')

const doPreview = () => guard(async () => {
  const path = await pickFile(t('snapshot.importTitle'), ['json'])
  if (!path) return
  applied.value = null
  diff.value = await snapshotPreview(path)
}, 'preview')

const doApply = () => guard(async () => {
  const path = await pickFile(t('snapshot.importTitle'), ['json'])
  if (!path) return
  const items = await snapshotApply(path)
  applied.value = items
  const ok = items.filter((i) => i.status === 'installed').length
  toast.success(t('snapshot.applied', { n: String(ok) }))
}, 'apply')

const doPack = () => guard(async () => {
  const path = await pickSaveFile(t('snapshot.packTitle'), 'dsh-offline-plugins.zip', ['zip'])
  if (!path) return
  const s = await offlinePack(path)
  toast.success(t('snapshot.packed', { n: String(s.plugins), p: s.path }))
}, 'pack')

const doRestore = () => guard(async () => {
  const path = await pickFile(t('snapshot.restoreTitle'), ['zip'])
  if (!path) return
  const n = await offlineApply(path)
  toast.success(t('snapshot.restored', { n: String(n) }))
}, 'restore')
</script>

<template>
  <WDialog v-model="model" :title="t('snapshot.title')" wide>
    <div class="snap-grid">
      <section class="snap-card">
        <h4>{{ t('snapshot.exportTitle') }}</h4>
        <p>{{ t('snapshot.exportDesc') }}</p>
        <WButton :disabled="!!busy" :loading="busy === 'export'" @click="doExport">{{ t('snapshot.exportBtn') }}</WButton>
      </section>

      <section class="snap-card">
        <h4>{{ t('snapshot.importTitle') }}</h4>
        <p>{{ t('snapshot.importDesc') }}</p>
        <div class="snap-actions">
          <WButton size="mini" :disabled="!!busy" @click="doPreview">{{ t('snapshot.previewBtn') }}</WButton>
          <WButton size="mini" :disabled="!!busy" @click="doApply">{{ t('snapshot.applyBtn') }}</WButton>
        </div>
        <div v-if="diff" class="snap-diff">
          <p>{{ t('snapshot.diffLine', { s: String(diff.snapshotCount), c: String(diff.currentCount), ok: String(diff.okCount), m: String(diff.missing.length), v: String(diff.versionMismatch.length) }) }}</p>
          <ul v-if="diff.missing.length">
            <li v-for="m in diff.missing" :key="m.id">{{ m.name }} <span class="mono">{{ m.version }}</span></li>
          </ul>
        </div>
        <div v-if="applied" class="snap-diff">
          <p v-for="a in applied" :key="a.id" :class="['snap-status', a.status]">
            {{ a.name }} — {{ a.status }}: {{ a.detail }}
          </p>
        </div>
      </section>

      <section class="snap-card">
        <h4>{{ t('snapshot.packTitle') }}</h4>
        <p>{{ t('snapshot.packDesc') }}</p>
        <WButton :disabled="!!busy" :loading="busy === 'pack'" @click="doPack">{{ t('snapshot.packBtn') }}</WButton>
      </section>

      <section class="snap-card">
        <h4>{{ t('snapshot.restoreTitle') }}</h4>
        <p>{{ t('snapshot.restoreDesc') }}</p>
        <WButton :disabled="!!busy" :loading="busy === 'restore'" @click="doRestore">{{ t('snapshot.restoreBtn') }}</WButton>
      </section>
    </div>
    <WLoading v-if="busy" block :text="t('snapshot.working')" />
  </WDialog>
</template>

<style scoped>
.snap-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px;
}

.snap-card {
  border: 1px solid var(--border);
  border-radius: var(--r-md);
  padding: 12px 14px;
  display: grid;
  gap: 8px;
  justify-items: start;
}

.snap-card h4 {
  margin: 0;
  font-size: 13px;
}

.snap-card p {
  margin: 0;
  font-size: 12px;
  color: var(--fg-2);
}

.snap-actions {
  display: flex;
  gap: 8px;
}

.snap-diff {
  font-size: 12px;
  width: 100%;
}

.snap-diff ul {
  margin: 4px 0 0;
  padding-left: 18px;
}

.snap-status.installed { color: var(--ok, #07c160); }
.snap-status.failed { color: var(--danger, #fa5151); }
.snap-status.skipped { color: var(--fg-2); }
</style>
