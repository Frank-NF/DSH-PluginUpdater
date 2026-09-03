<template>
  <WDialog
    v-model="visible"
    :title="t('settings.title')"
    wide
    :close-on-mask="false"
  >
    <div class="weui-form w-form">
      <!-- ============ 代理设置 ============ -->
      <div class="weui-cells__group weui-cells__group_form">
        <div class="weui-cells__title">{{ t('settings.network') }}</div>
        <div class="weui-cells">
          <div class="weui-cell weui-cell_active">
            <div class="weui-cell__hd">
              <label class="weui-label">{{ t('settings.proxyUrl') }}</label>
            </div>
            <div class="weui-cell__bd">
              <input
                v-model="formData.proxy_base_url"
                class="weui-input"
                type="text"
                :placeholder="t('settings.proxyPlaceholder')"
                @blur="validate"
              />
            </div>
          </div>

          <div class="weui-cell weui-cell_active">
            <div class="weui-cell__hd">
              <label class="weui-label">{{ t('settings.defaultDir') }}</label>
            </div>
            <div class="weui-cell__bd w-dir-cell">
              <input
                v-model="formData.plugin_directory"
                class="weui-input"
                type="text"
                :placeholder="t('settings.dirPlaceholder')"
                spellcheck="false"
              />
              <WButton size="mini" icon="folder" :title="t('settings.dirPickTip')" @click="pickDir">
                {{ t('settings.dirPick') }}
              </WButton>
            </div>
          </div>
        </div>
        <div v-if="errors.proxy || errors.port" class="weui-cells__tips weui-cells__tips_warn">
          {{ errors.proxy || errors.port }}
        </div>
        <div v-else class="weui-cells__tips">
          {{ t('settings.proxyTip') }}<br />{{ t('settings.dirTip') }}
        </div>
      </div>

      <!-- ============ 安装源 ============ -->
      <div class="weui-cells__group weui-cells__group_form">
        <div class="weui-cells__title">{{ t('settings.installRegistry') }}</div>
        <div class="weui-cells weui-cells_radio">
          <label
            v-for="opt in registryOptions"
            :key="opt.value"
            class="weui-cell weui-cell_active weui-check__label"
          >
            <div class="weui-cell__bd"><p>{{ opt.label }}</p></div>
            <div class="weui-cell__ft">
              <input
                v-model="registryMode"
                type="radio"
                class="weui-check"
                name="registry"
                :value="opt.value"
                @change="onRegistryModeChange"
              />
              <span class="weui-icon-checked" aria-hidden="true" />
            </div>
          </label>
        </div>

        <div v-if="registryMode === 'custom'" class="weui-cells">
          <div class="weui-cell weui-cell_active">
            <div class="weui-cell__bd">
              <input
                v-model="formData.install_registry"
                class="weui-input"
                type="text"
                placeholder="https://registry.example.com/"
                @blur="validate"
              />
            </div>
          </div>
        </div>

        <div v-if="errors.registry" class="weui-cells__tips weui-cells__tips_warn">
          {{ errors.registry }}
        </div>
        <div v-else class="weui-cells__tips">{{ t('settings.installRegistryTip') }}</div>
      </div>

      <!-- ============ 更新设置 ============ -->
      <div class="weui-cells__group weui-cells__group_form">
        <div class="weui-cells__title">{{ t('settings.updates') }}</div>
        <div class="weui-cells">
          <div class="weui-cell weui-cell_active weui-cell_switch">
            <div class="weui-cell__bd">
              <p class="w-switch-title">{{ t('settings.autoCheck') }}</p>
              <p class="w-switch-desc">{{ t('settings.autoCheckDesc') }}</p>
            </div>
            <div class="weui-cell__ft">
              <input v-model="formData.auto_check_updates" class="weui-switch" type="checkbox" />
            </div>
          </div>

          <div class="weui-cell weui-cell_active weui-cell_switch">
            <div class="weui-cell__bd">
              <p class="w-switch-title">{{ t('settings.autoBackup') }}</p>
              <p class="w-switch-desc">{{ t('settings.autoBackupDesc') }}</p>
            </div>
            <div class="weui-cell__ft">
              <input v-model="formData.backup_before_update" class="weui-switch" type="checkbox" />
            </div>
          </div>
        </div>
      </div>

      <!-- ============ 服务器同步 ============ -->
      <div class="weui-cells__group weui-cells__group_form">
        <div class="weui-cells__title">{{ t('settings.serverGroup') }}</div>
        <div class="weui-cells">
          <div class="weui-cell weui-cell_active">
            <div class="weui-cell__hd">
              <label class="weui-label">{{ t('settings.serverHost') }}</label>
            </div>
            <div class="weui-cell__bd">
              <input
                v-model="formData.server_host"
                class="weui-input"
                type="text"
                :placeholder="t('settings.serverHostPh')"
              />
            </div>
          </div>

          <div class="weui-cell weui-cell_active">
            <div class="weui-cell__hd">
              <label class="weui-label">{{ t('settings.serverPort') }}</label>
            </div>
            <div class="weui-cell__bd">
              <input
                v-model.number="formData.server_port"
                class="weui-input"
                type="number"
                min="1"
                max="65535"
              />
            </div>
          </div>

          <div class="weui-cell weui-cell_active">
            <div class="weui-cell__hd">
              <label class="weui-label">{{ t('settings.serverUser') }}</label>
            </div>
            <div class="weui-cell__bd">
              <input
                v-model="formData.server_user"
                class="weui-input"
                type="text"
                :placeholder="t('settings.serverUserPh')"
              />
            </div>
          </div>

          <div class="weui-cell weui-cell_active">
            <div class="weui-cell__hd">
              <label class="weui-label">{{ t('settings.serverKey') }}</label>
            </div>
            <div class="weui-cell__bd">
              <input
                v-model="formData.server_key"
                class="weui-input"
                type="text"
                placeholder="~/.ssh/id_ed25519"
              />
            </div>
          </div>

          <div class="weui-cell weui-cell_active">
            <div class="weui-cell__hd">
              <label class="weui-label">{{ t('settings.serverRemoteDir') }}</label>
            </div>
            <div class="weui-cell__bd">
              <input
                v-model="formData.server_remote_dir"
                class="weui-input"
                type="text"
                :placeholder="t('settings.serverRemoteDirPh')"
              />
            </div>
          </div>

          <div class="weui-cell weui-cell_active">
            <div class="weui-cell__hd">
              <label class="weui-label">{{ t('settings.serverDshDir') }}</label>
            </div>
            <div class="weui-cell__bd">
              <input
                v-model="formData.server_dsh_dir"
                class="weui-input"
                type="text"
                :placeholder="t('settings.serverDshDirPh')"
              />
            </div>
          </div>

          <div class="weui-cell weui-cell_active">
            <div class="weui-cell__hd">
              <label class="weui-label">{{ t('settings.serverUpdateCmd') }}</label>
            </div>
            <div class="weui-cell__bd">
              <input
                v-model="formData.server_update_cmd"
                class="weui-input"
                type="text"
                :placeholder="t('settings.serverUpdateCmdPh')"
              />
            </div>
          </div>
        </div>

        <div class="w-server-actions">
          <WButton size="mini" icon="link" :loading="testing" @click="testServer">
            {{ t('settings.serverTest') }}
          </WButton>
          <WButton
            size="mini"
            icon="upload"
            :loading="syncing === 'app'"
            @click="syncServer('app')"
          >
            {{ t('settings.syncApp') }}
          </WButton>
          <WButton
            size="mini"
            icon="refresh"
            :loading="syncing === 'catalog'"
            @click="syncServer('catalog')"
          >
            {{ t('settings.syncCatalog') }}
          </WButton>
          <WButton
            size="mini"
            icon="layers"
            :loading="syncing === 'plugins'"
            @click="syncServer('plugins')"
          >
            {{ t('settings.syncPlugins') }}
          </WButton>
        </div>
      </div>

      <!-- ============ 关于 ============ -->
      <div class="weui-cells__group weui-cells__group_form">
        <div class="weui-cells__title">{{ t('settings.about') }}</div>
        <div class="weui-cells">
          <div class="weui-cell">
            <div class="weui-cell__bd">
              <p>{{ t('settings.version') }}</p>
              <p class="w-switch-desc mono">{{ t('settings.currentVersion') }} v{{ appVersion || '…' }}</p>
            </div>
            <div class="weui-cell__ft">
              <WButton size="mini" icon="refresh" :loading="checkingUpdate" @click="checkAppUpdate">
                {{ t('settings.checkAppUpdate') }}
              </WButton>
            </div>
          </div>

          <a class="weui-cell weui-cell_access" href="javascript:" @click="openWebsite">
            <div class="weui-cell__bd">
              <p>{{ t('settings.website') }}</p>
              <p class="w-switch-desc">{{ t('settings.websiteDesc') }}</p>
            </div>
            <div class="weui-cell__ft w-text-2">dsh.huilinsh.cn</div>
          </a>
        </div>
      </div>
    </div>

    <template #footer>
      <WButton @click="emit('open-mcp')">{{ t('mcp.entry') }}</WButton>
      <WButton @click="emit('open-snapshot')">{{ t('snapshot.entry') }}</WButton>
      <WButton @click="handleClose">{{ t('common.cancel') }}</WButton>
      <WButton type="primary" :loading="saving" @click="handleSave">
        {{ t('common.save') }}
      </WButton>
    </template>
  </WDialog>
