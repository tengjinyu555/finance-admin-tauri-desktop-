<template>
  <div>
    <el-row :gutter="16" class="stats-row">
      <el-col :span="6">
        <el-card shadow="never" class="stat-card">
          <div class="stat-label">供应商总数</div>
          <div class="stat-value blue">{{ tableData.length }}</div>
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
          <div class="stat-label">收入总额</div>
          <div class="stat-value green">¥{{ formatMoney(stats.totalIncome) }}</div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="never" class="stat-card">
          <div class="stat-label">净额</div>
          <div class="stat-value" :class="stats.balance >= 0 ? 'green' : 'red'">
            ¥{{ formatMoney(stats.balance) }}
          </div>
        </el-card>
      </el-col>
    </el-row>

    <el-card>
      <template #header>
        <div class="flex-between">
          <span>供应商收支统计</span>
          <el-button @click="exportData">
            <el-icon><Download /></el-icon> 导出Excel
          </el-button>
        </div>
      </template>

      <div class="toolbar">
        <el-input v-model="searchKeyword" placeholder="搜索供应商..." clearable style="width: 250px" @input="loadData" />
      </div>

      <el-table :data="filteredData" border v-loading="loading" style="width: 100%" show-summary :summary-method="getSummary">
        <el-table-column type="index" label="序号" width="60" align="center" />
        <el-table-column prop="name" label="供应商" min-width="200" align="center">
          <template #default="{ row }">
            <strong>{{ row.name }}</strong>
          </template>
        </el-table-column>
        <el-table-column prop="paymentCount" label="支出发票数" align="center" width="100" />
        <el-table-column prop="totalPayment" label="支出金额" align="right" width="140">
          <template #default="{ row }">
            <span v-if="row.totalPayment > 0" class="text-orange">¥{{ formatMoney(row.totalPayment) }}</span>
            <span v-else>-</span>
          </template>
        </el-table-column>
        <el-table-column prop="incomeCount" label="关联回款数" align="center" width="100" />
        <el-table-column prop="totalIncome" label="回款金额" align="right" width="140">
          <template #default="{ row }">
            <span v-if="row.totalIncome > 0" class="text-green">¥{{ formatMoney(row.totalIncome) }}</span>
            <span v-else>-</span>
          </template>
        </el-table-column>
        <el-table-column label="下余" align="right" width="140">
          <template #default="{ row }">
            <span :class="(row.totalPayment - row.totalIncome) > 0 ? 'text-red' : 'text-green'">
              ¥{{ formatMoney(row.totalPayment - row.totalIncome) }}
            </span>
          </template>
        </el-table-column>
        <el-table-column label="操作" width="120" align="center">
          <template #default="{ row }">
            <el-button size="small" @click="showDetail(row)">往来明细</el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>

    <!-- 往来明细对话框 -->
    <el-dialog v-model="detailVisible" :title="currentSupplier + ' - 往来明细'" width="900px">
      <el-tabs v-model="detailTab">
        <el-tab-pane label="支出发票" name="payment">

          <el-table :data="paymentInvoices" border style="width: 100%">
            <el-table-column type="index" label="序号" width="60" align="center" />
            <el-table-column prop="number" label="发票号码" min-width="150" align="center" />
            <el-table-column prop="sellerName" label="销售方" min-width="150" align="center" />
            <el-table-column prop="date" label="开票日期" width="110" align="center" />
            <el-table-column prop="total" label="金额" align="right" width="120">
              <template #default="{ row }">¥{{ formatMoney(row.total) }}</template>
            </el-table-column>
            <el-table-column label="关联回款" width="100" align="center">
              <template #default="{ row }">
                <el-tag size="small" type="success">{{ getRelatedCount(row.number) }}张</el-tag>
              </template>
            </el-table-column>
          </el-table>
        </el-tab-pane>
        <el-tab-pane label="关联回款" name="income">
          <el-table :data="relatedIncomes" border style="width: 100%">
            <el-table-column type="index" label="序号" width="60" align="center" />
            <el-table-column prop="number" label="发票号码" min-width="150" align="center" />
            <el-table-column prop="buyerName" label="购买方" min-width="150" align="center" />
            <el-table-column prop="date" label="开票日期" width="110" align="center" />
            <el-table-column prop="total" label="金额" align="right" width="120">
              <template #default="{ row }">¥{{ formatMoney(row.total) }}</template>
            </el-table-column>
            <el-table-column label="关联支出" width="120" align="center">
              <template #default="{ row }">
                <el-tag size="small">{{ getRelatedSource(row.number) }}</el-tag>
              </template>
            </el-table-column>
          </el-table>
        </el-tab-pane>
      </el-tabs>
      <div class="detail-summary">
        <span>支出合计：<strong class="text-orange">¥{{ formatMoney(detailStats.totalPayment) }}</strong></span>
        <span>收入合计：<strong class="text-green">¥{{ formatMoney(detailStats.totalIncome) }}</strong></span>
        <span>下余：<strong :class="detailStats.balance > 0 ? 'text-red' : 'text-green'">¥{{ formatMoney(detailStats.balance) }}</strong></span>
      </div>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, reactive, computed, onMounted } from 'vue'
import { Download } from '@element-plus/icons-vue'
import api from '../api/index'
import { InvoiceApi } from '../api/invoice'

const loading = ref(false)
const tableData = ref([])
const searchKeyword = ref('')

const stats = reactive({
  totalPayment: 0,
  totalIncome: 0,
  balance: 0
})

