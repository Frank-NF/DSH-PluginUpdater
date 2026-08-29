<template>
  <div class="plugin-view-container">
    <el-tabs v-model="activeTab" class="market-tabs">
      <!-- ============ 全部市场 ============ -->
      <el-tab-pane :label="t('tab.market') + ' (' + marketPlugins.length + ')'" name="market">
        <div class="market-search-row">
          <el-input
            v-model="marketSearch"
            :placeholder="t('market.searchPlaceholder')"
            clearable
            size="default"
            class="market-search"
          >
            <template #prefix>
              <el-icon><Search /></el-icon>
            </template>
          </el-input>
          <span v-if="marketSearch" class="search-count">{{ t('market.searchCount', { n: marketTotal }) }}</span>
          <!-- 排序下拉 -->
          <el-dropdown trigger="click" @command="onSortChange">
            <el-button size="default" :class="{ 'sort-active': marketSort !== 'default' }">
              <el-icon><Sort /></el-icon>
              <span class="sort-label">{{ sortLabel }}</span>
              <el-icon class="el-icon--right"><ArrowDown /></el-icon>
            </el-button>
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item :command="'default'" :disabled="marketSort === 'default'">{{ t('market.sortDefault') }}</el-dropdown-item>
                <el-dropdown-item :command="'stars'" :disabled="marketSort === 'stars'">{{ t('market.sortStars') }}</el-dropdown-item>
                <el-dropdown-item :command="'downloads'" :disabled="marketSort === 'downloads'">{{ t('market.sortDownloads') }}</el-dropdown-item>
                <el-dropdown-item :command="'latest'" :disabled="marketSort === 'latest'">{{ t('market.sortLatest') }}</el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
        </div>
        <div class="category-filter">
          <span
            class="cat-chip"
            :class="{ active: marketCatFilter === null }"
            @click="marketCatFilter = null"
          >{{ t('table.allCategories') }}</span>
          <span
            v-for="[cat, count] in marketCategories"
            :key="cat"
            class="cat-chip"
            :class="{ active: marketCatFilter === cat }"
            :style="{ '--cat-color': categoryColor(cat) }"
            @click="marketCatFilter = marketCatFilter === cat ? null : cat"
          >
            {{ categoryName(cat) }} {{ count }}
          </span>
        </div>
        <div v-if="marketPlugins.length === 0" class="updates-empty">
          <el-icon :size="36"><Loading class="is-loading" /></el-icon>
          <p>{{ t('repair.checking') }}</p>
        </div>
        <div v-else class="plugin-grid">
          <div
            v-for="mp in pagedMarket"
            :key="mp.name"
            class="plugin-card market-card"
            :class="{ 'card-installed': isInstalled(mp) }"
          >
            <div class="card-header">
              <div class="card-icon">
                <el-icon :size="22" color="var(--accent)"><Grid /></el-icon>
              </div>
              <div class="card-title">
                <div class="card-name">
                  <span>{{ marketTitle(mp) }}</span>
                </div>
                <div class="card-id">
                  <span
                    v-if="mp.category"
                    class="cat-badge"
                    :style="{ background: categoryColor(mp.category) + '22', color: categoryColor(mp.category), borderColor: categoryColor(mp.category) + '55' }"
                  >{{ categoryName(mp.category) }}</span>
                  {{ mp.name }}
                </div>
              </div>
              <div class="card-status">
                <el-tag v-if="isInstalled(mp)" size="small" type="success" effect="plain">{{ t('tab.installedTag') }}</el-tag>
                <el-button
                  v-else-if="mp.npm"
                  size="small"
                  type="primary"
                  :loading="pluginStore.installingNpm === mp.npm"
                  @click.stop="openInstall(mp)"
                >{{ t('market.install') }}</el-button>
                <!-- 官方/镜像链接下拉（无任何地址的插件自动隐藏） -->
                <el-dropdown v-if="hasMarketLink(mp)" trigger="click" @command="(cmd: string) => openMarketLink(cmd, mp)">
                  <el-button size="small" class="link-btn" :title="t('market.links')">
                    <el-icon><Link /></el-icon>
                    <el-icon class="el-icon--right"><ArrowDown /></el-icon>
                  </el-button>
                  <template #dropdown>
                    <el-dropdown-menu>
                      <el-dropdown-item v-if="mp.url" command="github">{{ t('market.linkGithub') }}</el-dropdown-item>
                      <el-dropdown-item v-if="mp.npm" command="npm">{{ t('market.linkNpm') }}</el-dropdown-item>
                      <el-dropdown-item v-if="mp.npm" command="mirror">{{ t('market.linkMirror') }}</el-dropdown-item>
                    </el-dropdown-menu>
                  </template>
                </el-dropdown>
              </div>
            </div>
            <div class="card-desc" :title="marketDesc(mp)">{{ marketDesc(mp) }}</div>
            <div class="card-metrics">
              <span class="metric">
                <el-icon><Star /></el-icon>
                {{ formatCount(mp.stars) }}
              </span>
              <span class="metric">
                <el-icon><Download /></el-icon>
                {{ formatCount(mp.downloads) }}
              </span>
            </div>
          </div>
        </div>
        <div class="market-pager" v-if="marketTotal > marketPageSize">
          <el-pagination
            layout="prev, pager, next"
            :total="marketTotal"
            :page-size="marketPageSize"
            v-model:current-page="marketPage"
            background
            small
          />
        </div>
      </el-tab-pane>

      <!-- ============ 已安装 ============ -->
      <el-tab-pane :label="t('tab.installed') + ' (' + plugins.length + ')'" name="installed">
    <!-- 工具行 -->
    <div class="grid-toolbar">
      <div class="toolbar-left">
        <span class="plugin-count">{{ t('table.total', { n: plugins.length }) }}</span>
        <span v-if="updatableCount > 0" class="updatable-count">
          {{ t('table.updatable', { n: updatableCount }) }}
        </span>
      </div>
      <div class="category-filter">
        <span
          class="cat-chip"
          :class="{ active: categoryFilter === null }"
          @click="categoryFilter = null"
        >{{ t('table.allCategories') }}</span>
        <span
          v-for="[cat, count] in categories"
          :key="cat"
          class="cat-chip"
          :class="{ active: categoryFilter === cat }"
          :style="{ '--cat-color': categoryColor(cat) }"
          @click="categoryFilter = categoryFilter === cat ? null : cat"
        >
          {{ categoryName(cat) }} {{ count }}
        </span>
      </div>
      <div class="toolbar-right">
        <!-- 布局切换 -->
        <el-radio-group v-model="viewMode" size="small">
          <el-radio-button value="grid">
            <el-icon><Grid /></el-icon>
            <span class="view-label"> {{ t('table.grid') }} </span>
          </el-radio-button>
          <el-radio-button value="table">
            <el-icon><Expand /></el-icon>
            <span class="view-label"> {{ t('table.list') }} </span>
          </el-radio-button>
        </el-radio-group>
        <el-button
          size="small"
          :icon="MagicStick"
            :disabled="plugins.length === 0"
          >
  
        </el-button>
      </div>
    </div>

    <!-- ============ 网格视图 ============ -->
    <div v-if="viewMode === 'grid'" class="plugin-grid">
      <div
        v-for="row in filteredPlugins"
        :key="row.manifest.id"
        class="plugin-card"
        :class="{
          'card-update': row.update_available,
          'card-disabled': !row.manifest.enabled,
          'card-core': row.manifest.type === 'agent-core',
        }"
      >
        <div class="card-header">
          <div class="card-icon">
            <el-icon :size="22" :color="row.manifest.type === 'agent-core' ? 'var(--primary)' : 'var(--accent)'">
              <Cpu v-if="row.manifest.type === 'agent-core'" />
              <Grid v-else />
            </el-icon>
          </div>
          <div class="card-title">
            <div class="card-name">
              <span :class="{ 'core-name': row.manifest.type === 'agent-core' }">
                {{ row.manifest.name }}
              </span>
              <el-tag
                v-if="row.manifest.type === 'agent-core'"
                size="small"
                type="primary"
                effect="plain"
              >{{ t('table.notSet') }}</el-tag>
            </div>
            <div class="card-id">
              <span
                v-if="row.category"
                class="cat-badge"
                :style="{ background: categoryColor(row.category) + '22', color: categoryColor(row.category), borderColor: categoryColor(row.category) + '55' }"
              >{{ categoryName(row.category) }}</span>
              {{ row.manifest.id }}
            </div>
          </div>
          <div class="card-status">
            <StatusTag :row="row" />
          </div>
        </div>

        <div class="card-desc" :title="row.manifest.description">
          {{ localeDescription(row) }}
        </div>

        <div class="card-versions">
          <div class="version-item">
            <span class="version-label"> {{ t('table.current') }} </span>
            <span class="version-num">v{{ row.manifest.current_version }}</span>
          </div>
          <template v-if="row.update_available && row.latest_version">
            <el-icon class="version-arrow" color="var(--warning)"><Right /></el-icon>
            <div class="version-item latest">
              <span class="version-label"> {{ t('table.latest') }} </span>
              <span class="version-num highlight">v{{ row.latest_version }}</span>
            </div>
          </template>
          <template v-else-if="row.latest_version">
            <el-icon class="version-arrow" color="var(--accent)"><Check /></el-icon>
            <div class="version-item latest">
              <span class="version-label">{{ t('table.latest') }}</span>
              <span class="version-num">v{{ row.latest_version }}</span>
            </div>
          </template>
          <div v-else-if="row.check_error" class="version-error" :title="row.check_error">
            <el-icon color="var(--danger)"><Warning /></el-icon>
            {{ t('table.checkFailed') }}
          </div>
        </div>

        <div class="card-metrics" v-if="row.stars != null || row.downloads != null">
          <span class="metric" v-if="row.stars != null">
            <el-icon><Star /></el-icon>
            {{ formatCount(row.stars) }}
          </span>
          <span class="metric" v-if="row.downloads != null">
            <el-icon><Download /></el-icon>
            {{ formatCount(row.downloads) }}
          </span>
        </div>

        <div class="card-links">
          <el-link
            v-if="row.manifest.github_repo"
            type="primary"
            :underline="false"
            @click.prevent="openExternal('https://github.com/' + row.manifest.github_repo)"
          >
            <el-icon><Link /></el-icon>
            <span class="repo-name">{{ row.manifest.github_repo }}</span>
          </el-link>
          <el-link
            v-if="row.release_url"
            type="success"
            :underline="false"
            @click.prevent="openExternal(row.release_url)"
          >
            <el-icon><Document /></el-icon>
            Release
          </el-link>
          <el-link
            type="warning"
            :underline="false"
            @click.prevent="openExternal(npmMirrorUrl(row.manifest.id))"
          >
            <el-icon><Link /></el-icon>
            <span class="repo-name">{{ t('market.mirrorTag') }}</span>
          </el-link>
          <span v-if="!row.manifest.github_repo" class="text-muted">{{ t('table.noRepo') }}</span>
        </div>

        <div class="card-footer">
          <div v-if="isUpdating(row.manifest.id)" class="update-progress">
            <el-progress
              :percentage="getProgressPercent(row.manifest.id)"
              :stroke-width="8"
              :status="getProgressStatus(row.manifest.id)"
            />
            <span class="progress-text">{{ getProgressMessage(row.manifest.id) }}</span>
          </div>
          <div v-else class="card-actions">
            <el-button
              v-if="row.update_available"
              type="primary"
              size="small"
              :icon="Upload"
              @click="$emit('update', row)"
            >
              {{ t('table.update') }}
            </el-button>
            <el-button
              size="small"
              :type="row.manifest.enabled ? 'warning' : 'success'"
              :icon="SwitchButton"
              @click="$emit('toggle-enabled', row)"
            >
              {{ row.manifest.enabled ? t('table.disable') : t('table.enable') }}
            </el-button>
            <el-button size="small" :icon="FolderOpened" @click="$emit('open-folder', row)">
              {{ t('table.folder') }}
            </el-button>
            <el-dropdown trigger="click" @command="(cmd: string) => handleMoreCommand(cmd, row)">
              <el-button size="small">
                <el-icon><MoreFilled /></el-icon>
              </el-button>
              <template #dropdown>
                <el-dropdown-menu>
                  <el-dropdown-item command="check" :icon="Refresh">
                    {{ t('table.recheck') }}
                  </el-dropdown-item>
                  <el-dropdown-item v-if="row.release_notes" command="notes" :icon="Document">
                    {{ t('table.releaseNotes') }}
                  </el-dropdown-item>
                  <el-dropdown-item
                    v-if="row.manifest.type !== 'agent-core'"
                    command="uninstall"
                    :icon="Delete"
                    divided
                  >
                    <span style="color: var(--danger)">卸载</span>
                  </el-dropdown-item>
                </el-dropdown-menu>
              </template>
            </el-dropdown>
          </div>
        </div>
      </div>
    </div>

    <!-- ============ 列表视图 ============ -->
    <div v-else class="plugin-table-wrap">
      <table class="ptable">
        <thead>
          <tr>
            <th class="col-status">{{ t('table.status') }}</th>
            <th class="col-name">{{ t('table.name') }}</th>
            <th class="col-desc">{{ t('table.description') }}</th>
            <th class="col-version">{{ t('table.version') }}</th>
            <th class="col-github">GitHub</th>
            <th class="col-actions">{{ t('table.actions') }}</th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="row in plugins"
            :key="row.manifest.id"
            :class="{
              'tr-update': row.update_available,
              'tr-disabled': !row.manifest.enabled,
              'tr-core': row.manifest.type === 'agent-core',
            }"
          >
            <td class="col-status"><StatusTag :row="row" /></td>
            <td class="col-name">
              <div class="cell-name">
                <el-icon
                  :size="16"
                  :color="row.manifest.type === 'agent-core' ? 'var(--primary)' : 'var(--accent)'"
                >
                  <Cpu v-if="row.manifest.type === 'agent-core'" />
                  <Grid v-else />
                </el-icon>
                <div>
                  <div class="cell-title">
                    {{ row.manifest.name }}
                    <el-tag
                      v-if="row.manifest.type === 'agent-core'"
                      size="small"
                      type="primary"
                      effect="plain"
                    >
                      {{ t('app.coreTag') }}
                    </el-tag>
                  </div>
                  <div class="cell-id">
                    <span
                      v-if="row.category"
                      class="cat-badge"
                      :style="{ background: categoryColor(row.category) + '22', color: categoryColor(row.category), borderColor: categoryColor(row.category) + '55' }"
                    >{{ categoryName(row.category) }}</span>
                    {{ row.manifest.id }}
                  </div>
                </div>
              </div>
            </td>
            <td class="col-desc">
              <span class="cell-desc" :title="row.manifest.description">
                {{ localeDescription(row) }}
              </span>
              <span class="cell-metrics" v-if="row.stars != null || row.downloads != null">
                <span class="metric" v-if="row.stars != null">
                  <el-icon><Star /></el-icon>
                  {{ formatCount(row.stars) }}
                </span>
                <span class="metric" v-if="row.downloads != null">
                  <el-icon><Download /></el-icon>
                  {{ formatCount(row.downloads) }}
                </span>
              </span>
            </td>
            <td class="col-version">
              <div class="cell-version">
                <span class="version-num">v{{ row.manifest.current_version }}</span>
                <template v-if="row.update_available && row.latest_version">
                  <el-icon color="var(--warning)"><Right /></el-icon>
                  <span class="version-num highlight">v{{ row.latest_version }}</span>
                </template>
                <span v-else-if="row.check_error" class="cell-error" :title="row.check_error">
                  {{ t('table.checkFailed') }}
                </span>
              </div>
            </td>
            <td class="col-github">
              <el-link
                v-if="row.manifest.github_repo"
                type="primary"
                :underline="false"
                @click.prevent="openExternal('https://github.com/' + row.manifest.github_repo)"
              >
                <span class="repo-name">{{ row.manifest.github_repo }}</span>
              </el-link>
              <span v-else class="text-muted">{{ t('table.notSet') }}</span>
              <el-link
                type="warning"
                :underline="false"
                class="mirror-link"
                @click.prevent="openExternal(npmMirrorUrl(row.manifest.id))"
              >{{ t('market.mirrorTag') }}</el-link>
            </td>
            <td class="col-actions">
              <div v-if="isUpdating(row.manifest.id)" class="update-progress">
                <el-progress
                  :percentage="getProgressPercent(row.manifest.id)"
                  :stroke-width="8"
                  :status="getProgressStatus(row.manifest.id)"
                />
                <span class="progress-text">{{ getProgressMessage(row.manifest.id) }}</span>
              </div>
              <div v-else class="row-actions">
                <el-button
                  v-if="row.update_available"
                  type="primary"
                  size="small"
                  @click="$emit('update', row)"
                >
                  {{ t('table.update') }}
                </el-button>
                <el-button
                  size="small"
                  :type="row.manifest.enabled ? 'warning' : 'success'"
                  @click="$emit('toggle-enabled', row)"
                >
                  {{ row.manifest.enabled ? t('table.disable') : t('table.enable') }}
                </el-button>
                <el-button size="small" :icon="FolderOpened" @click="$emit('open-folder', row)" />
                <el-dropdown trigger="click" @command="(cmd: string) => handleMoreCommand(cmd, row)">
                  <el-button size="small" :icon="MoreFilled" />
                  <template #dropdown>
                    <el-dropdown-menu>
                      <el-dropdown-item command="check" :icon="Refresh">{{ t('table.recheck') }}</el-dropdown-item>
                      <el-dropdown-item v-if="row.release_notes" command="notes" :icon="Document">{{ t('table.releaseNotes') }}</el-dropdown-item>
                      <el-dropdown-item
                        v-if="row.manifest.type !== 'agent-core'"
                        command="uninstall"
                        :icon="Delete"
                        divided
                      >
                        <span style="color: var(--danger)">卸载</span>
                      </el-dropdown-item>
                    </el-dropdown-menu>
                  </template>
                </el-dropdown>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- 加载遮罩 -->
    <div v-if="isCheckingUpdates" class="loading-overlay">
      <el-icon class="is-loading" :size="32"><Loading /></el-icon>
      <span>{{ t('check.inProgress') }}</span>
    </div>
      </el-tab-pane>

      <!-- ============ 可更新 ============ -->
      <el-tab-pane :label="t('tab.updates') + ' (' + updatableCount + ')'" name="updates">
        <div v-if="pluginStore.updatablePlugins.length === 0" class="updates-empty">
          <el-icon :size="36"><Check /></el-icon>
          <p>{{ t('check.allLatest') }}</p>
        </div>
        <div v-else class="plugin-grid">
          <div
            v-for="row in pluginStore.updatablePlugins"
            :key="row.manifest.id"
            class="plugin-card card-update"
          >
            <div class="card-header">
              <div class="card-icon">
                <el-icon :size="22" color="var(--primary)">
                  <Cpu v-if="row.manifest.type === 'agent-core'" />
                  <Grid v-else />
                </el-icon>
              </div>
              <div class="card-title">
                <div class="card-name">
                  <span>{{ row.manifest.name }}</span>
                </div>
                <div class="card-id">{{ row.manifest.id }}</div>
              </div>
              <div class="card-status"><StatusTag :row="row" /></div>
            </div>
            <!-- 插件介绍 -->
            <div class="card-desc" :title="row.manifest.description || row.description_zh || row.description_en">
              {{ localeDescription(row) }}
            </div>
            <div class="card-versions">
              <div class="version-item">
                <span class="version-label">{{ t('table.current') }}</span>
                <span class="version-num">v{{ row.manifest.current_version }}</span>
              </div>
              <el-icon class="version-arrow" color="var(--warning)"><Right /></el-icon>
              <div class="version-item latest">
                <span class="version-label">{{ t('table.latest') }}</span>
                <span class="version-num highlight">v{{ row.latest_version }}</span>
              </div>
            </div>
            <!-- 链接和操作 -->
            <div class="card-links">
              <el-link
                v-if="row.manifest.github_repo"
                type="primary"
                :underline="false"
                @click.prevent="openExternal('https://github.com/' + row.manifest.github_repo)"
              >
                <el-icon><Link /></el-icon>
                <span class="repo-name">{{ row.manifest.github_repo }}</span>
              </el-link>
              <el-link
                v-if="row.release_url"
                type="success"
                :underline="false"
                @click.prevent="openExternal(row.release_url)"
              >
                <el-icon><Document /></el-icon>
                {{ t('notes.viewOnGithub') }}
              </el-link>
              <el-link
                type="warning"
                :underline="false"
                @click.prevent="openExternal(npmMirrorUrl(row.manifest.id))"
              >
                <el-icon><Link /></el-icon>
                <span class="repo-name">{{ t('market.mirrorTag') }}</span>
              </el-link>
              <div class="card-actions" style="margin-left: auto;">
                <el-button
                  type="primary"
                  size="small"
                  :icon="Upload"
                  :loading="isUpdating(row.manifest.id)"
                  @click="emit('update', row)"
                >{{ t('table.update') }}</el-button>
              </div>
            </div>
          </div>
        </div>
      </el-tab-pane>
    </el-tabs>

    <!-- 安装插件对话框 -->
    <el-dialog
      v-model="installDialogVisible"
      :title="t('market.installTitle')"
      width="520px"
      :close-on-click-modal="!installingNpmName"
    >
      <div v-if="installingNpmName" class="install-progress-row">
        <el-icon class="is-loading"><Loading /></el-icon>
        <span>{{ t('market.installing', { name: installingNpmName }) }}</span>
      </div>
      <template v-else>
        <p class="install-target-label">{{ t('market.chooseTarget') }}</p>
        <el-select v-model="installTarget" style="width: 100%" size="default">
          <el-option v-for="dir in installTargets" :key="dir" :label="dir" :value="dir" />
        </el-select>
      </template>
      <template #footer>
        <template v-if="!installingNpmName">
          <el-button @click="installDialogVisible = false">{{ t('common.cancel') }}</el-button>
          <el-button type="primary" :disabled="!installTarget" @click="confirmInstall">{{ t('market.install') }}</el-button>
        </template>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch, h, defineComponent, type PropType } from 'vue'