</template>

<script setup lang="ts">
import { reactive, ref, watch } from 'vue'
import WDialog from './WDialog.vue'
import WButton from './WButton.vue'
import { pluginApi } from '../api'
import { t } from '../i18n'
import { useAppVersion } from '../composables/useAppVersion'
import { useToast } from '../composables/useToast'
import { useConfirm } from '../composables/useConfirm'
import type { AppConfig } from '../types'

const props = defineProps<{
  modelValue: boolean
  config: AppConfig | null
}>()

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
  save: [config: AppConfig, done: (err?: unknown) => void]
  'open-mcp': [],
  'open-snapshot': []
}>()

const toast = useToast()
const { appVersion } = useAppVersion()
const { confirm } = useConfirm()
const visible = ref(props.modelValue)
const testing = ref(false)
const syncing = ref<string | null>(null)
const saving = ref(false)

const MIRROR_REGISTRY = 'https://registry.npmmirror.com'

const formData = reactive<AppConfig>({
  server_host: '',
  server_port: 22,
  server_user: '',
  server_key: '',
  server_remote_dir: '',
  server_dsh_dir: '',
  server_update_cmd: '',
  proxy_base_url: '',
  plugin_directory: '',
  auto_check_updates: true,
  backup_before_update: true,
  install_registry: '',
})

