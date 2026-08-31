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
                ref="marketSearchInput"
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
            <label v-if="!searchFocused && !marketSearch" class="weui-search-bar__label" @click.prevent="focusMarketSearch">
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

    <!-- ============ 组合包 ============ -->
    <section ref="bundlesPanel" v-show="activeTab === 'bundles'" class="w-panel">
      <WLoading v-if="bundlesLoading" block :text="t('bundle.loading')" />

      <WEmpty
        v-else-if="!bundles.length"
        type="empty"
        icon="inbox"
        :title="t('bundle.emptyTitle')"
        :desc="t('bundle.emptyDesc')"
      >
        <template #action>
          <WButton icon="refresh" @click="loadBundles(true)">
            {{ t('common.retry') }}
          </WButton>
        </template>
      </WEmpty>

      <div v-else class="w-grid w-grid-market">
        <article
          v-for="b in bundles"
          :key="b.id"
          class="w-card w-plugin-card w-bundle-card"
          @click="openBundleDetail(b)"
        >
          <div class="weui-media-box weui-media-box_appmsg">
            <div class="weui-media-box__hd">
              <span class="w-plugin-icon is-bundle"><WIcon name="package" :size="20" /></span>
            </div>
            <div class="weui-media-box__bd">
              <h4 class="weui-media-box__title">
                <span class="w-truncate">{{ b.name }}</span>
                <span class="w-tag w-tag_brand">{{ t('bundle.presetTag') }}</span>
              </h4>
              <p class="weui-media-box__desc w-clamp-2" :title="b.description">
                {{ b.description || t('table.noDesc') }}
              </p>
              <div v-if="b.tags.length" class="w-bundle-tags">
                <span v-for="tag in b.tags" :key="tag" class="w-badge">{{ tag }}</span>
              </div>
            </div>
          </div>

          <div class="w-card-metrics">
            <span><WIcon name="plugin" :size="12" /> {{ t('bundle.pluginCount', { n: b.plugins.length }) }}</span>
            <span v-if="b.mcpServers.length">{{ t('bundle.mcpCount', { n: b.mcpServers.length }) }}</span>
            <span v-if="b.skills.length">{{ t('bundle.skillCount', { n: b.skills.length }) }}</span>
          </div>
        </article>
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

    <!-- ============ 组合包详情对话框 ============ -->
    <WDialog
      v-model="bundleDialogVisible"
      :title="activeBundle?.name || t('bundle.detailTitle')"
      :close-on-mask="!bundleInstalling"
      :busy="bundleInstalling"
      :closable="!bundleInstalling"
      wide
    >
      <div v-if="activeBundle" class="w-bundle-detail">
        <p class="w-bundle-desc">
          <span v-if="activeBundle.mode === 'preset'" class="w-tag w-tag_info" :title="t('bundle.presetTip')">{{ t('bundle.presetModeTag') }}</span>
          {{ activeBundle.description }}
        </p>
        <div v-if="activeBundle.mode === 'preset'" class="w-callout w-callout_warn">
          {{ t('bundle.presetTip') }}
        </div>

        <!-- 安装中：分阶段进度（预检/备份/下载/安装/校验/提交/回滚） -->
        <div v-if="bundleInstalling" class="w-bundle-progress">
          <UpdateProgress
            :percent="bundleProgress?.percent ?? 0"
            :message="bundleProgressText"
          />
          <WButton size="mini" :disabled="!bundleProgress" @click="cancelBundleInstall">
            {{ t('bundle.cancel') }}
          </WButton>
        </div>

        <!-- 失败：错误 + 已回滚提示 -->
        <div v-else-if="bundleError" class="w-bundle-error">
          <span class="w-tag w-tag_danger">{{ t('bundle.installFailed') }}</span>
          <span v-if="bundleRolledBack" class="w-tag w-tag_warn">{{ t('bundle.rolledBackTag') }}</span>
          <p class="w-bundle-error-text">{{ bundleError }}</p>
        </div>

        <!-- 整包冲突警示 -->
        <div v-if="bundlePreview?.hasBlockingConflict" class="w-callout w-callout_danger">
          {{ t('bundle.blockingConflict') }}
        </div>
        <div v-else-if="bundlePreview && bundlePreview.items.some(i => i.conflicts?.length)" class="w-callout w-callout_warn">
          {{ t('bundle.warnConflict') }}
        </div>

        <!-- 插件清单 -->
        <p class="w-label">{{ t('bundle.pluginList') }}</p>
        <WLoading v-if="bundlePreviewLoading" block :text="t('bundle.previewing')" />
        <div class="weui-cells w-cells w-bundle-list">
          <div
            v-for="item in bundleDisplayItems"
            :key="item.pluginRef"
            class="weui-cell w-cell"
          >
            <div class="weui-cell__hd">
              <span class="w-plugin-icon sm"><WIcon name="plugin" :size="16" /></span>
            </div>
            <div class="weui-cell__bd">
              <p class="w-cell__name mono">{{ item.pluginRef }}</p>
              <p class="w-cell-desc">
                {{ bundleActionLabel(item.action) }}
                <template v-if="item.currentVersion"> · v{{ item.currentVersion }}</template>
              </p>
            </div>
            <div class="weui-cell__ft">
              <span v-if="item.conflicts?.length" class="w-tag w-tag_danger" :title="bundleConflictTitle(item)">{{ t('bundle.conflictTag') }}</span>
              <span v-if="!item.required" class="w-tag">{{ t('bundle.optional') }}</span>
              <span v-if="item.action === 'skip'" class="w-tag w-tag_success">{{ t('bundle.alreadyLatest') }}</span>
              <span v-else-if="item.installed" class="w-tag w-tag_warn">{{ t('bundle.overwriteTag') }}</span>
              <span v-else class="w-tag w-tag_info">{{ t('bundle.installTag') }}</span>
            </div>
          </div>
        </div>

        <!-- MCP 服务模板（env 仅列键名） -->
        <template v-if="activeBundle.mcpServers.length">
          <p class="w-label">{{ t('bundle.mcpList') }}</p>
          <div class="weui-cells w-cells w-bundle-list">
            <div v-for="m in activeBundle.mcpServers" :key="m.serverId" class="weui-cell w-cell">
              <div class="weui-cell__bd">
                <p class="w-cell__name">{{ m.name }} <span class="mono w-text-2">{{ m.serverId }}</span></p>
                <p class="w-cell-desc mono">{{ m.transport }} · {{ m.command }} {{ m.args.join(' ') }}</p>
                <p v-if="m.envKeys.length" class="w-cell-desc">
                  {{ t('bundle.envKeys') }}:
                  <span v-for="k in m.envKeys" :key="k" class="w-badge mono">{{ k }}</span>
                  <span class="w-text-2">{{ t('bundle.envKeysHint') }}</span>
                </p>
              </div>
            </div>
          </div>
        </template>

        <!-- 技能（Skill=插件，P0 载体） -->
        <template v-if="activeBundle.skills.length">
          <p class="w-label">{{ t('bundle.skillList') }}</p>
          <div class="weui-cells w-cells w-bundle-list">
            <div v-for="s in activeBundle.skills" :key="s.skillId" class="weui-cell w-cell">
              <div class="weui-cell__bd">
                <p class="w-cell__name">{{ s.name }} <span class="mono w-text-2">{{ s.skillId }}</span></p>
                <p class="w-cell-desc mono">{{ s.source }} · {{ s.scope }}</p>
              </div>
            </div>
          </div>
        </template>
      </div>

      <template #footer>
        <WButton :disabled="bundleInstalling" @click="bundleDialogVisible = false">
          {{ t('common.close') }}
        </WButton>
        <WButton
          type="primary"
          icon="download"
          :disabled="bundleInstalling || !!bundleError"
          :loading="bundleInstalling"
          @click="confirmBundleInstall"
        >
          {{ t('bundle.install') }}
        </WButton>
      </template>
    </WDialog>

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
import { computed, nextTick, onBeforeUnmount, onMounted, ref, watch, h, defineComponent, type PropType } from 'vue'
import WButton from './WButton.vue'
import WIcon from './WIcon.vue'
import WMenu from './WMenu.vue'
import WDialog from './WDialog.vue'
import WEmpty from './WEmpty.vue'
import WLoading from './WLoading.vue'
import UpdateProgress from './UpdateProgress.vue'
import { usePluginStore } from '../stores/pluginStore'
import { pluginApi } from '../api'
import { bundleApi } from '../api/bundles'
import { t, locale, categoryName, categoryColor, formatCount } from '../i18n'
import { useToast } from '../composables/useToast'
import { staggerIn, panelIn } from '../composables/useMotion'
import type { PluginInfo, MarketPlugin, BundleDef, BundlePreview, BundlePreviewItem, BundleProgress } from '../types'

