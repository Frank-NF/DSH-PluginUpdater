<template>
  <div ref="rootEl" class="plugin-view">
    <!-- ============ 工具栏一行：Tab + 搜索 + 排序 + 视图 ============ -->
    <div class="w-toolbar-row">
      <div class="w-tabs">
        <a
          v-for="tab in tabs"
          :key="tab.name"
          href="javascript:"
          class="w-tab-btn"
          :class="{ 'is-active': activeTab === tab.name }"
          :aria-current="activeTab === tab.name ? 'page' : undefined"
          @click="switchTab(tab.name)"
        >
          {{ tab.label }}
        </a>
      </div>

      <template v-if="activeTab === 'market'">
        <div
          class="weui-search-bar w-search w-toolbar-row__search"
          :class="{ 'weui-search-bar_focusing': searchFocused || !!marketSearch }"
        >
          <div class="weui-search-bar__form">
            <div class="weui-search-bar__box">
              <i class="weui-icon-search" aria-hidden="true" />
              <input
                v-model="marketSearch"
                type="search"
                class="weui-search-bar__input"
                :placeholder="t('market.searchPlaceholder')"
                :aria-label="t('market.searchPlaceholder')"
                @focus="searchFocused = true"
                @blur="searchFocused = false"
              />
              <a
                v-if="marketSearch"
                href="javascript:"
                class="weui-icon-clear"
                :aria-label="t('common.clear')"
                @click="marketSearch = ''"
              />
            </div>
            <label v-if="!searchFocused && !marketSearch" class="weui-search-bar__label">
              <i class="weui-icon-search" aria-hidden="true" />
              <span>{{ t('market.searchPlaceholder') }}</span>
            </label>
          </div>
        </div>

        <WMenu :items="sortItems" :model-value="marketSort" align="right" @select="onSortChange">
          <template #trigger>
            <WButton size="inline" icon="sort">{{ sortLabel }}</WButton>
          </template>
        </WMenu>
      </template>

      <div class="w-toolbar-row__spacer"></div>

      <WButton
        size="inline"
        :icon="viewMode === 'grid' ? 'grid' : 'list'"
        :title="viewMode === 'grid' ? t('table.list') : t('table.grid')"
        @click="toggleView"
      />
    </div>

    <!-- ============ 插件市场 ============ -->
    <section ref="marketPanel" v-show="activeTab === 'market'" class="w-panel">
      <!-- 分类筛选 -->
      <div v-if="marketCategories.length" class="w-chips">
        <button
          class="w-chip"
          :class="{ active: marketCatFilter === null }"
          @click="marketCatFilter = null"
        >
          {{ t('table.allCategories') }}
        </button>
        <button
          v-for="[cat, count] in marketCategories"
          :key="cat"
          class="w-chip"
          :class="{ active: marketCatFilter === cat }"
          :style="{ '--chip-color': categoryColor(cat) }"
          @click="marketCatFilter = marketCatFilter === cat ? null : cat"
        >
          {{ categoryName(cat) }} {{ count }}
        </button>
      </div>

      <p v-if="marketSearch" class="w-count">
        {{ t('market.searchCount', { n: marketTotal }) }}
      </p>

      <!-- 加载中 -->
      <WLoading v-if="marketLoading" block :text="t('market.loading')" />

      <!-- 空：市场整体无数据 -->
      <WEmpty
        v-else-if="!marketPlugins.length"
        type="empty"
        icon="wifiOff"
        :title="t('market.emptyTitle')"
        :desc="t('market.emptyDesc')"
      >
        <template #action>
          <WButton icon="refresh" @click="emit('refresh-market')">
            {{ t('common.retry') }}
          </WButton>
        </template>
      </WEmpty>

      <!-- 空：筛选无结果 -->
      <WEmpty
        v-else-if="!pagedMarket.length"
        type="empty"
        icon="searchOff"
        :title="t('market.noResult')"
        :desc="t('market.noResultDesc')"
      >
        <template #action>
          <WButton @click="resetMarketFilter">{{ t('market.resetFilter') }}</WButton>
        </template>
      </WEmpty>

      <!-- 市场卡片（网格） -->
      <div v-else-if="viewMode === 'grid'" class="w-grid w-grid-market">
        <article
          v-for="mp in pagedMarket"
          :key="mp.name"
          class="w-card w-plugin-card"
          :class="{ 'is-installed': isInstalled(mp) }"
        >
          <div class="weui-media-box weui-media-box_appmsg">
            <div class="weui-media-box__hd">
              <span class="w-plugin-icon"><WIcon name="plugin" :size="20" /></span>
            </div>
            <div class="weui-media-box__bd">
              <h4 class="weui-media-box__title">
                <span class="w-truncate">{{ marketTitle(mp) }}</span>
                <span v-if="isInstalled(mp)" class="w-tag w-tag_success">
                  {{ t('tab.installedTag') }}
                </span>
              </h4>
              <p class="weui-media-box__desc w-clamp-2" :title="marketDesc(mp)">
                <span
                  v-if="mp.category"
                  class="w-badge"
                  :style="badgeStyle(mp.category)"
                >{{ categoryName(mp.category) }}</span>
                {{ marketDesc(mp) }}
              </p>
            </div>
          </div>

          <div class="w-card-metrics">
            <span><WIcon name="star" :size="12" /> {{ formatCount(mp.stars) }}</span>
            <span><WIcon name="download" :size="12" /> {{ formatCount(mp.downloads) }}</span>
          </div>

          <div class="weui-panel__ft w-card-ft">
            <WButton
              v-if="!isInstalled(mp) && mp.npm"
              type="primary"
              size="mini"
              icon="download"
              :loading="pluginStore.installingNpm === mp.npm"
              @click="openInstall(mp)"
            >
              {{ t('market.install') }}
            </WButton>
            <WMenu
              :items="marketLinkItemsFor(mp)"
              align="right"
              @select="(cmd: string) => onMarketLinkSelect(cmd, mp)"
            >
              <template #trigger>
                <WButton size="mini" icon="link" :title="t('market.links')" />
              </template>
            </WMenu>
          </div>
        </article>
      </div>

      <!-- 市场列表（紧凑行） -->
      <div v-else class="weui-cells w-cells w-market-cells">
        <div v-for="mp in pagedMarket" :key="mp.name" class="weui-cell w-cell">
          <div class="weui-cell__hd">
            <span class="w-plugin-icon"><WIcon name="plugin" :size="18" /></span>
          </div>
          <div class="weui-cell__bd">
            <div class="w-cell__title-row">
              <span class="w-cell__name">{{ marketTitle(mp) }}</span>
              <span
                v-if="mp.category"
                class="w-badge"
                :style="badgeStyle(mp.category)"
              >{{ categoryName(mp.category) }}</span>
              <span v-if="isInstalled(mp)" class="w-tag w-tag_success">
                {{ t('tab.installedTag') }}
              </span>
            </div>
            <p class="w-cell__desc w-truncate" :title="marketDesc(mp)">{{ marketDesc(mp) }}</p>
          </div>
          <div class="weui-cell__ft w-cell__ft-actions">
            <span class="w-metric"><WIcon name="star" :size="12" />{{ formatCount(mp.stars) }}</span>
            <span class="w-metric"><WIcon name="download" :size="12" />{{ formatCount(mp.downloads) }}</span>
            <WButton
              v-if="!isInstalled(mp) && mp.npm"
              type="primary"
              size="mini"
              icon="download"
              :loading="pluginStore.installingNpm === mp.npm"
              @click="openInstall(mp)"
            >
              {{ t('market.install') }}
            </WButton>
            <WMenu
              :items="marketLinkItemsFor(mp)"
              align="right"
              @select="(cmd: string) => onMarketLinkSelect(cmd, mp)"
            >
              <template #trigger>
                <WButton size="mini" icon="link" :title="t('market.links')" />
              </template>
            </WMenu>
          </div>
        </div>
      </div>

      <!-- 加载更多 -->
      <div v-if="marketTotal > pagedMarket.length" class="w-more">
        <WButton size="inline" icon="chevronDown" @click="marketPage++">
          {{ t('common.loadMore') }}
        </WButton>
      </div>
    </section>

    <!-- ============ 已安装 ============ -->
    <section ref="installedPanel" v-show="activeTab === 'installed'" class="w-panel">
      <div class="w-toolbar">
        <div class="w-flex w-items-center w-gap-2 w-flex-1">
          <span class="w-text-2">{{ t('table.total', { n: plugins.length }) }}</span>
          <span v-if="updatableCount > 0" class="w-tag w-tag_warn">
            {{ t('table.updatable', { n: updatableCount }) }}
          </span>
        </div>

        <div class="w-flex w-items-center w-gap-2">
          <WButton
            size="inline"
            :icon="viewMode === 'grid' ? 'grid' : 'list'"
            :title="viewMode === 'grid' ? t('table.list') : t('table.grid')"
            @click="toggleView"
          />
        </div>
      </div>

      <!-- 分类筛选 -->
      <div v-if="categories.length" class="w-chips">
        <button
          class="w-chip"
          :class="{ active: categoryFilter === null }"
          @click="categoryFilter = null"
        >
          {{ t('table.allCategories') }}
        </button>
        <button
          v-for="[cat, count] in categories"
          :key="cat"
          class="w-chip"
          :class="{ active: categoryFilter === cat }"
          :style="{ '--chip-color': categoryColor(cat) }"
          @click="categoryFilter = categoryFilter === cat ? null : cat"
        >
          {{ categoryName(cat) }} {{ count }}
        </button>
      </div>

      <!-- 空：未安装任何插件 -->
      <WEmpty
        v-if="!plugins.length"
        type="empty"
        icon="inbox"
        :title="t('installed.emptyTitle')"
        :desc="t('installed.emptyDesc')"
      />

      <!-- 空：筛选无结果 -->
      <WEmpty
        v-else-if="!filteredPlugins.length"
        type="empty"
        icon="searchOff"
        :title="t('market.noResult')"
        :desc="t('market.noResultDesc')"
      >
        <template #action>
          <WButton @click="categoryFilter = null">{{ t('market.resetFilter') }}</WButton>
        </template>
      </WEmpty>

      <!-- 网格视图 -->
      <div v-else-if="viewMode === 'grid'" class="w-grid w-grid-plugin">
        <article
          v-for="row in filteredPlugins"
          :key="row.manifest.id"
          class="w-card w-plugin-card"
          :class="{
            'is-update': row.update_available,
            'is-disabled': !row.manifest.enabled,
            'is-core': row.manifest.type === 'agent-core',
          }"
        >
          <div class="weui-media-box weui-media-box_appmsg">
            <div class="weui-media-box__hd">
              <span
                class="w-plugin-icon"
                :class="{ 'is-core': row.manifest.type === 'agent-core', 'is-update': row.update_available }"
              >
                <WIcon :name="row.manifest.type === 'agent-core' ? 'core' : 'plugin'" :size="20" />
              </span>
            </div>
            <div class="weui-media-box__bd">
              <h4 class="weui-media-box__title">
                <span class="w-truncate">{{ row.manifest.name }}</span>
                <span v-if="row.manifest.type === 'agent-core'" class="w-tag w-tag_brand">
                  {{ t('app.coreTag') }}
                </span>
              </h4>
              <p class="weui-media-box__desc w-clamp-2" :title="row.manifest.description">
                {{ localeDescription(row) }}
              </p>
            </div>
            <div class="w-card-status">
              <StatusTag :row="row" />
            </div>
          </div>

          <!-- 版本 -->
          <div class="w-version" :class="{ 'is-update': row.update_available }">
            <span class="w-version__label">{{ t('table.current') }}</span>
            <span class="w-version__num mono">v{{ row.manifest.current_version }}</span>
            <template v-if="row.update_available && row.latest_version">
              <WIcon name="arrowRight" :size="13" class="w-text-warn" />
              <span class="w-version__label">{{ t('table.latest') }}</span>
              <span class="w-version__num mono is-latest">v{{ row.latest_version }}</span>
            </template>
            <template v-else-if="row.latest_version">
              <WIcon name="check" :size="13" class="w-text-success" />
              <span class="w-version__label">{{ t('table.latest') }}</span>
              <span class="w-version__num mono">v{{ row.latest_version }}</span>
            </template>
            <span v-else-if="row.check_error" class="w-version__err" :title="row.check_error">
              <WIcon name="alert" :size="13" />{{ t('table.checkFailed') }}
            </span>
          </div>

          <!-- 操作 -->
          <div class="weui-panel__ft w-card-ft">
            <template v-if="isUpdating(row.manifest.id)">
              <UpdateProgress
                :percent="getProgressPercent(row.manifest.id)"
                :message="getProgressMessage(row.manifest.id)"
              />
            </template>
            <template v-else>
              <WButton
                v-if="row.update_available"
                type="primary"
                size="mini"
                icon="upload"
                @click="emit('update', row)"
              >
                {{ t('table.update') }}
              </WButton>
              <WButton
                size="mini"
                :icon="row.manifest.enabled ? 'power' : 'check'"
                :title="row.manifest.enabled ? t('table.disable') : t('table.enable')"
                @click="emit('toggle-enabled', row)"
              >
                {{ row.manifest.enabled ? t('table.disable') : t('table.enable') }}
              </WButton>
              <WButton
                size="mini"
                icon="folder"
                :title="t('table.folder')"
                @click="emit('open-folder', row)"
              />
              <WMenu
                :items="moreItemsFor(row)"
                align="right"
                @select="(cmd: string) => onMoreSelect(cmd, row)"
              >
                <template #trigger>
                  <WButton size="mini" icon="more" :title="t('common.more')" />
                </template>
              </WMenu>
            </template>
          </div>
        </article>
      </div>

      <!-- 列表视图（WeUI Cells） -->
      <div v-else class="weui-cells w-cells">
        <div
          v-for="row in filteredPlugins"
          :key="row.manifest.id"
          class="weui-cell w-cell"
          :class="{ 'is-disabled': !row.manifest.enabled }"
        >
          <div class="weui-cell__hd">
            <span
              class="w-plugin-icon sm"
              :class="{ 'is-core': row.manifest.type === 'agent-core', 'is-update': row.update_available }"
            >
              <WIcon :name="row.manifest.type === 'agent-core' ? 'core' : 'plugin'" :size="16" />
            </span>
          </div>

          <div class="weui-cell__bd">
            <p class="w-cell-title">
              {{ row.manifest.name }}
              <span v-if="row.manifest.type === 'agent-core'" class="w-tag w-tag_brand">
                {{ t('app.coreTag') }}
              </span>
              <StatusTag :row="row" />
            </p>
            <p class="w-cell-desc w-clamp-2">{{ localeDescription(row) }}</p>
            <p class="w-cell-meta">
              <span class="mono">v{{ row.manifest.current_version }}</span>
              <template v-if="row.update_available && row.latest_version">
                <WIcon name="arrowRight" :size="11" />
                <span class="mono w-text-warn">v{{ row.latest_version }}</span>
              </template>
              <span v-if="row.stars != null" class="w-cell-metric">
                <WIcon name="star" :size="11" />{{ formatCount(row.stars) }}
              </span>
            </p>
          </div>

          <div class="weui-cell__ft w-cell-ft">
            <template v-if="isUpdating(row.manifest.id)">
              <UpdateProgress
                :percent="getProgressPercent(row.manifest.id)"
                :message="getProgressMessage(row.manifest.id)"
              />
            </template>
            <template v-else>
              <WButton
                v-if="row.update_available"
                type="primary"
                size="mini"
                icon="upload"
                @click="emit('update', row)"
              >
                {{ t('table.update') }}
              </WButton>
              <WButton
                size="mini"
                :icon="row.manifest.enabled ? 'power' : 'check'"
                @click="emit('toggle-enabled', row)"
              />
              <WButton
                size="mini"
                icon="folder"
                :title="t('table.folder')"
                @click="emit('open-folder', row)"
              />
              <WMenu
                :items="moreItemsFor(row)"
                align="right"
                @select="(cmd: string) => onMoreSelect(cmd, row)"
              >
                <template #trigger>
                  <WButton size="mini" icon="more" :title="t('common.more')" />
                </template>
              </WMenu>
            </template>
          </div>
        </div>
      </div>
    </section>

    <!-- ============ 可更新 ============ -->
    <section ref="updatesPanel" v-show="activeTab === 'updates'" class="w-panel">
      <!-- 统计行（与已安装面板同款样式） -->
      <div v-if="updatableList.length" class="w-toolbar">
        <div class="w-flex w-items-center w-gap-2 w-flex-1">
          <span class="w-text-2">{{ t('updates.total', { n: updatableList.length }) }}</span>
          <span class="w-tag w-tag_warn">{{ t('updates.autoTip') }}</span>
        </div>
      </div>

      <!-- 检查中 -->
      <WLoading v-if="isCheckingUpdates" block :text="t('check.inProgress')" />

      <!-- 全部最新 -->
      <WEmpty
        v-else-if="!updatableList.length"
        type="success"
        :title="t('check.allLatest')"
        :desc="t('updates.allLatestDesc')"
      />

      <!-- 可更新列表（网格） -->
      <div v-else-if="viewMode === 'grid'" class="w-grid w-grid-plugin">
        <article v-for="row in updatableList" :key="row.manifest.id" class="w-card w-plugin-card is-update">
          <div class="weui-media-box weui-media-box_appmsg">
            <div class="weui-media-box__hd">
              <span class="w-plugin-icon is-update">
                <WIcon :name="row.manifest.type === 'agent-core' ? 'core' : 'plugin'" :size="20" />
              </span>
            </div>
            <div class="weui-media-box__bd">
              <h4 class="weui-media-box__title">
                <span class="w-truncate">{{ row.manifest.name }}</span>
                <span v-if="row.manifest.type === 'agent-core'" class="w-tag w-tag_brand">
                  {{ t('app.coreTag') }}
                </span>
              </h4>
              <p class="weui-media-box__desc w-clamp-2" :title="row.manifest.description">
                {{ localeDescription(row) }}
              </p>
            </div>
            <div class="w-card-status"><StatusTag :row="row" /></div>
          </div>

          <div class="w-version is-update">
            <span class="w-version__label">{{ t('table.current') }}</span>
            <span class="w-version__num mono">v{{ row.manifest.current_version }}</span>
            <WIcon name="arrowRight" :size="13" class="w-text-warn" />
            <span class="w-version__label">{{ t('table.latest') }}</span>
            <span class="w-version__num mono is-latest">v{{ row.latest_version }}</span>
          </div>

          <div class="weui-panel__ft w-card-ft">
            <WButton
              type="primary"
              size="mini"
              icon="upload"
              :loading="isUpdating(row.manifest.id)"
              @click="emit('update', row)"
            >
              {{ t('table.update') }}
            </WButton>
            <WButton
              v-if="row.release_notes"
              size="mini"
              icon="fileText"
              @click="emit('view-release-notes', row)"
            >
              {{ t('table.releaseNotes') }}
            </WButton>
          </div>
        </article>
      </div>

      <!-- 可更新列表（紧凑行） -->
      <div v-else class="weui-cells w-cells w-updates-cells">
        <div v-for="row in updatableList" :key="row.manifest.id" class="weui-cell w-cell">
          <div class="weui-cell__hd">
            <span class="w-plugin-icon is-update">
              <WIcon :name="row.manifest.type === 'agent-core' ? 'core' : 'plugin'" :size="18" />
            </span>
          </div>
          <div class="weui-cell__bd">
            <div class="w-cell__title-row">
              <span class="w-cell__name">{{ row.manifest.name }}</span>
              <span v-if="row.manifest.type === 'agent-core'" class="w-tag w-tag_brand">
                {{ t('app.coreTag') }}
              </span>
            </div>
            <p class="w-cell__desc w-truncate" :title="localeDescription(row)">
              {{ localeDescription(row) }}
            </p>
            <div class="w-cell__version mono">
              v{{ row.manifest.current_version }} → v{{ row.latest_version }}
            </div>
          </div>
          <div class="weui-cell__ft w-cell__ft-actions">
            <WButton
              type="primary"
              size="mini"
              icon="upload"
              :loading="isUpdating(row.manifest.id)"
              @click="emit('update', row)"
            >
              {{ t('table.update') }}
            </WButton>
            <WButton
              v-if="row.release_notes"
              size="mini"
              icon="fileText"
              @click="emit('view-release-notes', row)"
            >
              {{ t('table.releaseNotes') }}
            </WButton>
          </div>
        </div>
      </div>
    </section>

    <!-- ============ 底部 Tab（移动） ============ -->
    <div class="weui-tabbar w-tabbar w-hide-desktop">
      <a
        v-for="tab in tabs"
        :key="tab.name"
        href="javascript:"
        class="weui-tabbar__item"
        :class="{ 'weui-bar__item_on': activeTab === tab.name }"
        :aria-current="activeTab === tab.name ? 'page' : undefined"
        @click="switchTab(tab.name)"
      >
        <span class="weui-tabbar__icon"><WIcon :name="tab.icon" :size="22" /></span>
        <p class="weui-tabbar__label">{{ tab.short }}</p>
      </a>
    </div>

    <!-- ============ 安装插件对话框 ============ -->
    <WDialog
      v-model="installDialogVisible"
      :title="t('market.installTitle')"
      :close-on-mask="!installingNpmName"
      :busy="!!installingNpmName"
      :closable="!installingNpmName"
    >
      <!-- 安装中 -->
      <div v-if="installingNpmName" class="w-installing">
        <WLoading block :text="t('market.installing', { name: installingNpmName })" />
      </div>

      <!-- 选择目标 -->
      <template v-else>
        <p class="w-label">{{ t('market.chooseTarget') }}</p>
        <div class="weui-cells w-cells">
          <label
            v-for="dir in installTargets"
            :key="dir"
            class="weui-cell weui-cell_access w-cell-tappable"
          >
            <div class="weui-cell__bd">
              <p class="mono w-cell-dir">{{ dir }}</p>
            </div>
            <div class="weui-cell__ft">
              <input v-model="installTarget" type="radio" class="weui-check" :value="dir" />
              <span class="weui-icon-checked" aria-hidden="true" />
            </div>
          </label>
        </div>
      </template>

      <template #footer>
        <template v-if="!installingNpmName">
          <WButton @click="installDialogVisible = false">{{ t('common.cancel') }}</WButton>
          <WButton
            type="primary"
            icon="download"
            :disabled="!installTarget"
            @click="confirmInstall"
          >
            {{ t('market.install') }}
          </WButton>
        </template>
      </template>
    </WDialog>

  </div>
