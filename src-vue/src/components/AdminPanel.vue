<template>
  <ElDialog v-model="visible" title="兼容性管理" width="900px" :close-on-click-modal="false">
    <ElTabs v-model="activeTab">
      <!-- 兼容规则 -->
      <ElTabPane label="兼容规则" name="compat">
        <div class="admin-toolbar">
          <ElButton type="primary" @click="showCompatForm = true">新增规则</ElButton>
          <ElButton @click="loadCompatRules">刷新</ElButton>
        </div>
        <ElTable :data="compatRules" border stripe max-height="400">
          <ElTableColumn prop="plugin_id" label="插件 ID" width="200" />
          <ElTableColumn prop="dsh_version" label="DSH 版本" width="120" />
          <ElTableColumn prop="compatible" label="兼容性" width="100">
            <template #default="{ row }">
              <ElTag :type="row.compatible ? 'success' : 'danger'">
                {{ row.compatible ? '兼容' : '不兼容' }}
              </ElTag>
            </template>
          </ElTableColumn>
          <ElTableColumn prop="note" label="说明" />
          <ElTableColumn label="操作" width="100">
            <template #default="{ row }">
              <ElButton type="danger" size="small" @click="deleteCompatRule(row)">删除</ElButton>
            </template>
          </ElTableColumn>
        </ElTable>
      </ElTabPane>

      <!-- 冲突规则 -->
      <ElTabPane label="冲突规则" name="conflicts">
        <div class="admin-toolbar">
          <ElButton type="primary" @click="showConflictForm = true">新增冲突</ElButton>
          <ElButton @click="loadConflicts">刷新</ElButton>
        </div>
        <ElTable :data="conflicts" border stripe max-height="400">
          <ElTableColumn prop="plugin_id" label="插件 ID" width="200" />
          <ElTableColumn prop="conflict_with" label="冲突对象" width="200" />
          <ElTableColumn prop="severity" label="级别" width="80">
            <template #default="{ row }">
              <ElTag :type="row.severity === 'block' ? 'danger' : 'warning'">
                {{ row.severity === 'block' ? '阻塞' : '警告' }}
              </ElTag>
            </template>
          </ElTableColumn>
          <ElTableColumn prop="reason" label="原因" />
          <ElTableColumn label="操作" width="100">
            <template #default="{ row }">
              <ElButton type="danger" size="small" @click="deleteConflict(row.id)">删除</ElButton>
            </template>
          </ElTableColumn>
        </ElTable>
      </ElTabPane>
    </ElTabs>

    <!-- 新增/编辑兼容规则表单 -->
    <ElDialog v-model="showCompatForm" title="新增兼容规则" width="500px">
      <ElForm :model="compatForm" label-width="100px">
        <ElFormItem label="插件 ID">
          <ElInput v-model="compatForm.plugin_id" placeholder="如：dsh-agent-core" />
        </ElFormItem>
        <ElFormItem label="DSH 版本">
          <ElInput v-model="compatForm.dsh_version" placeholder="如：* 或 0.1.1-rc.2" />
        </ElFormItem>
        <ElFormItem label="兼容性">
          <ElSwitch v-model="compatForm.compatible" :active-text="'兼容'" :inactive-text="'不兼容'" />
        </ElFormItem>
        <ElFormItem label="说明">
          <ElInput v-model="compatForm.note" type="textarea" :rows="3" />
        </ElFormItem>
      </ElForm>
      <template #footer>
        <ElButton @click="showCompatForm = false">取消</ElButton>
        <ElButton type="primary" @click="submitCompatRule">保存</ElButton>
      </template>
    </ElDialog>

    <!-- 新增/编辑冲突规则表单 -->
    <ElDialog v-model="showConflictForm" title="新增冲突规则" width="500px">
      <ElForm :model="conflictForm" label-width="100px">
        <ElFormItem label="插件 ID">
          <ElInput v-model="conflictForm.plugin_id" placeholder="如：dsh-agent-core" />
        </ElFormItem>
        <ElFormItem label="冲突对象">
          <ElInput v-model="conflictForm.conflict_with" placeholder="如：dsh-plugin-comfyui" />
        </ElFormItem>
        <ElFormItem label="级别">
          <ElRadio-group v-model="conflictForm.severity">
            <ElRadio value="warn">警告</ElRadio>
            <ElRadio value="block">阻塞</ElRadio>
          </ElRadio-group>
        </ElFormItem>
        <ElFormItem label="原因">
          <ElInput v-model="conflictForm.reason" type="textarea" :rows="3" />
        </ElFormItem>
      </ElForm>
      <template #footer>
        <ElButton @click="showConflictForm = false">取消</ElButton>
        <ElButton type="primary" @click="submitConflict">保存</ElButton>
      </template>
    </ElDialog>
  </ElDialog>