import {
  Cpu,
  Grid,
  Link,
  Document,
  Warning,
  Upload,
  SwitchButton,
  FolderOpened,
  Refresh,
  Delete,
  Loading,
  Right,
  Check,
  MoreFilled,
  MagicStick,
  Expand,
  ArrowDown,
  Sort,
} from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'
import { usePluginStore } from '../stores/pluginStore'
import { pluginApi } from '../api'
import { t, locale, categoryName, categoryColor, formatCount } from '../i18n'
import type { PluginInfo, MarketPlugin } from '../types'

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
  }>()

const pluginStore = usePluginStore()

// ============ 插件安装（市场 → DSH profile） ============
const installDialogVisible = ref(false)
const installTargets = ref<string[]>([])
const installTarget = ref('')
const pendingInstallNpm = ref('')
const installingNpmName = ref('')

async function openInstall(mp: MarketPlugin) {
  if (!mp.npm) {
    ElMessage.warning(t('market.noNpm'))
    return
  }
  pendingInstallNpm.value = mp.npm
  try {
    installTargets.value = await pluginApi.listInstallTargets()
  } catch {
    installTargets.value = []
  }
  if (installTargets.value.length === 0) {
    ElMessage.warning(t('market.noTargets'))
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
    ElMessage.success(t('market.installSuccess', { name: npmName, dir }))
    installDialogVisible.value = false
  } catch (e: any) {
    ElMessage.error(e?.toString() || t('market.installFailed'))
  } finally {
    installingNpmName.value = ''
  }
}