</template>

<script setup lang="ts">
import { computed, nextTick, onMounted, ref, watch, h, defineComponent, type PropType } from 'vue'
import WButton from './WButton.vue'
import WIcon from './WIcon.vue'
import WMenu from './WMenu.vue'
import WDialog from './WDialog.vue'
import WEmpty from './WEmpty.vue'
import WLoading from './WLoading.vue'
import UpdateProgress from './UpdateProgress.vue'
import { usePluginStore } from '../stores/pluginStore'
import { pluginApi } from '../api'
import { t, locale, categoryName, categoryColor, formatCount } from '../i18n'
import { useToast } from '../composables/useToast'
import { staggerIn, panelIn } from '../composables/useMotion'
import type { PluginInfo, MarketPlugin } from '../types'

type TabName = 'market' | 'installed' | 'updates'

const props = defineProps<{
  plugins: PluginInfo[]
  marketPlugins: MarketPlugin[]
  isCheckingUpdates: boolean
}>()

const emit = defineEmits<{
  update: [plugin: PluginInfo]
  uninstall: [plugin: PluginInfo]
  'toggle-enabled': [plugin: PluginInfo]
  'open-folder': [plugin: PluginInfo]
  'check-single': [plugin: PluginInfo]
  'view-release-notes': [plugin: PluginInfo]
  'refresh-market': []
}>()