const registryMode = ref<'official' | 'mirror' | 'custom'>('official')
const errors = ref<{ proxy?: string; registry?: string; port?: string }>({})

const registryOptions = [
  { label: t('settings.registryOfficial'), value: 'official' as const },
  { label: t('settings.registryMirror'), value: 'mirror' as const },
  { label: t('settings.registryCustom'), value: 'custom' as const },
]

function syncRegistryMode(v: string) {
  registryMode.value = !v ? 'official' : v === MIRROR_REGISTRY ? 'mirror' : 'custom'
}

function onRegistryModeChange() {
  if (registryMode.value === 'official') {
    formData.install_registry = ''
  } else if (registryMode.value === 'mirror') {
    formData.install_registry = MIRROR_REGISTRY
  } else if (!formData.install_registry) {
    formData.install_registry = 'https://'
  }
}

/** 轻量校验：代理与自定义源都允许留空，填写时才校验协议头 */
function validate(): boolean {
  const next: { proxy?: string; registry?: string; port?: string } = {}
  const proxy = formData.proxy_base_url?.trim()
  if (proxy && !/^https?:\/\//.test(proxy)) {
    next.proxy = t('settings.proxyFormat')
  }
  if (registryMode.value === 'custom') {
    const reg = formData.install_registry?.trim()
    if (reg && !/^https?:\/\//.test(reg)) {
      next.registry = t('settings.registryFormat')
    }
  }
  const port = Number(formData.server_port)
  if (!Number.isInteger(port) || port < 1 || port > 65535) {
    next.port = t('settings.portInvalid')
  }
  errors.value = next
  return Object.keys(next).length === 0
}