// ============ 外部链接（系统浏览器打开，统一入口）============
function openExternal(url: string) {
  if (!url) return
  pluginApi.openExternal(url).catch(() => {})
}

function npmMirrorUrl(npmName: string): string {
  const n = (npmName || '').trim()
  return n ? `https://npmmirror.com/package/${n}` : ''
}

function hasMarketLink(mp: MarketPlugin): boolean {
  return !!(mp.url || mp.npm)
}

function openMarketLink(cmd: string, mp: MarketPlugin) {
  const npm = (mp.npm || '').trim()
  let url = ''
  if (cmd === 'github') url = mp.url || ''
  else if (cmd === 'npm') url = npm ? `https://www.npmjs.com/package/${npm}` : ''
  else if (cmd === 'mirror') url = npm ? `https://npmmirror.com/package/${npm}` : ''
  openExternal(url)
}

// 布局切换（持久化到 localStorage）
const VIEW_KEY = 'dsh-updater-view-mode'
const viewMode = ref<'grid' | 'table'>(
  (localStorage.getItem(VIEW_KEY) as 'grid' | 'table') || 'grid'
)
watch(viewMode, (v) => localStorage.setItem(VIEW_KEY, v))

const updatableCount = computed(() => props.plugins.filter((p) => p.update_available).length)