type TabName = 'market' | 'bundles' | 'installed' | 'updates'

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
    name: 'bundles' as const,
    label: `${t('tab.bundles')} (${bundles.value.length})`,
    short: t('tab.bundlesShort'),
    icon: 'package',
  },
  {
    name: 'installed' as const,
    label: `${t('tab.installed')} (${props.plugins.filter((p) => p.manifest.type !== 'agent-core').length})`,
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
const bundlesPanel = ref<HTMLElement | null>(null)
const installedPanel = ref<HTMLElement | null>(null)
const updatesPanel = ref<HTMLElement | null>(null)

function activePanel(): HTMLElement | null {
  switch (activeTab.value) {
    case 'market':
      return marketPanel.value
    case 'bundles':
      return bundlesPanel.value
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
  // 组合包首次切换时懒加载
  if (name === 'bundles') loadBundles()
  // 面板切换动画（横向淡入）
  nextTick(() => panelIn(activePanel()))
}

/* ---------------- 市场 ---------------- */
const marketCatFilter = ref<string | null>(null)
const marketSearch = ref('')
const marketSearchInput = ref<HTMLInputElement | null>(null)

function focusMarketSearch() {
  marketSearchInput.value?.focus()
}
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
  () => props.plugins.filter((p) => p.update_available && p.manifest.type !== 'agent-core').length
)
const updatableList = computed(() =>
  props.plugins.filter((p) => p.update_available && p.manifest.type !== 'agent-core')
)

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

/* ---------------- 组合包（Bundle，V2 §3 事务安装） ---------------- */
const bundles = ref<BundleDef[]>([])
const bundlesLoading = ref(false)
const bundlesLoaded = ref(false)
const bundleDialogVisible = ref(false)
const activeBundle = ref<BundleDef | null>(null)
const bundlePreview = ref<BundlePreview | null>(null)
const bundlePreviewLoading = ref(false)
const bundleInstalling = ref(false)
const bundleProgress = ref<BundleProgress | null>(null)
const bundleError = ref('')
const bundleRolledBack = ref(false)

async function loadBundles(force = false) {
  if (bundlesLoading.value) return
  if (bundlesLoaded.value && !force) return
  bundlesLoading.value = true
  try {
    bundles.value = await bundleApi.listBundles()
    bundlesLoaded.value = true
  } catch (e) {
    console.error('[bundle] load failed:', e)
  } finally {
    bundlesLoading.value = false
  }
}

/** 详情展示清单：优先预检结果，预检不可用时回退到原始插件清单 */
const bundleDisplayItems = computed(() => {
  const b = activeBundle.value
  if (!b) return []
  if (bundlePreview.value) return bundlePreview.value.items
  return b.plugins.map((p) => ({
    pluginRef: p.pluginRef,
    required: p.required,
    installed: false,
    currentVersion: null as string | null,
    action: 'install' as const,
    conflicts: [],
  }))
})

const bundleProgressText = computed(() => {
  const p = bundleProgress.value
  if (!p) return ''
  return `${stageLabel(p.stage)}：${p.message}`
})

function stageLabel(stage: string): string {
  const key = `bundle.stage.${stage}`
  const label = t(key)
  return label === key ? stage : label
}

function bundleActionLabel(action: string): string {
  if (action === 'install') return t('bundle.actionInstall')
  if (action === 'overwrite') return t('bundle.actionOverwrite')
  return t('bundle.actionSkip')
}

function bundleConflictTitle(item: BundlePreviewItem): string {
  return (item.conflicts || [])
    .map((c) => `${c.conflictWith}${c.reason ? ': ' + c.reason : ''}`)
    .join('\n')
}

async function openBundleDetail(b: BundleDef) {
  activeBundle.value = b
  bundlePreview.value = null
  bundleError.value = ''
  bundleRolledBack.value = false
  bundleProgress.value = null
  bundleDialogVisible.value = true
  bundlePreviewLoading.value = true
  try {
    bundlePreview.value = await bundleApi.previewBundle(b.id)
  } catch (e) {
    // 预检失败不阻塞浏览（安装时后端会再次强校验）
    bundlePreview.value = null
    console.error('[bundle] preview failed:', e)
  } finally {
    bundlePreviewLoading.value = false
  }
}

async function confirmBundleInstall() {
  const b = activeBundle.value
  if (!b || bundleInstalling.value) return
  bundleInstalling.value = true
  bundleError.value = ''
  bundleRolledBack.value = false
  bundleProgress.value = null
  try {
    const result = await bundleApi.installBundle(b.id)
    if (result.status === 'committed') {
      toast.success(t('bundle.installSuccess', { name: b.name }))
      bundleDialogVisible.value = false
      // 安装成功后刷新已安装列表（尽力而为，失败不阻塞结果提示）
      const dir = pluginStore.config?.plugin_directory
      if (dir) pluginStore.scanPlugins(dir).catch(() => {})
    } else if (result.status === 'cancelled') {
      toast.warn(t('bundle.cancelledDone'))
    } else {
      bundleError.value = result.message
      bundleRolledBack.value = result.status === 'rolled_back'
    }
  } catch (e) {
    const msg = String(e)
    bundleError.value = msg
    // 后端失败文案含「已回滚」即代表环境已恢复（V2 §3 规则 1）
    bundleRolledBack.value = msg.includes('已回滚')
    toast.error(t('bundle.installFailed'))
  } finally {
    bundleInstalling.value = false
    bundleProgress.value = null
  }
}

async function cancelBundleInstall() {
  const taskId = bundleProgress.value?.taskId
  if (taskId) {
    try {
      await bundleApi.cancelInstall(taskId)
    } catch {
      // 取消请求失败时事务继续，下次边界仍可取消
    }
  }
}

// 进度事件订阅（bundle_progress，按 taskId 路由；跟随现有 listen 封装）
let unlistenBundleProgress: (() => void) | null = null
onMounted(() => {
  bundleApi
    .onBundleProgress((p) => {
      if (bundleInstalling.value) bundleProgress.value = p
    })
    .then((fn) => {
      unlistenBundleProgress = fn
    })
    .catch(() => {})
})
onBeforeUnmount(() => {
  if (unlistenBundleProgress) unlistenBundleProgress()
})

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

/* ============ 组合包 ============ */
.w-bundle-card {
  cursor: pointer;
  transition: box-shadow 0.2s ease, transform 0.2s ease;
}

.w-bundle-card:hover {
  box-shadow: var(--shadow-lg);
}

.w-plugin-icon.is-bundle {
  color: var(--brand-2);
  background: var(--brand-soft);
}

.w-bundle-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  margin-top: 6px;
}

.w-bundle-tags .w-badge {
  font-size: 11px;
  padding: 1px 8px;
  border-radius: 999px;
  background: var(--brand-soft);
  color: var(--brand-2);
}

.w-card-metrics {
  display: flex;
  gap: 14px;
  align-items: center;
  padding: 8px 16px 12px;
  font-size: 12px;
  color: var(--fg-2);
  border-top: 1px solid var(--line);
}

.w-card-metrics span {
  display: inline-flex;
  align-items: center;
  gap: 4px;
}

.w-bundle-detail .w-bundle-desc {
  font-size: 13px;
  line-height: 1.6;
  color: var(--fg-2);
  margin: 0 0 10px;
}

.w-bundle-progress {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 12px;
}

.w-bundle-error {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px;
  padding: 10px 12px;
  border-radius: var(--r-md);
  background: rgba(239, 68, 68, 0.08);
  margin-bottom: 12px;
}

.w-bundle-error-text {
  width: 100%;
  margin: 0;
  font-size: 12.5px;
  line-height: 1.6;
  color: var(--c-warn);
  word-break: break-all;
}

.w-bundle-list {
  max-height: 260px;
  overflow-y: auto;
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