const pluginStore = usePluginStore()
const toast = useToast()
const rootEl = ref<HTMLElement | null>(null)

/* ---------------- Tab ---------------- */
const activeTab = ref<TabName>('installed')

const tabs = computed(() => [
  {
    name: 'market' as const,
    label: `${t('tab.market')} (${props.marketPlugins.length})`,
    short: t('tab.marketShort'),
    icon: 'grid',
  },
  {
    name: 'installed' as const,
    label: `${t('tab.installed')} (${props.plugins.length})`,
    short: t('tab.installedShort'),
    icon: 'package',
  },
  {
    name: 'updates' as const,
    label: `${t('tab.updates')} (${updatableCount.value})`,
    short: t('tab.updatesShort'),
    icon: 'upload',
  },
])

/** 三个面板的元素引用：切换时用于 GSAP 进场 */
const marketPanel = ref<HTMLElement | null>(null)
const installedPanel = ref<HTMLElement | null>(null)
const updatesPanel = ref<HTMLElement | null>(null)

function activePanel(): HTMLElement | null {
  switch (activeTab.value) {
    case 'market':
      return marketPanel.value
    case 'installed':
      return installedPanel.value
    case 'updates':
      return updatesPanel.value
    default:
      return null
  }
}

function switchTab(name: TabName) {
  if (activeTab.value === name) return
  activeTab.value = name
  // 面板切换动画（横向淡入）
  nextTick(() => panelIn(activePanel()))
}