// 明细相关
const detailVisible = ref(false)
const detailTab = ref('payment')
const currentSupplier = ref('')
const paymentInvoices = ref([])
const relatedIncomes = ref([])
const allRelations = ref([])
const detailStats = reactive({
  totalPayment: 0,
  totalIncome: 0,
  balance: 0
})

const filteredData = computed(() => {
  if (!searchKeyword.value) return tableData.value
  const keyword = searchKeyword.value.toLowerCase()
  return tableData.value.filter(r => r.name.toLowerCase().includes(keyword))
})

const formatMoney = (num) => {
  return Number(num || 0).toLocaleString('zh-CN', { minimumFractionDigits: 2 })
}

const exportData = () => {
  const headers = ['序号', '供应商', '支出发票数', '支出金额', '关联回款数', '回款金额', '下余']
  const rows = filteredData.value.map((row, index) => [
    index + 1,
    row.name,
    row.paymentCount,
    row.totalPayment || 0,
    row.incomeCount,
    row.totalIncome || 0,
    row.totalPayment - row.totalIncome
  ])

  const csvContent = [headers, ...rows].map(row => row.join(',')).join('\n')
  const blob = new Blob(['\uFEFF' + csvContent], { type: 'text/csv;charset=utf-8;' })
  const link = document.createElement('a')
  link.href = URL.createObjectURL(blob)
  link.download = '供应商收支统计_' + new Date().toISOString().slice(0, 10) + '.csv'
  link.click()
}

let loadDataTimer = null
const loadData = async () => {
  if (loadDataTimer) return
  loadDataTimer = setTimeout(() => { loadDataTimer = null }, 100)

  loading.value = true
  try {
    // 获取供应商统计数据
    const data = await api.get('/supplier-stats')
    tableData.value = data || []

    // 更新统计数据
    stats.totalPayment = tableData.value.reduce((sum, r) => sum + Number(r.totalPayment || 0), 0)
    stats.totalIncome = tableData.value.reduce((sum, r) => sum + Number(r.totalIncome || 0), 0)
    stats.balance = stats.totalIncome - stats.totalPayment
  } catch (e) {
    console.error(e)
  }
  loading.value = false
}

// 明细相关方法
const showDetail = async (row) => {
  currentSupplier.value = row.name
  detailVisible.value = true
  detailTab.value = 'payment'

  try {
    // 获取所有发票
    const allInvoices = await InvoiceApi.list()

    // 筛选该供应商的支出发票
    paymentInvoices.value = (allInvoices || []).filter(inv =>
      inv.type === 'input' && inv.sellerName === row.name
    )

    // 获取所有关联关系
    allRelations.value = await api.get('/invoice-relations')

    // 筛选与支出发票关联的收入发票
    const paymentNumbers = paymentInvoices.value.map(inv => inv.number)
    const relatedNumbers = allRelations.value
      .filter(r => paymentNumbers.includes(r.sourceInvoiceNo))
      .map(r => r.targetInvoiceNo)

    relatedIncomes.value = (allInvoices || []).filter(inv =>
      relatedNumbers.includes(inv.number)
    )

    // 统计
    detailStats.totalPayment = paymentInvoices.value.reduce((sum, inv) => sum + Number(inv.total || 0), 0)
    detailStats.totalIncome = relatedIncomes.value.reduce((sum, inv) => sum + Number(inv.total || 0), 0)
    detailStats.balance = detailStats.totalPayment - detailStats.totalIncome
  } catch (e) {
    console.error(e)
  }
}

const getRelatedCount = (invoiceNo) => {
  return allRelations.value.filter(r => r.sourceInvoiceNo === invoiceNo).length
}

const getRelatedSource = (invoiceNo) => {
  const relation = allRelations.value.find(r => r.targetInvoiceNo === invoiceNo)
  return relation ? relation.sourceInvoiceNo : '-'
}

const getSummary = ({ columns, data }) => {
  const sums = []
  columns.forEach((column, index) => {
    if (index === 0) {
      sums[index] = '合计'
      return
    }
    if (index === 1) {
      sums[index] = ''
      return
    }

    // 下余列特殊处理（倒数第二列）
    if (index === columns.length - 2) {
      const totalPayment = data.reduce((sum, item) => sum + Number(item.totalPayment || 0), 0)
      const totalIncome = data.reduce((sum, item) => sum + Number(item.totalIncome || 0), 0)
      const balance = totalPayment - totalIncome
      sums[index] = '¥' + formatMoney(balance)
      return
    }

    // 操作列
    if (index === columns.length - 1) {
      sums[index] = ''
      return
    }

    const values = data.map(item => Number(item[column.property] || 0))
    if (!values.every(value => isNaN(value))) {
      const sum = values.reduce((prev, curr) => {
        const value = Number(curr)
        if (!isNaN(value)) {
          return prev + curr
        } else {
          return prev
        }
      }, 0)
      sums[index] = '¥' + formatMoney(sum)
    } else {
      sums[index] = ''
    }
  })
  return sums
}

onMounted(loadData)
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
.text-red { color: #f56c6c; }
.flex-between { display: flex; justify-content: space-between; align-items: center; }
.toolbar { display: flex; gap: 12px; margin-bottom: 16px; }
.detail-summary {
  margin-top: 16px;
  padding: 12px 16px;
  background: #f5f7fa;
  border-radius: 4px;
  display: flex;
  gap: 32px;
  font-size: 14px;
}
.detail-summary strong {
  font-size: 16px;
}

:deep(.el-table th) {
  text-align: center;
}
</style>