// 分类筛选：null = 全部；点击分类 chip 切换
const categoryFilter = ref<string | null>(null)

/** 插件里出现的分类（按数量降序） */
const categories = computed(() => {
  const map = new Map<string, number>()
  for (const p of props.plugins) {
    if (p.category) map.set(p.category, (map.get(p.category) || 0) + 1)
  }
  return [...map.entries()].sort((a, b) => b[1] - a[1])
})

/** 按分类筛选后的插件 */
const filteredPlugins = computed(() => {
  if (!categoryFilter.value) return props.plugins
  return props.plugins.filter((p) => p.category === categoryFilter.value)
})

// ===== 插件市场标签页 =====
const activeTab = ref<'market' | 'installed' | 'updates'>('market')
const marketCatFilter = ref<string | null>(null)
const marketSearch = ref('')
const marketPage = ref(1)
const marketPageSize = 48
const marketSort = ref<'default' | 'stars' | 'downloads' | 'latest'>('default')

// 排序标签（根据当前排序模式显示）
const sortLabel = computed(() => {
  const map: Record<string, string> = {
    default: t('market.sortDefault'),
    stars: t('market.sortStars'),
    downloads: t('market.sortDownloads'),
    latest: t('market.sortLatest'),
  }
  return map[marketSort.value] || t('market.sortDefault')
})