/* ---------------- 市场 ---------------- */
const marketCatFilter = ref<string | null>(null)
const marketSearch = ref('')
const marketPage = ref(1)
const marketPageSize = 48
const marketSort = ref<'default' | 'stars' | 'downloads' | 'latest'>('default')
const searchFocused = ref(false)

/** 市场数据是否仍在加载（父组件传入空数组且未标结束） */
const marketLoading = computed(() => props.marketPlugins.length === 0)

const sortLabel = computed(() => {
  const map = {
    default: t('market.sortDefault'),
    stars: t('market.sortStars'),
    downloads: t('market.sortDownloads'),
    latest: t('market.sortLatest'),
  }
  return map[marketSort.value] || t('market.sortDefault')
})

const sortItems = computed(() => [
  { label: t('market.sortDefault'), value: 'default' },
  { label: t('market.sortStars'), value: 'stars' },
  { label: t('market.sortDownloads'), value: 'downloads' },
  { label: t('market.sortLatest'), value: 'latest' },
])

function onSortChange(cmd: string) {
  marketSort.value = cmd as typeof marketSort.value
}

const marketFiltered = computed(() => {
  let list = props.marketPlugins
  if (marketCatFilter.value) {
    list = list.filter((mp) => mp.category === marketCatFilter.value)
  }
  const q = marketSearch.value.trim().toLowerCase()
  if (q) {
    list = list.filter((mp) => {
      const cat = categoryName(mp.category).toLowerCase()
      return (
        mp.name.toLowerCase().includes(q) ||
        (mp.desc_zh || '').toLowerCase().includes(q) ||
        (mp.desc_en || '').toLowerCase().includes(q) ||
        cat.includes(q)
      )
    })
  }
  if (marketSort.value === 'stars') {
    list = [...list].sort((a, b) => (b.stars ?? 0) - (a.stars ?? 0))
  } else if (marketSort.value === 'downloads') {
    list = [...list].sort((a, b) => (b.downloads ?? 0) - (a.downloads ?? 0))
  } else if (marketSort.value === 'latest') {
    list = [...list].sort((a, b) => a.name.localeCompare(b.name))
  }
  // 有可用更新的插件置顶（npm/id 命中已安装的可更新列表；稳定排序）
  if (updatableNpmKeys.value.size) {
    list = [...list].sort(
      (a, b) =>
        Number(matchesUpdatable(b)) - Number(matchesUpdatable(a))
    )
  }
  return list
})

