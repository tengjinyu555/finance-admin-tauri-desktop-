<template>
  <div>
    <!-- 统计卡片 -->
    <el-row :gutter="16" class="stats-row">
      <el-col :span="8">
        <div class="stat-card blue" @click="router.push('/customer')">
          <div class="stat-content">
            <div class="stat-value">{{ stats.customerCount }}</div>
            <div class="stat-label">客户总数</div>
          </div>
          <div class="stat-icon"><el-icon size="40"><OfficeBuilding /></el-icon></div>
        </div>
      </el-col>
      <el-col :span="8">
        <div class="stat-card green" @click="router.push('/project')">
          <div class="stat-content">
            <div class="stat-value">{{ stats.projectCount }}</div>
            <div class="stat-label">项目总数</div>
          </div>
          <div class="stat-icon"><el-icon size="40"><Folder /></el-icon></div>
        </div>
      </el-col>
      <el-col :span="8">
        <div class="stat-card orange" @click="router.push('/invoice')">
          <div class="stat-content">
            <div class="stat-value">{{ stats.invoiceCount }}</div>
            <div class="stat-label">发票总数</div>
          </div>
          <div class="stat-icon"><el-icon size="40"><Document /></el-icon></div>
        </div>
      </el-col>
    </el-row>

    <!-- 项目概览 -->
    <el-card class="section-card">
      <template #header>
        <div class="card-header" style="justify-content: space-between;">
          <span class="card-title"><el-icon style="margin-right: 6px;"><Folder /></el-icon>项目概览</span>
          <el-button type="primary" link @click="goToProject">
            查看更多 <el-icon><ArrowRight /></el-icon>
          </el-button>
        </div>
      </template>
      <el-table :data="projectList" border style="width: 100%" :header-cell-style="{ padding: '8px 0' }" :cell-style="{ padding: '6px 0' }">
        <el-table-column type="index" label="序号" width="60" align="center" />
        <el-table-column prop="projectNo" label="项目编号" min-width="120" align="center" />
        <el-table-column prop="projectName" label="项目名称" min-width="150" align="center">
          <template #default="{ row }">
            <strong>{{ row.projectName }}</strong>
          </template>
        </el-table-column>
        <el-table-column prop="manager" label="负责人" width="100" align="center" />
        <el-table-column prop="budget" label="预算金额" align="center" width="150" :show-overflow-tooltip="true">
          <template #default="{ row }"><span style="white-space: nowrap">¥{{ formatMoney(row.budget) }}</span></template>
        </el-table-column>
        <el-table-column prop="status" label="状态" width="100" align="center">
          <template #default="{ row }">
            <el-tag :type="row.status === '进行中' ? 'primary' : row.status === '已完成' ? 'success' : 'info'" size="small">
              {{ row.status }}
            </el-tag>
          </template>
        </el-table-column>
      </el-table>
    </el-card>

    <!-- 最近操作记录 -->
    <el-card class="section-card" style="margin-top: 16px;">
      <template #header>
        <div class="card-header">
          <span class="card-title"><el-icon style="margin-right: 6px;"><Document /></el-icon>最近操作记录</span>
        </div>
      </template>
      <el-table :data="recentLogs" border style="width: 100%">
        <el-table-column type="index" label="序号" width="60" align="center" />
        <el-table-column prop="operator" label="操作人" width="100" align="center" />
        <el-table-column prop="action" label="操作类型" width="100" align="center">
          <template #default="{ row }">
            <el-tag :type="getActionType(row.action)" size="small">{{ row.action }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="module" label="模块" width="120" align="center" />
        <el-table-column prop="content" label="内容" min-width="200" align="center" show-overflow-tooltip />
        <el-table-column prop="time" label="时间" width="160" align="center" />
      </el-table>
    </el-card>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { OfficeBuilding, Folder, Document, ArrowRight } from '@element-plus/icons-vue'
import api from '../api/index'
import { InvoiceApi } from '../api/invoice'

const router = useRouter()

const stats = reactive({
  customerCount: 0,
  projectCount: 0,
  invoiceCount: 0
})

const projectList = ref([])
const recentLogs = ref([])

const formatMoney = (num) => {
  return Number(num || 0).toLocaleString('zh-CN', { minimumFractionDigits: 2 })
}

const getActionType = (action) => {
  const types = { '新增': 'success', '编辑': '', '删除': 'danger', '导入': 'warning', '登录': 'info' }
  return types[action] || 'info'
}

const loadData = async () => {
  try {
    // 获取客户数量
    const customers = await api.get('/suppliers')
    stats.customerCount = (customers || []).length

    // 获取项目数量和列表
    const projects = await api.get('/projects')
    stats.projectCount = (projects || []).length
    projectList.value = (projects || []).slice(0, 3) // 只显示前3个

    // 获取发票数量
    const invoices = await InvoiceApi.list()
    stats.invoiceCount = (invoices || []).length
  } catch (e) {
    console.error(e)
  }
}

// 获取真实操作记录
const loadRecentLogs = async () => {
  try {
    const logs = await api.get('/operation-logs')
    // 格式化时间并只取前5条
    recentLogs.value = (logs || []).slice(0, 5).map(log => ({
      operator: log.operator,
      action: log.action,
      module: log.module,
      content: log.content,
      time: log.operationTime ? log.operationTime.replace('T', ' ').slice(0, 16) : ''
    }))
  } catch (e) {
    console.error(e)
  }
}

const goToProject = () => {
  router.push('/project')
}

onMounted(async () => {
  await loadData()
  await loadRecentLogs()
})
</script>

<style scoped>
.stats-row {
  margin-bottom: 16px;
}

.stat-card {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 24px;
  border-radius: 8px;
  color: #fff;
  cursor: pointer;
  transition: transform 0.2s, box-shadow 0.2s;
}

.stat-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
}

.stat-card.blue {
  background: linear-gradient(135deg, #409eff 0%, #66b1ff 100%);
}

.stat-card.green {
  background: linear-gradient(135deg, #67c23a 0%, #85ce61 100%);
}

.stat-card.orange {
  background: linear-gradient(135deg, #e6a23c 0%, #ebb563 100%);
}

.stat-content {
  flex: 1;
}

.stat-value {
  font-size: 36px;
  font-weight: 700;
  margin-bottom: 4px;
}

.stat-label {
  font-size: 14px;
  opacity: 0.9;
}

.stat-icon {
  opacity: 0.8;
}

.section-card :deep(.el-card__header) {
  padding: 14px 20px;
  background: #fafafa;
  border-bottom: 1px solid #f0f0f0;
}

.card-header {
  display: flex;
  align-items: center;
}

.card-title {
  font-size: 15px;
  font-weight: 600;
  color: #303133;
  display: flex;
  align-items: center;
}

:deep(.el-table th) {
  text-align: center;
}

:deep(.el-table td .cell) {
  text-align: center;
}
</style>