function onSortChange(cmd: string) {
  marketSort.value = cmd as typeof marketSort.value
  marketPage.value = 1
}

/** 市场插件过滤 + 排序 */
const marketFiltered = computed(() => {
  let list = props.marketPlugins
  if (marketCatFilter.value) {
    list = list.filter((mp) => mp.category === marketCatFilter.value)
  }
  const q = marketSearch.value.trim().toLowerCase()
  if (q) {
    list = list.filter((mp) => {
      const catName = categoryName(mp.category).toLowerCase()
      return (
        mp.name.toLowerCase().includes(q) ||
        (mp.desc_zh || '').toLowerCase().includes(q) ||
        (mp.desc_en || '').toLowerCase().includes(q) ||
        catName.includes(q)
      )
    })
  }
  // 排序
  if (marketSort.value === 'stars') {
    list = [...list].sort((a, b) => (b.stars ?? 0) - (a.stars ?? 0))
  } else if (marketSort.value === 'downloads') {
    list = [...list].sort((a, b) => (b.downloads ?? 0) - (a.downloads ?? 0))
  } else if (marketSort.value === 'latest') {
    // 按名称字母序（作为最新发布代理）
    list = [...list].sort((a, b) => a.name.localeCompare(b.name))
  }
  return list
})

// 搜索词变化时回到第一页
watch(marketSearch, () => { marketPage.value = 1 })
watch(marketCatFilter, () => { marketPage.value = 1 })
watch(marketSort, () => { marketPage.value = 1 })