const marketTotal = computed(() => marketFiltered.value.length)
const pagedMarket = computed(() =>
  marketFiltered.value.slice(0, marketPage.value * marketPageSize)
)

const marketCategories = computed(() => {
  const map = new Map<string, number>()
  for (const mp of props.marketPlugins) {
    if (mp.category) map.set(mp.category, (map.get(mp.category) || 0) + 1)
  }
  return [...map.entries()].sort((a, b) => b[1] - a[1])
})

function resetMarketFilter() {
  marketSearch.value = ''
  marketCatFilter.value = null
  marketSort.value = 'default'
  marketPage.value = 1
}

watch([marketSearch, marketCatFilter, marketSort], () => {
  marketPage.value = 1
})

/* ---------------- 已安装 ---------------- */
const VIEW_KEY = 'dsh-updater-view-mode'
const viewMode = ref<'grid' | 'list'>(
  (localStorage.getItem(VIEW_KEY) as 'grid' | 'list') || 'grid'
)

function toggleView() {
  viewMode.value = viewMode.value === 'grid' ? 'list' : 'grid'
  animateCards()
}

watch(viewMode, (v) => {
  localStorage.setItem(VIEW_KEY, v)
})

const categoryFilter = ref<string | null>(null)

const categories = computed(() => {
  const map = new Map<string, number>()
  for (const p of props.plugins) {
    if (p.category) map.set(p.category, (map.get(p.category) || 0) + 1)
  }
  return [...map.entries()].sort((a, b) => b[1] - a[1])
})