async function pickDir() {
  try {
    const dir = await pluginApi.pickDirectory()
    if (!dir) return
    formData.plugin_directory = dir
  } catch (e) {
    toast.error(String(e))
  }
}

async function persistForm() {
  // 测试/同步走后端已保存配置——先把当前表单落盘，避免测到旧值
  await pluginApi.updateConfig({ ...formData })
}

async function testServer() {
  testing.value = true
  try {
    await persistForm()
    const res = await pluginApi.testServerConnection()
    toast.success(t('settings.serverTestOk') + ': ' + res)
  } catch (e) {
    toast.error(String(e) || t('settings.serverTestFail'))
  } finally {
    testing.value = false
  }
}

async function syncServer(kind: 'app' | 'catalog' | 'plugins') {
  syncing.value = kind
  try {
    await persistForm()
    const res = await pluginApi.syncToServer(kind)
    toast.success(res)
  } catch (e) {
    toast.error(String(e) || t('settings.syncFail'))
  } finally {
    syncing.value = null
  }
}

const checkingUpdate = ref(false)

async function checkAppUpdate() {
  checkingUpdate.value = true
  try {
    const info = await pluginApi.checkSelfUpdate()
    if (!info.available) {
      toast.success(t('settings.upToDate') + ' (v' + info.current_version + ')')
      return
    }
    const ok = await confirm({
      title: t('settings.newVersion'),
      message: 'v' + info.latest_version + (info.changelog && info.changelog.length ? '\n' + info.changelog.slice(0, 5).join('\n') : ''),
      confirmText: t("settings.selfUpdateNow"),
      cancelText: t('common.cancel'),
    })
    if (!ok) return
    const msg = await pluginApi.selfUpdate()
    toast.success(msg || t("settings.selfUpdateDone"))
  } catch (e) {
    toast.error(String(e) || t('settings.selfUpdateFail'))
  } finally {
    checkingUpdate.value = false
  }
}

function openWebsite() {
  pluginApi.openExternal('https://dsh.huilinsh.cn').catch(() => {})
}

function handleClose() {
  visible.value = false
}

function handleSave() {
  if (!validate()) return
  saving.value = true
  emit('save', { ...formData }, (err) => {
    saving.value = false
    if (err) return
    visible.value = false
  })
}

watch(
  () => props.modelValue,
  (val) => {
    visible.value = val
    if (val && props.config) {
      Object.assign(formData, props.config)
      syncRegistryMode(formData.install_registry || '')
      errors.value = {}
    }
  }
)

watch(visible, (val) => emit('update:modelValue', val))
</script>

<style scoped>
.w-form {
  padding: 0;
  min-height: auto;
  text-align: left;
}

/* 开关行的标题与说明 */
.w-switch-title {
  font-size: 14px;
  color: var(--fg);
}

.w-switch-desc {
  font-size: 12px;
  color: var(--fg-2);
  margin-top: 2px;
  line-height: 1.5;
}

.w-server-actions {
  display: flex;
  flex-wrap: wrap;
  gap: var(--sp-2);
  padding: var(--sp-3) var(--sp-4) 0;
}

/* 桌面端表单标签加宽，避免中文换行 */
@media (min-width: 768px) {
  .weui-label {
    width: 105px;
  }
}

/* 目录行：只读输入框 + 选择按钮 */
.w-dir-cell {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
}

.w-dir-cell .weui-input {
  flex: 1 1 auto;
  min-width: 0;
}

.w-dir-cell .weui-btn {
  flex: 0 0 auto;
}
</style>
