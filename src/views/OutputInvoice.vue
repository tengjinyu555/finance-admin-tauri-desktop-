<template>
  <div>
    <el-row :gutter="16" class="stats-row">
      <el-col :span="4">
        <el-card shadow="never" class="stat-card">
          <div class="stat-label">销项发票总数</div>
          <div class="stat-value blue">{{ stats.totalCount }}</div>
        </el-card>
      </el-col>
      <el-col :span="5">
        <el-card shadow="never" class="stat-card">
          <div class="stat-label">销项不含税金额</div>
          <div class="stat-value blue">¥{{ formatMoney(stats.totalAmount) }}</div>
        </el-card>
      </el-col>
      <el-col :span="5">
        <el-card shadow="never" class="stat-card">
          <div class="stat-label">销项税额</div>
          <div class="stat-value orange">¥{{ formatMoney(stats.totalTax) }}</div>
        </el-card>
      </el-col>
    </el-row>

    <el-card>
      <template #header>
        <div class="flex-between">
          <span>销项发票明细台账</span>
          <div class="btn-group">
            <el-button @click="exportData">
              <el-icon><Download /></el-icon> 导出Excel
            </el-button>
            <el-button type="primary" @click="showAdd">
              <el-icon><Plus /></el-icon> 新增
            </el-button>
          </div>
        </div>
      </template>

      <div class="toolbar">
        <el-input v-model="searchKeyword" placeholder="搜索发票号/客户..." clearable style="width: 200px" @input="loadData" />
        <el-select v-model="filterStatus" placeholder="全部状态" clearable style="width: 120px" @change="loadData">
          <el-option label="正常" value="正常" />
          <el-option label="作废" value="作废" />
          <el-option label="红冲" value="红冲" />
          <el-option label="未回款" value="未回款" />
        </el-select>
      </div>

      <el-table :data="tableData" border v-loading="loading" header-cell-class-name="header-center" cell-class-name="cell-center">
        <el-table-column type="index" label="序号" width="60" />
        <el-table-column prop="code" label="发票代码" width="140" />
        <el-table-column prop="number" label="发票号码" width="200" />
        <el-table-column label="客户" width="200">
          <template #default="{ row }">
            <strong>{{ row.supplier?.name || '-' }}</strong>
          </template>
        </el-table-column>
        <el-table-column prop="date" label="开票日期" width="120" />
        <el-table-column prop="amount" label="不含税销售额" width="130">
          <template #default="{ row }">{{ row.amount ? '¥' + formatMoney(row.amount) : '-' }}</template>
        </el-table-column>
        <el-table-column prop="tax" label="销项税金" width="120">
          <template #default="{ row }">{{ row.tax ? '¥' + formatMoney(row.tax) : '-' }}</template>
        </el-table-column>
        <el-table-column prop="total" label="价税合计" width="130">
          <template #default="{ row }">{{ row.total ? '¥' + formatMoney(row.total) : '-' }}</template>
        </el-table-column>
        <el-table-column prop="rate" label="税率" width="80" />
        <el-table-column prop="status" label="状态" width="100">
          <template #default="{ row }">
            <el-tag :type="statusType(row.status)" size="small">{{ row.status || '-' }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column label="操作" width="120" fixed="right">
          <template #default="{ row }">
            <el-button size="small" @click="showEdit(row)">编辑</el-button>
            <el-button size="small" type="danger" @click="handleDelete(row.id)">删除</el-button>
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

    <!-- 新增/编辑对话框 -->
    <el-dialog v-model="dialogVisible" :title="isEdit ? '编辑销项发票' : '新增销项发票'" width="600px">
      <el-form :model="formData" label-width="100px">
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item label="发票代码">
              <el-input v-model="formData.code" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="发票号码">
              <el-input v-model="formData.number" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item label="开票日期">
              <el-date-picker v-model="formData.date" type="date" value-format="YYYY-MM-DD" style="width: 100%" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="状态">
              <el-select v-model="formData.status">
                <el-option label="正常" value="正常" />
                <el-option label="作废" value="作废" />
                <el-option label="红冲" value="红冲" />
                <el-option label="未回款" value="未回款" />
              </el-select>
            </el-form-item>
          </el-col>
        </el-row>
        <el-row :gutter="20">
          <el-col :span="8">
            <el-form-item label="不含税销售额">
              <el-input-number v-model="formData.amount" :precision="2" style="width: 100%" />
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="销项税金">
              <el-input-number v-model="formData.tax" :precision="2" style="width: 100%" />
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="税率">
              <el-select v-model="formData.rate">
                <el-option label="13%" value="13%" />
                <el-option label="9%" value="9%" />
                <el-option label="6%" value="6%" />
                <el-option label="3%" value="3%" />
              </el-select>
            </el-form-item>
          </el-col>
        </el-row>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleSave">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, reactive, computed, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Plus, Download } from '@element-plus/icons-vue'
import { OutputInvoiceApi } from '../api/invoice'
import Pagination from '../components/Pagination.vue'

const loading = ref(false)
const tableData = ref([])
const searchKeyword = ref('')
const filterStatus = ref('')
const currentPage = ref(1)
const pageSize = ref(10)
const total = ref(0)
const totalPages = computed(() => Math.ceil(total.value / pageSize.value) || 1)

const stats = reactive({
  totalCount: 0,
  totalAmount: 0,
  totalTax: 0
})

const dialogVisible = ref(false)
const isEdit = ref(false)

const formData = reactive({
  id: null,
  code: '',
  number: '',
  date: '',
  amount: 0,
  tax: 0,
  rate: '13%',
  status: '正常'
})

const formatMoney = (num) => {
  return Number(num || 0).toLocaleString('zh-CN', { minimumFractionDigits: 2 })
}

const statusType = (status) => {
  const types = { '正常': 'success', '作废': 'warning', '红冲': 'danger', '未回款': 'info' }
  return types[status] || 'info'
}

const exportData = () => {
  const headers = ['序号', '发票代码', '发票号码', '客户', '开票日期', '不含税销售额', '销项税金', '价税合计', '税率', '状态']
  const rows = tableData.value.map((row, index) => [
    index + 1,
    row.code,
    row.number,
    row.supplier?.name || '',
    row.date,
    row.amount,
    row.tax,
    row.total,
    row.rate,
    row.status
  ])

  const csvContent = [headers, ...rows].map(row => row.join(',')).join('\n')
  const blob = new Blob(['\uFEFF' + csvContent], { type: 'text/csv;charset=utf-8;' })
  const link = document.createElement('a')
  link.href = URL.createObjectURL(blob)
  link.download = '销项发票_' + new Date().toISOString().slice(0, 10) + '.csv'
  link.click()
}

let loadDataTimer = null
const loadData = async () => {
  // 防止重复调用
  if (loadDataTimer) return
  loadDataTimer = setTimeout(() => { loadDataTimer = null }, 100)

  loading.value = true
  try {
    const params = {
      search: searchKeyword.value || undefined,
      status: filterStatus.value || undefined
    }
    tableData.value = await OutputInvoiceApi.list(params)
    total.value = tableData.value.length
  } catch (e) {
    console.error(e)
  }
  loading.value = false
}

const loadStats = async () => {
  try {
    const data = await OutputInvoiceApi.stats()
    Object.assign(stats, data)
  } catch (e) {
    console.error(e)
  }
}

const showAdd = () => {
  isEdit.value = false
  Object.assign(formData, {
    id: null, code: '', number: '', date: '', amount: 0, tax: 0, rate: '13%', status: '正常'
  })
  dialogVisible.value = true
}

const showEdit = (row) => {
  isEdit.value = true
  Object.assign(formData, row)
  dialogVisible.value = true
}

const handleSave = async () => {
  try {
    if (isEdit.value) {
      await OutputInvoiceApi.update(formData.id, formData)
      ElMessage.success('更新成功')
    } else {
      await OutputInvoiceApi.create(formData)
      ElMessage.success('创建成功')
    }
    dialogVisible.value = false
    loadData()
    loadStats()
  } catch (e) {
    ElMessage.error('保存失败')
  }
}

const handleDelete = async (id) => {
  await ElMessageBox.confirm('确认删除该发票？', '提示', { type: 'warning' })
  try {
    await OutputInvoiceApi.delete(id)
    ElMessage.success('删除成功')
    loadData()
    loadStats()
  } catch (e) {
    ElMessage.error('删除失败')
  }
}

onMounted(() => {
  loadData()
  loadStats()
})
</script>

<style scoped>
.stats-row { margin-bottom: 20px; }
.stat-card { text-align: center; }
.stat-label { font-size: 12px; color: #999; }
.stat-value { font-size: 20px; font-weight: 700; color: #1a1a2e; margin-top: 4px; }
.stat-value.orange { color: #e6a23c; }
.stat-value.blue { color: #409eff; }
.flex-between { display: flex; justify-content: space-between; align-items: center; }
.toolbar { display: flex; gap: 12px; margin-bottom: 16px; }
:deep(.header-center .cell) { text-align: center; }
:deep(.cell-center .cell) { text-align: center; }
</style>