/** 按分类筛选后的插件（网格与列表视图共用——修复原列表视图未筛选的问题） */
const filteredPlugins = computed(() => {
  let list = props.plugins
  if (categoryFilter.value) {
    list = list.filter((p) => p.category === categoryFilter.value)
  }
  // 可更新插件置顶（稳定排序：组内保持原有顺序）
  return [...list].sort(
    (a, b) => Number(b.update_available) - Number(a.update_available)
  )
})

const updatableCount = computed(
  () => props.plugins.filter((p) => p.update_available).length
)
const updatableList = computed(() => props.plugins.filter((p) => p.update_available))

/** 可更新插件的匹配 key（manifest.id 及其短名），用于市场列表置顶 */
const updatableNpmKeys = computed(() => {
  const s = new Set<string>()
  for (const p of updatableList.value) {
    s.add(p.manifest.id.toLowerCase())
    const short = p.manifest.id.split('/').pop() || ''
    if (short) s.add(short.toLowerCase())
  }
  return s
})

function matchesUpdatable(mp: MarketPlugin): boolean {
  if (mp.name && updatableNpmKeys.value.has(mp.name.toLowerCase())) return true
  if (mp.npm && updatableNpmKeys.value.has(mp.npm.toLowerCase())) return true
  return false
}

/* ---------------- 安装 ---------------- */
const installDialogVisible = ref(false)
const installTargets = ref<string[]>([])
const installTarget = ref('')
const pendingInstallNpm = ref('')
const installingNpmName = ref('')

async function openInstall(mp: MarketPlugin) {
  if (!mp.npm) {
    toast.warn(t('market.noNpm'))
    return
  }
  pendingInstallNpm.value = mp.npm
  try {
    installTargets.value = await pluginApi.listInstallTargets()
  } catch {
    installTargets.value = []
  }
  if (!installTargets.value.length) {
    toast.warn(t('market.noTargets'))
    return
  }
  if (!installTarget.value || !installTargets.value.includes(installTarget.value)) {
    installTarget.value = installTargets.value[0]
  }
  installDialogVisible.value = true
}

async function confirmInstall() {
  const npmName = pendingInstallNpm.value
  const dir = installTarget.value
  if (!npmName || !dir) return
  installingNpmName.value = npmName
  try {
    await pluginStore.installPlugin(npmName, dir)
    toast.success(t('market.installSuccess', { name: npmName, dir }))
    installDialogVisible.value = false
  } catch (e: unknown) {
    toast.error(String(e) || t('market.installFailed'))
  } finally {
    installingNpmName.value = ''
  }
}