/** 市场中出现的分类（按数量降序） */
const marketCategories = computed(() => {
  const map = new Map<string, number>()
  for (const mp of props.marketPlugins) {
    if (mp.category) map.set(mp.category, (map.get(mp.category) || 0) + 1)
  }
  return [...map.entries()].sort((a, b) => b[1] - a[1])
})



const marketTotal = computed(() => marketFiltered.value.length)



/** 已安装插件 key 集合（id 小写 + 去 scope 短名） */
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

/** 市场卡片标题：取可读短名（去 scope 前缀） */
function marketTitle(mp: MarketPlugin): string {
  const short = mp.name.split('/').pop() || mp.name
  return short
}

function marketDesc(mp: MarketPlugin): string {
  if (locale.value === 'zh') return mp.desc_zh || mp.desc_en || t('table.noDesc')
  return mp.desc_en || mp.desc_zh || t('table.noDesc')
}

function isUpdating(pluginId: string): boolean {
  return pluginStore.isUpdating(pluginId)
}

function getProgressPercent(pluginId: string): number {
  const progress = pluginStore.getUpdateProgress(pluginId)
  return progress?.percent || 0
}

function getProgressMessage(pluginId: string): string {
  const progress = pluginStore.getUpdateProgress(pluginId)
  return progress?.message || ''
}

function getProgressStatus(pluginId: string): '' | 'success' | 'exception' | 'warning' {
  const progress = pluginStore.getUpdateProgress(pluginId)
  if (!progress) return ''
  if (progress.phase === 'complete') return 'success'
  if (progress.phase === 'error') return 'exception'
  return ''
}

// 按当前语言选择描述：zh→官方中文，en→官方英文，回退原始描述
const localeDescription = (row: PluginInfo): string => {
  const d = row.manifest.description || ''
  if (locale.value === 'zh') return row.description_zh || d || t('table.noDesc')
  return row.description_en || d || t('table.noDesc')
}

// 状态标签（网格/列表共用，保证两视图一致）
const StatusTag = defineComponent({
  props: { row: { type: Object as PropType<PluginInfo>, required: true } },
  setup(p) {
    return () => {
      const row = p.row
      const [type, text] = row.update_available
        ? (['warning', t('table.updatableTag')] as const)
        : row.check_error
          ? (['danger', t('table.errorTag')] as const)
          : !row.manifest.enabled
            ? (['info', t('table.disabledTag')] as const)
            : (['success', t('table.latestTag')] as const)
      return h('span', { class: ['mini-tag', `mini-tag-${type}`] }, text)
    }
  },
})

function handleMoreCommand(command: string, row: PluginInfo) {
  switch (command) {
    case 'check':
      emit('check-single', row)
      break
    case 'notes':
      emit('view-release-notes', row)
      break
    case 'uninstall':
      emit('uninstall', row)
      break
  }
}
</script>

<style scoped>
.plugin-view-container {
  position: relative;
}

/* ---------- 工具行 ---------- */
.grid-toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  margin-bottom: 16px;
  flex-wrap: wrap;
}

.toolbar-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.toolbar-right {
  display: flex;
  align-items: center;
  gap: 10px;
}

