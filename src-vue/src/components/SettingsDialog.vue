<template>
  <el-dialog
    v-model="visible"
    :title="t('settings.title')"
    width="600px"
    :close-on-click-modal="false"
    @close="handleClose"
  >
    <el-form ref="formRef" :model="formData" :rules="rules" label-width="150px">
      <!-- {{ t('settings.proxyGroup') }} -->
      <div class="setting-group">
        <div class="group-title">
          <el-icon><Connection /></el-icon>
          {{ t('settings.network') }}
        </div>

        <el-form-item :label="t('settings.proxyUrl')" prop="proxy_base_url">
          <el-input
            v-model="formData.proxy_base_url"
            :placeholder="t('settings.proxyPlaceholder')"
          >
            <template #prefix>
              <el-icon><Link /></el-icon>
            </template>
          </el-input>
          <div class="form-tip">{{ t('settings.proxyTip') }}</div>
        </el-form-item>

        <el-form-item :label="t('settings.installRegistry')">
          <el-select v-model="registryMode" style="width: 100%" @change="onRegistryModeChange">
            <el-option :label="t('settings.registryOfficial')" value="official" />
            <el-option :label="t('settings.registryMirror')" value="mirror" />
            <el-option :label="t('settings.registryCustom')" value="custom" />
          </el-select>
          <el-input
            v-if="registryMode === 'custom'"
            v-model="formData.install_registry"
            placeholder="https://registry.example.com/"
            style="margin-top: 8px"
          />
          <div class="form-tip">{{ t('settings.installRegistryTip') }}</div>
        </el-form-item>      </div>

      <!-- 插件目录 -->
      <div class="setting-group">
        <div class="group-title">
          <el-icon><FolderOpened /></el-icon>
          插件目录
        </div>

        <el-form-item :label="t('settings.defaultDir')">
          <el-input
            v-model="formData.plugin_directory"
            :placeholder="t('settings.dirPlaceholder')"
          >
            <template #prefix>
              <el-icon><Folder /></el-icon>
            </template>
          </el-input>
          <div class="form-tip">{{ t('settings.dirTip') }}</div>
        </el-form-item>
      </div>

      <!-- 更新设置 -->
      <div class="setting-group">
        <div class="group-title">
          <el-icon><Refresh /></el-icon>
          更新设置
        </div>

        <div class="switch-row">
          <div class="row-info">
            <div class="row-label">{{ t('settings.autoCheck') }}</div>
            <div class="row-desc">{{ t('settings.autoCheckDesc') }}</div>
          </div>
          <el-switch v-model="formData.auto_check_updates" />
        </div>

        <div class="switch-row">
          <div class="row-info">
            <div class="row-label">{{ t('settings.autoBackup') }}</div>
            <div class="row-desc">{{ t('settings.autoBackupDesc') }}</div>
          </div>
          <el-switch v-model="formData.backup_before_update" />
        </div>
      </div>

      <!-- 服务器同步 -->
      <div class="setting-group">
        <div class="group-title">
          <el-icon><Monitor /></el-icon>
          {{ t('settings.serverGroup') }}
        </div>

        <el-form-item :label="t('settings.serverHost')">
          <el-input v-model="formData.server_host" :placeholder="t('settings.serverHostPh')" />
        </el-form-item>
        <el-form-item :label="t('settings.serverPort')">
          <el-input-number v-model="formData.server_port" :min="1" :max="65535" style="width: 140px" />
        </el-form-item>
        <el-form-item :label="t('settings.serverUser')">
          <el-input v-model="formData.server_user" :placeholder="t('settings.serverUserPh')" />
        </el-form-item>
        <el-form-item :label="t('settings.serverKey')">
          <el-input v-model="formData.server_key" placeholder="~/.ssh/id_ed25519" />
        </el-form-item>
        <el-form-item :label="t('settings.serverRemoteDir')">
          <el-input v-model="formData.server_remote_dir" :placeholder="t('settings.serverRemoteDirPh')" />
        </el-form-item>
        <el-form-item :label="t('settings.serverDshDir')">
          <el-input v-model="formData.server_dsh_dir" :placeholder="t('settings.serverDshDirPh')" />
        </el-form-item>
        <el-form-item :label="t('settings.serverUpdateCmd')">
          <el-input v-model="formData.server_update_cmd" :placeholder="t('settings.serverUpdateCmdPh')" />
        </el-form-item>

        <div class="server-actions">
          <el-button size="small" :loading="testing" :icon="Connection" @click="testServer">{{ t('settings.serverTest') }}</el-button>
          <el-button size="small" type="primary" plain :loading="syncing === 'app'" :icon="Upload" @click="syncServer('app')">{{ t('settings.syncApp') }}</el-button>
          <el-button size="small" type="primary" plain :loading="syncing === 'catalog'" :icon="Refresh" @click="syncServer('catalog')">{{ t('settings.syncCatalog') }}</el-button>
          <el-button size="small" type="warning" plain :loading="syncing === 'plugins'" :icon="Upload" @click="syncServer('plugins')">{{ t('settings.syncPlugins') }}</el-button>
        </div>
      </div>

      <!-- 关于 -->
      <div class="setting-group">
        <div class="group-title">
          <el-icon><InfoFilled /></el-icon>
          关于
        </div>

        <div class="switch-row">
          <div class="row-info">
            <div class="row-label">{{ t('settings.version') }}</div>
            <div class="row-desc">{{ t('settings.currentVersion') }}</div>
          </div>
          <el-button size="small" @click="checkAppUpdate">
            <el-icon><Refresh /></el-icon>
            {{ t('settings.checkAppUpdate') }}
          </el-button>
        </div>

        <div class="switch-row">
          <div class="row-info">
            <div class="row-label">{{ t('settings.website') }}</div>
            <div class="row-desc">{{ t('settings.websiteDesc') }}</div>
          </div>
          <el-link type="primary" :underline="false" @click.prevent="openWebsite">
            dsh.huilinsh.cn
            <el-icon class="link-icon"><Link /></el-icon>
          </el-link>
        </div>
      </div>
    </el-form>

    <template #footer>
      <el-button @click="handleClose">{{ t('common.cancel') }}</el-button>
      <el-button type="primary" :loading="saving" @click="handleSave">{{ t('common.save') }}</el-button>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, watch, reactive } from 'vue'
