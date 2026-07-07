<template>
  <div>
    <el-card>
      <template #header>
        <div class="flex-between">
          <span>操作日志</span>
          <div class="btn-group">
            <el-select v-model="filterModule" placeholder="全部模块" clearable style="width: 120px" @change="loadData">
              <el-option label="客户" value="客户" />
              <el-option label="项目" value="项目" />
              <el-option label="发票" value="发票" />
              <el-option label="垫资" value="垫资" />
              <el-option label="支出" value="支出" />
              <el-option label="回款" value="回款" />
              <el-option label="员工" value="员工" />
              <el-option label="登录" value="登录" />
            </el-select>
            <el-select v-model="filterAction" placeholder="全部操作" clearable style="width: 120px" @change="loadData">
              <el-option label="新增" value="新增" />
              <el-option label="编辑" value="编辑" />
              <el-option label="删除" value="删除" />
              <el-option label="登录" value="登录" />
              <el-option label="导入" value="导入" />
            </el-select>
          </div>
        </div>
      </template>

      <el-table :data="tableData" border v-loading="loading" header-cell-class-name="header-center" cell-class-name="cell-center">
        <el-table-column type="index" label="序号" width="60" />
        <el-table-column prop="operator" label="操作人" width="100" align="center" />
        <el-table-column prop="action" label="操作类型" width="100" align="center">
          <template #default="{ row }">
            <el-tag :type="getActionType(row.action)" size="small">{{ row.action }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="module" label="模块" width="100" align="center">
          <template #default="{ row }">
            <el-tag type="info" size="small">{{ row.module }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="content" label="操作内容" min-width="250" show-overflow-tooltip />
        <el-table-column prop="operationTime" label="操作时间" width="170" align="center">
          <template #default="{ row }">
            {{ formatTime(row.operationTime) }}
          </template>
        </el-table-column>
      </el-table>

      <Pagination
        :total="total"
        :current-page="currentPage"
        :page-size="pageSize"
        @update:currentPage="currentPage = $event"
      />
    </el-card>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import api from '../api/index'
import Pagination from '../components/Pagination.vue'

const loading = ref(false)
const tableData = ref([])
const filterModule = ref('')
const filterAction = ref('')
const currentPage = ref(1)
const pageSize = ref(20)
const total = ref(0)

const getActionType = (action) => {
  const types = { '新增': 'success', '编辑': 'primary', '删除': 'danger', '导入': 'warning', '登录': 'info' }
  return types[action] || 'info'
}

const formatTime = (time) => {
  if (!time) return ''
  return time.replace('T', ' ').slice(0, 19)
}

const loadData = async () => {
  loading.value = true
  try {
    let logs
    if (filterModule.value) {
      logs = await api.get(`/operation-logs/module/${filterModule.value}`)
    } else {
      logs = await api.get('/operation-logs')
    }

    // 按操作类型筛选
    if (filterAction.value) {
      logs = logs.filter(log => log.action === filterAction.value)
    }

    tableData.value = logs
    total.value = logs.length
  } catch (e) {
    console.error(e)
  }
  loading.value = false
}

onMounted(() => {
  loadData()
})
</script>

<style scoped>
.flex-between { display: flex; justify-content: space-between; align-items: center; }
.btn-group { display: flex; gap: 12px; }
</style>
