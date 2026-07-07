<template>
  <div>
    <el-row :gutter="16" class="stats-row">
      <el-col :span="6">
        <el-card shadow="never" class="stat-card">
          <div class="stat-label">收入总额</div>
          <div class="stat-value green">¥{{ formatMoney(stats.totalIncome) }}</div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="never" class="stat-card">
          <div class="stat-label">支出总额</div>
          <div class="stat-value orange">¥{{ formatMoney(stats.totalPayment) }}</div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="never" class="stat-card">
          <div class="stat-label">收支差额</div>
          <div class="stat-value" :class="stats.balance >= 0 ? 'green' : 'red'">
            ¥{{ formatMoney(stats.balance) }}
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="never" class="stat-card">
          <div class="stat-label">流水笔数</div>
          <div class="stat-value blue">{{ stats.totalCount }}</div>
        </el-card>
      </el-col>
    </el-row>

    <el-card>
      <template #header>
        <div class="flex-between">
          <span>收支流水</span>
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
        <el-select v-model="filterType" placeholder="全部类型" clearable style="width: 120px" @change="loadData">
          <el-option label="收入" value="income" />
          <el-option label="支出" value="payment" />
        </el-select>
        <el-input v-model="searchKeyword" placeholder="搜索客户/发票号/流水号..." clearable style="width: 250px" @input="loadData" />
      </div>

      <el-table :data="tableData" border v-loading="loading" header-cell-class-name="header-center" cell-class-name="cell-center">
        <el-table-column type="index" label="序号" width="60" />
        <el-table-column prop="transactionNo" label="流水号" width="120" />
        <el-table-column label="类型" width="80">
          <template #default="{ row }">
            <el-tag :type="row.type === 'income' ? 'success' : 'warning'" size="small">
              {{ row.type === 'income' ? '收入' : '支出' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="buyerName" label="购买方" width="150">
          <template #default="{ row }">
            <strong>{{ row.buyerName }}</strong>
          </template>
        </el-table-column>
        <el-table-column prop="sellerName" label="销售方" width="150">
          <template #default="{ row }">
            <strong>{{ row.sellerName }}</strong>
          </template>
        </el-table-column>
        <el-table-column prop="invoiceNo" label="关联发票号" width="150" />
        <el-table-column prop="amount" label="金额" align="right" width="120">
          <template #default="{ row }">
            <span :class="row.type === 'income' ? 'text-green' : 'text-orange'">
              ¥{{ formatMoney(row.amount) }}
            </span>
          </template>
        </el-table-column>
        <el-table-column prop="transactionDate" label="日期" width="110" />
        <el-table-column prop="status" label="状态" width="90">
          <template #default="{ row }">
            <el-tag :type="statusType(row.status)" size="small">{{ row.status }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="remark" label="备注" min-width="150" show-overflow-tooltip />
        <el-table-column label="操作" width="120" align="center">
          <template #default="{ row }">
            <el-dropdown trigger="click">
              <el-button size="small">
                操作 <el-icon class="el-icon--right"><ArrowDown /></el-icon>
              </el-button>
              <template #dropdown>
                <el-dropdown-menu>
                  <el-dropdown-item @click="showEdit(row)">编辑</el-dropdown-item>
                  <el-dropdown-item divided @click="handleDelete(row.id)" style="color: #f56c6c">删除</el-dropdown-item>
                </el-dropdown-menu>
              </template>
            </el-dropdown>
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
    <el-dialog v-model="dialogVisible" :title="isEdit ? '编辑流水' : '新增流水'" width="600px">
      <el-form :model="formData" label-width="100px">
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item label="类型">
              <el-select v-model="formData.type">
                <el-option label="收入" value="income" />
                <el-option label="支出" value="payment" />
              </el-select>
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="日期">
              <el-date-picker v-model="formData.transactionDate" type="date" value-format="YYYY-MM-DD" style="width: 100%" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item label="购买方">
              <el-input v-model="formData.buyerName" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="销售方">
              <el-input v-model="formData.sellerName" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item label="金额">
              <el-input-number v-model="formData.amount" :precision="2" :min="0" style="width: 100%" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="状态">
              <el-select v-model="formData.status">
                <el-option label="已完成" value="已完成" />
                <el-option label="待确认" value="待确认" />
                <el-option label="已取消" value="已取消" />
              </el-select>
            </el-form-item>
          </el-col>
        </el-row>
        <el-form-item label="备注">
          <el-input v-model="formData.remark" type="textarea" :rows="2" />
        </el-form-item>
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
import { Plus, Download, ArrowDown } from '@element-plus/icons-vue'
import api from '../api/index'
import Pagination from '../components/Pagination.vue'

const loading = ref(false)
const tableData = ref([])
const searchKeyword = ref('')
const filterType = ref('')
const currentPage = ref(1)
const pageSize = ref(10)
const total = ref(0)

const stats = reactive({
  totalIncome: 0,
  totalPayment: 0,
  balance: 0,
  totalCount: 0
})

const dialogVisible = ref(false)
const isEdit = ref(false)

const formData = reactive({
  id: null,
  transactionNo: '',
  type: 'income',
  amount: 0,
  transactionDate: '',
  buyerName: '',
  sellerName: '',
  invoiceNo: '',
  status: '已完成',
  remark: ''
})

const formatMoney = (num) => {
  return Number(num || 0).toLocaleString('zh-CN', { minimumFractionDigits: 2 })
}

const statusType = (status) => {
  const types = { '已完成': 'success', '待确认': 'warning', '已取消': 'danger' }
  return types[status] || 'info'
}

const exportData = () => {
  const headers = ['序号', '流水号', '类型', '购买方', '销售方', '关联发票号', '金额', '日期', '状态', '备注']
  const rows = tableData.value.map((row, index) => [
    index + 1,
    row.transactionNo,
    row.type === 'income' ? '收入' : '支出',
    row.buyerName,
    row.sellerName,
    row.invoiceNo,
    row.amount || 0,
    row.transactionDate,
    row.status,
    row.remark
  ])

  const csvContent = [headers, ...rows].map(row => row.join(',')).join('\n')
  const blob = new Blob(['\uFEFF' + csvContent], { type: 'text/csv;charset=utf-8;' })
  const link = document.createElement('a')
  link.href = URL.createObjectURL(blob)
  link.download = '收支流水_' + new Date().toISOString().slice(0, 10) + '.csv'
  link.click()
}

let loadDataTimer = null
const loadData = async () => {
  if (loadDataTimer) return
  loadDataTimer = setTimeout(() => { loadDataTimer = null }, 100)

  loading.value = true
  try {
    const params = {
      type: filterType.value || undefined,
      search: searchKeyword.value || undefined
    }
    tableData.value = await api.get('/finance-transactions', { params })
    total.value = tableData.value.length
  } catch (e) {
    console.error(e)
  }
  loading.value = false
}

const loadStats = async () => {
  try {
    const data = await api.get('/finance-transactions/stats')
    Object.assign(stats, data)
  } catch (e) {
    console.error(e)
  }
}

const showAdd = () => {
  isEdit.value = false
  Object.assign(formData, {
    id: null,
    transactionNo: 'TX' + Date.now().toString(36).toUpperCase().slice(-6),
    type: 'income',
    amount: 0,
    transactionDate: '',
    customerName: '',
    invoiceNo: '',
    status: '已完成',
    remark: ''
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
      await api.put(`/finance-transactions/${formData.id}`, formData)
      ElMessage.success('更新成功')
    } else {
      await api.post('/finance-transactions', formData)
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
  await ElMessageBox.confirm('确认删除该流水？', '提示', { type: 'warning' })
  try {
    await api.delete(`/finance-transactions/${id}`)
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
.stat-value { font-size: 18px; font-weight: 700; color: #1a1a2e; margin-top: 4px; }
.stat-value.green { color: #67c23a; }
.stat-value.blue { color: #409eff; }
.stat-value.orange { color: #e6a23c; }
.stat-value.red { color: #f56c6c; }
.text-green { color: #67c23a; }
.text-orange { color: #e6a23c; }
.flex-between { display: flex; justify-content: space-between; align-items: center; }
.toolbar { display: flex; gap: 12px; margin-bottom: 16px; }
</style>