import { ElMessage, type FormInstance, type FormRules } from 'element-plus'
import {
  Link,
  Folder,
  FolderOpened,
  Connection,
  Refresh,
  InfoFilled,
} from '@element-plus/icons-vue'
import type { AppConfig } from '../types'
import { pluginApi } from '../api'
import { t } from '../i18n'

const props = defineProps<{
  modelValue: boolean
  config: AppConfig | null
}>()

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
  save: [config: AppConfig]
}>()

const visible = ref(props.modelValue)
const testing = ref(false)
const syncing = ref<string | null>(null)

async function testServer() {
  testing.value = true
  try {
    const res = await pluginApi.testServerConnection()
    ElMessage.success(t('settings.serverTestOk') + ': ' + res)
  } catch (e) {
    ElMessage.error(e?.toString() || t('settings.serverTestFail'))
  } finally {
    testing.value = false
  }
}

async function syncServer(kind: 'app' | 'catalog' | 'plugins') {
  syncing.value = kind
  try {
    const res = await pluginApi.syncToServer(kind)
    ElMessage.success(res)
  } catch (e) {
    ElMessage.error(e?.toString() || t('settings.syncFail'))
  } finally {
    syncing.value = null
  }
}
const formRef = ref<FormInstance>()
const saving = ref(false)

const formData = reactive<AppConfig>({
  proxy_base_url: '',
  plugin_directory: '',
  auto_check_updates: true,
  backup_before_update: true,
})