/* ---------------- 更多操作（下拉菜单） ---------------- */
function moreItemsFor(row: PluginInfo): { label: string; value: string; type?: 'warn'; desc?: string }[] {
  const items: { label: string; value: string; type?: 'warn'; desc?: string }[] = [
    { label: t('table.recheck'), value: 'check', desc: '' },
  ]
  if (row.release_notes) {
    items.push({ label: t('table.releaseNotes'), value: 'notes', desc: '' })
  }
  items.push({ label: t('table.folder'), value: 'folder', desc: '' })
  if (row.manifest.type !== 'agent-core') {
    items.push({ label: t('table.uninstall'), value: 'uninstall', type: 'warn', desc: '' })
  }
  return items
}

function onMoreSelect(cmd: string, row: PluginInfo) {
  switch (cmd) {
    case 'check':
      emit('check-single', row)
      break
    case 'notes':
      emit('view-release-notes', row)
      break
    case 'folder':
      emit('open-folder', row)
      break
    case 'uninstall':
      emit('uninstall', row)
      break
  }
}

/* ---------------- 市场链接（下拉菜单） ---------------- */
function marketLinkItemsFor(mp: MarketPlugin): { label: string; value: string }[] {
  const items: { label: string; value: string }[] = []
  if (mp.url) items.push({ label: t('market.linkGithub'), value: 'github' })
  if (mp.npm) items.push({ label: t('market.linkNpm'), value: 'npm' })
  if (mp.npm) items.push({ label: t('market.linkMirror'), value: 'mirror' })
  return items
}

function onMarketLinkSelect(cmd: string, mp: MarketPlugin) {
  const npm = (mp.npm || '').trim()
  let url = ''
  if (cmd === 'github') url = mp.url || ''
  else if (cmd === 'npm') url = npm ? `https://www.npmjs.com/package/${npm}` : ''
  else if (cmd === 'mirror') url = npm ? `https://npmmirror.com/package/${npm}` : ''
  if (url) pluginApi.openExternal(url).catch(() => {})
}

/* ---------------- 更新进度 ---------------- */
function isUpdating(id: string) {
  return pluginStore.isUpdating(id)
}

function getProgressPercent(id: string) {
  return pluginStore.getUpdateProgress(id)?.percent || 0
}

function getProgressMessage(id: string) {
  return pluginStore.getUpdateProgress(id)?.message || ''
}

/* 进度条与百分比的补间由 <UpdateProgress> 组件内部驱动（见该文件） */

/* ---------------- 展示辅助 ---------------- */
const installedKeys = computed(() => {
  const s = new Set<string>()
  for (const p of pluginStore.plugins) {
    s.add(p.manifest.id.toLowerCase())
    const short = p.manifest.id.split('/').pop() || ''
    if (short) s.add(short.toLowerCase())
  }
  return s
})

function isInstalled(mp: MarketPlugin): boolean {
  if (installedKeys.value.has(mp.name.toLowerCase())) return true
  if (mp.npm && installedKeys.value.has(mp.npm.toLowerCase())) return true
  return false
}

function marketTitle(mp: MarketPlugin) {
  return mp.name.split('/').pop() || mp.name
}

function marketDesc(mp: MarketPlugin) {
  if (locale.value === 'zh') return mp.desc_zh || mp.desc_en || t('table.noDesc')
  return mp.desc_en || mp.desc_zh || t('table.noDesc')
}

function localeDescription(row: PluginInfo): string {
  const d = row.manifest.description || ''
  if (locale.value === 'zh') return row.description_zh || d || t('table.noDesc')
  return row.description_en || d || t('table.noDesc')
}

function badgeStyle(cat: string | null) {
  const c = categoryColor(cat)
  return {
    background: `${c}1f`,
    color: c,
    borderColor: `${c}55`,
  }
}

/* ---------------- 状态标签 ---------------- */
const StatusTag = defineComponent({
  props: { row: { type: Object as PropType<PluginInfo>, required: true } },
  setup(p) {
    return () => {
      const row = p.row
      const [cls, text] = row.update_available
        ? (['w-tag_warn', t('table.updatableTag')] as const)
        : row.check_error
          ? (['w-tag_danger', t('table.errorTag')] as const)
          : !row.manifest.enabled
            ? (['w-tag_info', t('table.disabledTag')] as const)
            : (['w-tag_success', t('table.latestTag')] as const)
      return h('span', { class: ['w-tag', cls] }, text)
    }
  },
})

/* ---------------- 动画 ---------------- */
function animateCards() {
  nextTick(() => {
    const panels = rootEl.value?.querySelectorAll<HTMLElement>('.w-panel')
    panels?.forEach((panel) => {
      if (panel.style.display === 'none') return
      const cards = panel.querySelectorAll<HTMLElement>(
        '.w-plugin-card, .w-cells .w-cell'
      )
      if (cards.length) staggerIn(Array.from(cards).slice(0, 48))
    })
  })
}

// 列表/筛选变化时做交错进场（限前 48 个，避免长列表卡顿）
watch(
  () => [activeTab.value, marketPage.value, marketCatFilter.value, categoryFilter.value],
  () => animateCards()
)

watch(
  () => [props.plugins.length, props.marketPlugins.length],
  () => animateCards()
)

onMounted(() => animateCards())
</script>

<style scoped>
.plugin-view {
  position: relative;
  padding-bottom: 0;
}

/* ---------- 顶部 Tab ---------- */

/* ---------- 搜索 ---------- */
.w-search {
  flex: 1;
  min-width: 180px;
  max-width: 420px;
  padding: 0;
  background: transparent;
}