.view-label {
  margin-left: 4px;
}

.plugin-count {
  font-size: 13px;
  color: var(--text-secondary);
}

.updatable-count {
  font-size: 13px;
  color: var(--warning);
  font-weight: 600;
  padding: 3px 10px;
  border-radius: 20px;
  background: rgba(245, 158, 11, 0.14);
}

/* ---------- 网格视图 ---------- */
.plugin-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(360px, 1fr));
  gap: 16px;
}

.plugin-card {
  display: flex;
  flex-direction: column;
  background: var(--glass-bg);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
  border: 1px solid var(--glass-border);
  border-radius: var(--radius-lg);
  padding: 18px;
  box-shadow: var(--glass-shadow);
  /* 仅动画 transform / box-shadow / border-color */
  transition: transform var(--dur) var(--ease), box-shadow var(--dur) var(--ease),
    border-color var(--dur) var(--ease);
}

.plugin-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 16px 40px rgba(0, 0, 0, 0.45);
  border-color: rgba(99, 102, 241, 0.45);
}

.card-update {
  border-color: rgba(245, 158, 11, 0.4);
  background: rgba(245, 158, 11, 0.06);
}

.card-update:hover {
  border-color: rgba(245, 158, 11, 0.7);
}

.card-disabled {
  opacity: 0.55;
}

.card-core {
  border-color: rgba(99, 102, 241, 0.4);
  background: rgba(99, 102, 241, 0.07);
}

.card-core:hover {
  border-color: rgba(99, 102, 241, 0.7);
}

.card-header {
  display: flex;
  align-items: flex-start;
  gap: 12px;
}

.card-icon {
  flex-shrink: 0;
  width: 42px;
  height: 42px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(255, 255, 255, 0.06);
  border: 1px solid var(--glass-border);
  border-radius: var(--radius-md);
}

.card-core .card-icon {
  background: rgba(99, 102, 241, 0.16);
  border-color: rgba(99, 102, 241, 0.32);
}

.card-title {
  flex: 1;
  min-width: 0;
}

