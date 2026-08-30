<template>
  <WDialog v-model="visible" :title="t('repair.title')" wide>
    <div class="w-repair">
      <!-- ============ 环境体检 ============ -->
      <section class="w-section">
        <div class="w-section-head">
          <h4><WIcon name="shield" :size="15" /> {{ t('repair.envTitle') }}</h4>
          <WButton
            size="mini"
            type="primary"
            icon="refresh"
            :loading="checking"
            @click="runCheck"
          >
            {{ t('repair.runCheck') }}
          </WButton>
        </div>

        <WLoading v-if="checking" block :text="t('repair.checking')" />

        <div v-else-if="envItems.length" class="weui-cells w-cells">
          <div v-for="item in envItems" :key="item.id" class="weui-cell w-env-item">
            <div class="weui-cell__hd">
              <span class="w-env-icon" :class="item.status">
                <WIcon :name="statusIcon(item.status)" :size="13" />
              </span>
            </div>
            <div class="weui-cell__bd">
              <p class="w-env-name">
                {{ item.name }}
                <span class="w-tag" :class="statusTag(item.status)">
                  {{ statusText(item.status) }}
                </span>
              </p>
              <p class="w-env-msg">{{ item.message }}</p>
              <p v-if="item.fix_hint" class="w-env-fix">
                <WIcon name="info" :size="12" />
                {{ item.fix_hint }}
              </p>
            </div>
          </div>
        </div>

        <p v-else class="w-hint">{{ t('repair.noData') }}</p>
      </section>

      <!-- ============ 常见报错修复指南 ============ -->
      <section class="w-section">
        <div class="w-section-head">
          <h4><WIcon name="wrench" :size="15" /> {{ t('repair.guideTitle') }}</h4>
        </div>

        <div
          class="weui-search-bar w-search"
          :class="{ 'weui-search-bar_focusing': searchFocused || !!search }"
        >
          <div class="weui-search-bar__form">
            <div class="weui-search-bar__box">
              <i class="weui-icon-search" aria-hidden="true" />
              <input
                v-model="search"
                type="search"
                class="weui-search-bar__input"
                :placeholder="t('repair.searchPlaceholder')"
                @focus="searchFocused = true"
                @blur="searchFocused = false"
              />
              <a
                v-if="search"
                href="javascript:"
                class="weui-icon-clear"
                :aria-label="t('common.clear')"
                @click="search = ''"
              />
            </div>
          </div>
        </div>

        <div class="w-guide-list">
          <div v-for="g in filteredGuides" :key="g.id" class="w-guide">
            <button
              type="button"
              class="w-guide-head"
              :aria-expanded="expanded === g.id"
              @click="toggleGuide(g.id)"
            >
              <span class="w-guide-arrow" :class="{ open: expanded === g.id }">
                <WIcon name="chevronRight" :size="14" />
              </span>
              <span class="w-guide-title">
                {{ locale === 'zh' ? g.title.zh : g.title.en }}
              </span>
            </button>

            <div v-show="expanded === g.id" :ref="(el) => setGuideRef(g.id, el)" class="w-guide-detail">
              <p class="w-guide-cause">
                <span class="w-guide-label">{{ t('repair.cause') }}:</span>
                {{ locale === 'zh' ? g.cause.zh : g.cause.en }}
              </p>
              <ol class="w-guide-steps">
                <li v-for="(s, i) in locale === 'zh' ? g.steps.zh : g.steps.en" :key="i">
                  {{ s }}
                </li>
              </ol>
            </div>
          </div>

          <p v-if="!filteredGuides.length" class="w-hint">{{ t('repair.noMatch') }}</p>
        </div>
      </section>
    </div>

    <template #footer>
      <WButton @click="visible = false">{{ t('common.close') }}</WButton>
    </template>
  </WDialog>
</template>

<script setup lang="ts">
import { computed, nextTick, ref, watch } from 'vue'
import WDialog from './WDialog.vue'
import WButton from './WButton.vue'
import WIcon from './WIcon.vue'
import WLoading from './WLoading.vue'
import { pluginApi } from '../api'
import { t, locale } from '../i18n'
import { REPAIR_GUIDES } from '../data/repairGuide'
import { prefersReducedMotion, DUR, EASE } from '../composables/useMotion'
import { gsap } from 'gsap'
import type { EnvCheckItem } from '../types'

const props = defineProps<{ modelValue: boolean }>()
const emit = defineEmits<{ 'update:modelValue': [value: boolean] }>()

const visible = ref(props.modelValue)
watch(
  () => props.modelValue,
  (v) => {
    visible.value = v
  }
)
watch(visible, (v) => emit('update:modelValue', v))

const envItems = ref<EnvCheckItem[]>([])
const checking = ref(false)
const search = ref('')
const searchFocused = ref(false)
const expanded = ref<string | null>(null)

async function runCheck() {
  checking.value = true
  try {
    envItems.value = await pluginApi.checkEnvironment()
  } catch {
    envItems.value = []
  } finally {
    checking.value = false
  }
}

function statusIcon(s: string) {
  if (s === 'ok') return 'check'
  if (s === 'warn') return 'alert'
  return 'close'
}