.w-search::before,
.w-search::after {
  display: none;
}

.w-search .weui-search-bar__form,
.w-search .weui-search-bar__box {
  border-radius: var(--r-md);
}

.w-count {
  font-size: 12px;
  color: var(--fg-2);
  padding: 0 2px var(--sp-2);
}

/* ---------- 插件卡片 ---------- */
.w-plugin-card {
  display: flex;
  flex-direction: column;
}

/* WeUI media-box 在卡片内去掉上下边线 */
.w-plugin-card .weui-media-box {
  padding: var(--sp-4) var(--sp-4) var(--sp-2);
  flex: 1;
}

.w-plugin-card .weui-media-box::before {
  display: none;
}

.w-plugin-icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 44px;
  height: 44px;
  border-radius: var(--r-md);
  background: var(--brand-soft);
  color: var(--brand-2);
  flex-shrink: 0;
}

.w-plugin-icon.is-core {
  color: var(--brand-2);
  background: var(--brand-soft);
  box-shadow: 0 0 0 1px rgba(99, 102, 241, 0.25);
}

.w-plugin-icon.is-update {
  color: var(--c-warn);
  background: rgba(245, 158, 11, 0.12);
}

.w-plugin-icon.sm {
  width: 34px;
  height: 34px;
  border-radius: var(--r-sm);
  margin-right: var(--sp-2);
}

.weui-media-box__title {
  display: flex;
  align-items: center;
  gap: 6px;
  flex-wrap: wrap;
  font-size: 15px;
  font-weight: 600;
  color: var(--fg);
}

.weui-media-box__desc {
  font-size: 12.5px;
  line-height: 1.6;
  color: var(--fg-2);
  margin-top: 4px;
}

.w-card-status {
  flex-shrink: 0;
}

/* 指标 */
.w-card-metrics {
  display: flex;
  gap: var(--sp-4);
  padding: 0 var(--sp-4) var(--sp-2);
  font-size: 12px;
  color: var(--fg-2);
}

.w-card-metrics span {
  display: inline-flex;
  align-items: center;
  gap: 4px;
}

/* 卡片底部操作区（WeUI panel__ft） */
.w-card-ft {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
  flex-wrap: wrap;
  padding: var(--sp-3) var(--sp-4);
  background: transparent;
  border-top: 1px solid var(--border);
}

.w-card-ft::before {
  display: none;
}

.is-update {
  border-color: rgba(245, 158, 11, 0.45);
}

.is-update .w-card-ft {
  border-color: rgba(245, 158, 11, 0.2);
}

.is-core {
  border-color: rgba(99, 102, 241, 0.45);
}

.is-installed {
  border-color: rgba(34, 197, 94, 0.4);
}

.is-disabled {
  opacity: 0.55;
}

/* ---------- 列表视图（WeUI Cells） ---------- */
.w-cells {
  border-radius: var(--r-lg);
  overflow: hidden;
  font-size: 14px;
  background: var(--bg-card);
  backdrop-filter: var(--glass-blur);
  -webkit-backdrop-filter: var(--glass-blur);
  border: 1px solid var(--border);
  box-shadow: var(--shadow);
}

.w-cells::before,
.w-cells::after {
  display: none;
}

.w-cell {
  align-items: flex-start;
  gap: var(--sp-2);
  flex-wrap: wrap;
  padding: var(--sp-3) var(--sp-4);
  background: transparent;
}

.w-cell::before {
  border-color: var(--border) !important;
}

.w-cell-title {
  display: flex;
  align-items: center;
  gap: 6px;
  flex-wrap: wrap;
  font-size: 14px;
  font-weight: 600;
  color: var(--fg);
}

.w-cell-desc {
  font-size: 12px;
  color: var(--fg-1);
  margin-top: 3px;
  line-height: 1.55;
}

.w-cell-meta {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
  flex-wrap: wrap;
  margin-top: 4px;
  font-size: 11px;
  color: var(--fg-2);
}

.w-cell-metric {
  display: inline-flex;
  align-items: center;
  gap: 3px;
}

.w-cell-ft {
  display: flex;
  align-items: center;
  gap: var(--sp-1);
  flex-wrap: wrap;
  justify-content: flex-end;
}

.w-cell-tappable {
  cursor: pointer;
}

.w-cell-tappable:hover {
  background: var(--bg-hover);
}

.w-cell-dir {
  font-size: 12px;
  word-break: break-all;
}

/* ---------- 安装中 ---------- */
.w-installing {
  padding: var(--sp-4) 0;
}

.w-label {
  font-size: 13px;
  color: var(--fg-1);
  margin-bottom: var(--sp-2);
}

/* ---------- 加载更多 ---------- */
.w-more {
  display: flex;
  justify-content: center;
  padding: var(--sp-4) 0 var(--sp-2);
}

/* ---------- 底部 Tab（移动） ---------- */
.w-tabbar {
  position: fixed;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 800;
}

/* 移动端给底部 tabbar 留出滚动空间 */
@media (max-width: 767px) {
  .plugin-view {
    padding-bottom: 60px;
  }
}

/* 卡片内迷你主按钮：收敛光晕，避免整屏渐变噪音 */
.w-card-ft .weui-btn_primary,
.w-cell-ft .weui-btn_primary {
  box-shadow: none;
}

.w-card-ft .weui-btn_primary:not(:disabled):hover {
  box-shadow: 0 2px 10px rgba(99, 102, 241, 0.4);
}
</style>