.card-name {
  display: flex;
  align-items: center;
  gap: 6px;
  font-weight: 600;
  font-size: 14px;
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.core-name {
  color: var(--primary-light);
}

.card-id {
  font-size: 11px;
  color: var(--text-muted);
  margin-top: 2px;
  font-family: 'JetBrains Mono', 'Consolas', monospace;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.card-status {
  flex-shrink: 0;
}

/* 描述 */
.card-desc {
  margin: 14px 0;
  font-size: 13px;
  color: var(--text-secondary);
  line-height: 1.6;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
  min-height: 42px;
}

/* 版本信息 */
.card-versions {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 12px;
  background: rgba(255, 255, 255, 0.04);
  border: 1px solid var(--glass-border);
  border-radius: var(--radius-sm);
  margin-bottom: 12px;
  flex-wrap: wrap;
}

.card-update .card-versions {
  background: rgba(245, 158, 11, 0.1);
  border-color: rgba(245, 158, 11, 0.25);
}

.version-item {
  display: flex;
  align-items: center;
  gap: 5px;
}

.version-label {
  font-size: 11px;
  color: var(--text-muted);
}

.version-num {
  font-family: 'JetBrains Mono', 'Consolas', monospace;
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
}

.version-num.highlight {
  color: var(--warning);
}

.version-arrow {
  font-size: 14px;
}

.version-error {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 12px;
  color: var(--danger);
}

/* 链接 */
.card-links {
  display: flex;
  align-items: center;
  gap: 14px;
  margin-bottom: 14px;
  min-height: 20px;
}

.repo-name {
  font-size: 12px;
  max-width: 220px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.text-muted {
  font-size: 12px;
  color: var(--text-muted);
}

/* 操作区 */
.card-footer {
  margin-top: auto;
}

.card-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.card-actions .el-button {
  margin-left: 0;
}

.update-progress {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.progress-text {
  font-size: 11px;
  color: var(--text-muted);
  text-align: center;
}

/* ---------- 列表视图 ---------- */
.plugin-table-wrap {
  background: var(--glass-bg);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
  border: 1px solid var(--glass-border);
  border-radius: var(--radius-lg);
  box-shadow: var(--glass-shadow);
  overflow: auto;
}

.ptable {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
}

.ptable th,
.ptable td {
  padding: 12px 14px;
  text-align: left;
  border-bottom: 1px solid rgba(255, 255, 255, 0.06);
  vertical-align: middle;
}

.ptable thead th {
  background: rgba(15, 23, 42, 0.92);
  backdrop-filter: blur(8px);
  -webkit-backdrop-filter: blur(8px);
  color: var(--text-muted);
  font-weight: 600;
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.4px;
  white-space: nowrap;
  position: sticky;
  top: 0;
  z-index: 1;
}

.ptable tbody tr {
  transition: background-color var(--dur) var(--ease);
}

.ptable tbody tr:hover {
  background: rgba(99, 102, 241, 0.09);
}

.tr-update {
  background: rgba(245, 158, 11, 0.08);
}

.tr-disabled {
  opacity: 0.5;
}

.tr-core {
  background: rgba(99, 102, 241, 0.08);
}

.col-status { width: 84px; }
.col-name { min-width: 200px; }
.col-desc { min-width: 220px; max-width: 320px; }
.col-version { width: 180px; }
.col-github { min-width: 160px; }
.col-actions { width: 250px; white-space: nowrap; }

.cell-name {
  display: flex;
  align-items: center;
  gap: 10px;
}

.cell-title {
  font-weight: 500;
  color: var(--text-primary);
  display: flex;
  align-items: center;
  gap: 6px;
}

.cell-id {
  font-size: 11px;
  color: var(--text-muted);
  font-family: 'JetBrains Mono', 'Consolas', monospace;
}

.cell-desc {
  color: var(--text-secondary);
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.cell-version {
  display: flex;
  align-items: center;
  gap: 6px;
}

.cell-error {
  color: var(--danger);
  font-size: 12px;
}

.row-actions {
  display: flex;
  align-items: center;
  gap: 6px;
}

.row-actions .el-button {
  margin-left: 0;
}

/* 状态小标签（.mini-tag-*）样式统一由全局 main.css 提供，
   此处不再重复定义——避免 scoped 高优先级覆盖全局深色配色 */

/* 卡片/行内操作按钮：非主按钮统一玻璃描边风格 */
.card-actions .el-button:not(.el-button--primary),
.row-actions .el-button:not(.el-button--primary) {
  background: rgba(255, 255, 255, 0.04);
  border: 1px solid var(--glass-border);
  color: var(--text-secondary);
}

.card-actions .el-button:not(.el-button--primary):hover,
.row-actions .el-button:not(.el-button--primary):hover {
  background: rgba(255, 255, 255, 0.1);
  border-color: rgba(255, 255, 255, 0.22);
  color: var(--text-primary);
}

/* ---------- 加载遮罩 ---------- */
.loading-overlay {
  position: fixed;
  inset: 0;
  background: rgba(15, 23, 42, 0.72);
  backdrop-filter: blur(4px);
  -webkit-backdrop-filter: blur(4px);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 14px;
  color: var(--primary-light);
  font-size: 14px;
  z-index: 10;
}
.cat-badge {
  display: inline-block;
  font-size: 11px;
  line-height: 1.5;
  padding: 1px 8px;
  border-radius: 10px;
  border: 1px solid;
  margin-right: 6px;
  vertical-align: middle;
  white-space: nowrap;
}

.card-metrics {
  display: flex;
  gap: 14px;
  padding: 2px 0 6px;
}

.cell-metrics {
  display: flex;
  gap: 14px;
  margin-top: 4px;
}

.metric {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  font-size: 12px;
  color: var(--text-secondary, #8b93a7);
}

.metric .el-icon {
  font-size: 13px;
}
.category-filter {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  padding: 0 4px 10px;
}

.cat-chip {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  font-size: 12px;
  line-height: 1;
  padding: 5px 10px;
  border-radius: 12px;
  border: 1px solid var(--border-color, rgba(255, 255, 255, 0.12));
  background: rgba(255, 255, 255, 0.04);
  color: var(--text-secondary, #8b93a7);
  cursor: pointer;
  transition: all 0.15s ease;
  user-select: none;
}

.cat-chip:hover {
  border-color: var(--cat-color, #409eff);
  color: var(--cat-color, #409eff);
}

.cat-chip.active {
  background: color-mix(in srgb, var(--cat-color, #409eff) 18%, transparent);
  border-color: var(--cat-color, #409eff);
  color: var(--cat-color, #409eff);
  font-weight: 600;
}

.cat-chip.active:first-child {
  --cat-color: #409eff;
}
.market-tabs {
  --el-tabs-header-height: 40px;
}

.market-tabs :deep(.el-tabs__header) {
  margin-bottom: 10px;
}

.market-card.card-installed {
  border-color: rgba(103, 194, 58, 0.45);
}

.market-pager {
  display: flex;
  justify-content: center;
  padding: 14px 0 6px;
}

.updates-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 48px 0;
  color: var(--text-secondary, #8b93a7);
}
.market-search-row {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 0 4px 10px;
}

.market-search {
  max-width: 360px;
}

.search-count {
  font-size: 12px;
  color: var(--text-secondary, #8b93a7);
}

.sort-label {
  margin: 0 4px;
  font-size: 13px;
}

.sort-active {
  color: var(--el-color-primary);
  border-color: var(--el-color-primary);
}

.install-progress-row {
  display: flex;
  align-items: center;
  gap: 8px;
  color: var(--el-color-primary);
  padding: 8px 0;
}
.install-target-label {
  margin: 0 0 10px;
  font-size: 13px;
  color: var(--el-text-color-secondary);
}

/* 市场卡片链接下拉按钮：玻璃暗底上保证可见 */
.card-status .link-btn {
  color: var(--el-color-primary);
  border-color: var(--el-color-primary);
  background: rgba(255, 255, 255, 0.06);
}
.card-status .link-btn:hover {
  color: var(--el-color-primary-light-3);
  border-color: var(--el-color-primary-light-3);
}
/* 实心主色安装按钮：白字蓝底高对比 */
.card-status .el-button--primary {
  color: #ffffff;
}
/* 表格镜像链接与 GitHub 链接间距 */
.col-github .mirror-link {
  margin-left: 8px;
}
</style>