function statusText(s: string) {
  if (s === 'ok') return t('repair.statusOk')
  if (s === 'warn') return t('repair.statusWarn')
  return t('repair.statusError')
}

function statusTag(s: string) {
  if (s === 'ok') return 'w-tag_success'
  if (s === 'warn') return 'w-tag_warn'
  return 'w-tag_danger'
}

const filteredGuides = computed(() => {
  const q = search.value.trim().toLowerCase()
  if (!q) return REPAIR_GUIDES
  return REPAIR_GUIDES.filter((g) => {
    const haystack = [
      g.title.zh,
      g.title.en,
      g.cause.zh,
      g.cause.en,
      ...g.keywords,
    ]
      .join(' ')
      .toLowerCase()
    return haystack.includes(q)
  })
})

/* ---------- 折叠动画（GSAP，支持降级） ---------- */
const guideEls = new Map<string, HTMLElement>()

function setGuideRef(id: string, el: unknown) {
  if (el instanceof HTMLElement) guideEls.set(id, el)
  else guideEls.delete(id)
}

function toggleGuide(id: string) {
  const next = expanded.value === id ? null : id
  expanded.value = next

  if (!next) return
  nextTick(() => {
    const el = guideEls.get(id)
    if (!el || prefersReducedMotion()) return
    gsap.fromTo(
      el,
      { height: 0, opacity: 0 },
      {
        height: 'auto',
        opacity: 1,
        duration: DUR.base,
        ease: EASE.out,
        clearProps: 'height,opacity',
      }
    )
  })
}
</script>

<style scoped>
.w-repair {
  max-height: 62vh;
  overflow-y: auto;
  padding-right: 4px;
}

.w-section + .w-section {
  margin-top: var(--sp-6);
  padding-top: var(--sp-4);
  border-top: 1px solid var(--border);
}

.w-section-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--sp-3);
  margin-bottom: var(--sp-3);
}

.w-section-head h4 {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 14px;
  font-weight: 600;
  color: var(--fg);
}

/* ---------- 环境体检 ---------- */
.w-cells {
  border-radius: var(--r-md);
  overflow: hidden;
}

.w-cells::before,
.w-cells::after {
  display: none;
}

.w-env-item {
  align-items: flex-start;
}

.w-env-icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 22px;
  height: 22px;
  border-radius: 50%;
  margin-right: var(--sp-2);
  flex-shrink: 0;
  color: #fff;
}

.w-env-icon.ok {
  background: var(--c-success);
}

.w-env-icon.warn {
  background: var(--c-warn);
}

.w-env-icon.error {
  background: var(--c-danger);
}

.w-env-name {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
  flex-wrap: wrap;
  font-size: 13px;
  font-weight: 600;
  color: var(--fg);
}

.w-env-msg {
  font-size: 12px;
  color: var(--fg-1);
  margin-top: 2px;
  word-break: break-all;
  line-height: 1.55;
}

.w-env-fix {
  display: flex;
  align-items: flex-start;
  gap: 4px;
  font-size: 12px;
  color: var(--c-warn);
  margin-top: 4px;
  line-height: 1.55;
}

.w-hint {
  padding: var(--sp-4);
  text-align: center;
  font-size: 13px;
  color: var(--fg-2);
}

/* ---------- 修复指南 ---------- */
.w-search {
  padding: 0;
  background: transparent;
  margin-bottom: var(--sp-3);
}

.w-search::before,
.w-search::after {
  display: none;
}

.w-search .weui-search-bar__form,
.w-search .weui-search-bar__box {
  border-radius: var(--r-md);
}

.w-guide-list {
  display: flex;
  flex-direction: column;
  gap: var(--sp-2);
}

.w-guide {
  border: 1px solid var(--border);
  border-radius: var(--r-md);
  overflow: hidden;
  background: var(--bg-group);
}

.w-guide-head {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
  width: 100%;
  padding: 11px var(--sp-3);
  text-align: left;
  font-size: 13px;
  font-weight: 500;
  color: var(--fg);
  transition: background-color 0.2s var(--ease-out);
}

.w-guide-head:hover {
  background: var(--bg-hover);
}

.w-guide-arrow {
  display: inline-flex;
  color: var(--fg-2);
  transition: transform 0.24s var(--ease-out);
  flex-shrink: 0;
}

.w-guide-arrow.open {
  transform: rotate(90deg);
}

.w-guide-title {
  flex: 1;
  min-width: 0;
}

.w-guide-detail {
  overflow: hidden;
  padding: 0 var(--sp-3) var(--sp-3) calc(var(--sp-3) + 22px);
  font-size: 12px;
}

.w-guide-cause {
  color: var(--fg-1);
  margin-bottom: var(--sp-2);
  line-height: 1.65;
}

.w-guide-label {
  color: var(--c-warn);
  font-weight: 600;
}

.w-guide-steps {
  margin: 0;
  padding-left: 18px;
  line-height: 1.85;
  color: var(--fg);
}

@media (prefers-reduced-motion: reduce) {
  .w-guide-head,
  .w-guide-arrow {
    transition: none;
  }
}
</style>