const MIRROR_REGISTRY = 'https://registry.npmmirror.com'
const registryMode = ref<'official' | 'mirror' | 'custom'>('official')

function syncRegistryMode(v: string) {
  registryMode.value = !v ? 'official' : v === MIRROR_REGISTRY ? 'mirror' : 'custom'
}

function onRegistryModeChange(mode: string) {
  if (mode === 'official') {
    formData.install_registry = ''
  } else if (mode === 'mirror') {
    formData.install_registry = MIRROR_REGISTRY
  } else if (mode === 'custom' && !formData.install_registry) {
    formData.install_registry = 'https://'
  }
}

const rules: FormRules = {
  install_registry: [
    {
      validator: (_r: unknown, value: string, cb: (err?: Error) => void) => {
        if (registryMode.value === 'custom' && value && !/^https?:\/\//.test(value)) {
          cb(new Error(t('settings.registryFormat')))
        } else {
          cb()
        }
      },
      trigger: 'blur',
    },
  ],
  proxy_base_url: [
    {
      // 代理留空合法（本地直连）；仅填写时才校验 http(s) 格式，空值自动跳过 pattern
      pattern: /^https?:\/\//,
      message: t('settings.proxyFormat'),
      trigger: 'blur',
    },
  ],
}

watch(
  () => props.modelValue,
  (val) => {
    visible.value = val
    if (val && props.config) {
      Object.assign(formData, props.config)
      syncRegistryMode(formData.install_registry || '')
    }
  }
)

watch(visible, (val) => {
  emit('update:modelValue', val)
})

// 官网链接：统一走系统浏览器
function openWebsite() {
  pluginApi.openExternal('https://dsh.huilinsh.cn').catch(() => {})
}
function handleClose() {
  visible.value = false
}

async function handleSave() {
  if (!formRef.value) return

  try {
    await formRef.value.validate()
    saving.value = true
    emit('save', { ...formData })
  } catch (e) {
    // 验证失败，保持对话框打开
  } finally {
    saving.value = false
  }
}

function checkAppUpdate() {
  ElMessage.info(t('settings.checkAppUpdate') + '...')
  // 实际实现中会请求官网版本接口
}
</script>

<style scoped>
/* ---------- 分组容器 ---------- */
.setting-group {
  padding: 16px 18px;
  margin-bottom: 14px;
  border-radius: var(--radius-md);
  background: rgba(255, 255, 255, 0.03);
  border: 1px solid var(--glass-border);
}

.setting-group:last-child {
  margin-bottom: 0;
}

.group-title {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  font-weight: 600;
  color: var(--primary-light);
  margin-bottom: 14px;
  letter-spacing: 0.3px;
}

.group-title .el-icon {
  font-size: 15px;
}

/* ---------- 表单项 ---------- */
.form-tip {
  font-size: 12px;
  color: var(--text-muted);
  margin-top: 6px;
  line-height: 1.5;
}

/* 开关行：左信息 + 右控件 */
.switch-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  padding: 10px 0;
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
}

.switch-row:last-child {
  border-bottom: none;
  padding-bottom: 0;
}

.row-info {
  min-width: 0;
}

.row-label {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
}

.row-desc {
  font-size: 12px;
  color: var(--text-muted);
  margin-top: 2px;
  line-height: 1.5;
}

.link-icon {
  margin-left: 4px;
}

/* 表单标签深色 */
:deep(.el-form-item__label) {
  color: var(--text-secondary);
  font-size: 13px;
}

:deep(.el-form-item__error) {
  color: var(--danger);
}

.native-cred-input {
  width: 100%;
  height: 30px;
  padding: 0 10px;
  border: 1px solid #dcdfe6;
  border-radius: 4px;
  font-size: 13px;
  color: #606266;
  outline: none;
  box-sizing: border-box;
}
.native-cred-input:focus {
  border-color: #409eff;
}
.server-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  padding-left: 150px;
}
</style>