</template>

<script setup lang="ts">
import { ref, watch, defineProps, defineEmits } from 'vue'
import { ElMessage } from 'element-plus'
import { pluginApi } from '../api'

const props = defineProps<{ modelValue: boolean }>()
const emit = defineEmits<{ (e: 'update:modelValue', val: boolean): void }>()

const visible = ref(false)
watch(() => props.modelValue, v => { visible.value = v; if (v) { loadCompatRules(); loadConflicts(); } })
watch(visible, v => { if (!v) emit('update:modelValue', false); })

const activeTab = ref('compat')

// 兼容规则
const compatRules = ref<any[]>([])
const showCompatForm = ref(false)
const compatForm = ref({ plugin_id: '', dsh_version: '*', compatible: true, note: '' })

async function loadCompatRules() {
  try {
    const resp = await fetch('/api/compat/rules')
    compatRules.value = await resp.json()
  } catch (e) {
    ElMessage.error('加载兼容规则失败')
  }
}

async function submitCompatRule() {
  if (!compatForm.value.plugin_id) {
    ElMessage.warning('请填写插件 ID')
    return
  }
  try {
    await fetch('/api/compat/check', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(compatForm.value),
    })
    ElMessage.success('保存成功')
    showCompatForm.value = false
    loadCompatRules()
  } catch (e) {
    ElMessage.error('保存失败')
  }
}

async function deleteCompatRule(row: any) {
  try {
    await fetch(`/api/compat/check?plugin_id=${encodeURIComponent(row.plugin_id)}&dsh_version=${encodeURIComponent(row.dsh_version || '*')}`, {
      method: 'DELETE',
    })
    ElMessage.success('删除成功')
    loadCompatRules()
  } catch (e) {
    ElMessage.error('删除失败')
  }
}

// 冲突规则
const conflicts = ref<any[]>([])
const showConflictForm = ref(false)
const conflictForm = ref({ plugin_id: '', conflict_with: '', severity: 'warn', reason: '' })

async function loadConflicts() {
  try {
    const resp = await fetch('/api/conflicts')
    conflicts.value = await resp.json()
  } catch (e) {
    ElMessage.error('加载冲突规则失败')
  }
}

async function submitConflict() {
  if (!conflictForm.value.plugin_id || !conflictForm.value.conflict_with) {
    ElMessage.warning('请填写插件 ID 和冲突对象')
    return
  }
  try {
    await fetch('/api/conflicts', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(conflictForm.value),
    })
    ElMessage.success('保存成功')
    showConflictForm.value = false
    loadConflicts()
  } catch (e) {
    ElMessage.error('保存失败')
  }
}

async function deleteConflict(id: number) {
  try {
    await fetch(`/api/conflicts?id=${id}`, { method: 'DELETE' })
    ElMessage.success('删除成功')
    loadConflicts()
  } catch (e) {
    ElMessage.error('删除失败')
  }
}
</script>

<style scoped>
.admin-toolbar {
  margin-bottom: 16px;
}
</style>
