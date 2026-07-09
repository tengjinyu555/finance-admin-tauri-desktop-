<template>
  <div>
    <el-row :gutter="16" class="stats-row">
      <el-col :span="4">
        <el-card shadow="never" class="stat-card">
          <div class="stat-label">发票总数</div>
          <div class="stat-value blue">{{ stats.totalCount }}</div>
        </el-card>
      </el-col>
      <el-col :span="5">
        <el-card shadow="never" class="stat-card">
          <div class="stat-label">进项发票</div>
          <div class="stat-value blue">{{ stats.inputCount }}</div>
        </el-card>
      </el-col>
      <el-col :span="5">
        <el-card shadow="never" class="stat-card">
          <div class="stat-label">销项发票</div>
          <div class="stat-value orange">{{ stats.outputCount }}</div>
        </el-card>
      </el-col>
      <el-col :span="5">
        <el-card shadow="never" class="stat-card">
          <div class="stat-label">进项税额</div>
          <div class="stat-value green">¥{{ formatMoney(stats.inputTax) }}</div>
        </el-card>
      </el-col>
      <el-col :span="5">
        <el-card shadow="never" class="stat-card">
          <div class="stat-label">销项税额</div>
          <div class="stat-value orange">¥{{ formatMoney(stats.outputTax) }}</div>
        </el-card>
      </el-col>
    </el-row>

    <el-card>
      <template #header>
        <div class="flex-between">
          <span>发票台账</span>
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
          <el-option label="进项" value="input" />
          <el-option label="销项" value="output" />
        </el-select>
        <el-input v-model="searchKeyword" placeholder="搜索发票号/公司名..." clearable style="width: 200px" @input="loadData" />
        <el-select v-model="filterStatus" placeholder="全部状态" clearable style="width: 120px" @change="loadData">
          <el-option label="已认证" value="已认证" />
          <el-option label="未认证" value="未认证" />
          <el-option label="正常" value="正常" />
          <el-option label="红冲" value="红冲" />
          <el-option label="作废" value="作废" />
          <el-option label="不可抵扣" value="不可抵扣" />
        </el-select>
      </div>

      <el-table :data="tableData" border v-loading="loading" style="width: 100%">
        <el-table-column type="index" label="序号" width="60" align="center" />
        <el-table-column label="类型" width="80" align="center">
          <template #default="{ row }">
            <el-tag :type="row.type === 'input' ? 'primary' : 'warning'" size="small">
              {{ row.type === 'input' ? '进项' : '销项' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="code" label="发票代码" min-width="120" align="center" />
        <el-table-column prop="number" label="发票号码" min-width="150" align="center" />
        <el-table-column label="销售方" min-width="150" align="center">
          <template #default="{ row }">
            {{ row.sellerName || row.supplier?.name || '-' }}
          </template>
        </el-table-column>
        <el-table-column label="购买方" min-width="150" align="center">
          <template #default="{ row }">
            {{ row.buyerName || '-' }}
          </template>
        </el-table-column>
        <el-table-column prop="date" label="开票日期" width="110" align="center" />
        <el-table-column prop="amount" label="不含税金额" align="right" width="120">
          <template #default="{ row }">¥{{ formatMoney(row.amount) }}</template>
        </el-table-column>
        <el-table-column prop="tax" label="税额" align="right" width="100">
          <template #default="{ row }">¥{{ formatMoney(row.tax) }}</template>
        </el-table-column>
        <el-table-column prop="total" label="价税合计" align="right" width="120">
          <template #default="{ row }">¥{{ formatMoney(row.total) }}</template>
        </el-table-column>
        <el-table-column prop="status" label="状态" width="90" align="center">
          <template #default="{ row }">
            <el-tag :type="statusType(row.status)" size="small">{{ row.status }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column label="操作" width="120" fixed="right" align="center">
          <template #default="{ row }">
            <el-dropdown trigger="click">
              <el-button size="small">
                操作 <el-icon class="el-icon--right"><ArrowDown /></el-icon>
              </el-button>
              <template #dropdown>
                <el-dropdown-menu>
                  <el-dropdown-item @click="showEdit(row)">编辑</el-dropdown-item>
                  <el-dropdown-item @click="downloadInvoice(row)">下载发票</el-dropdown-item>
                  <el-dropdown-item v-if="row.type === 'input'" @click="showRelation(row)">关联收入</el-dropdown-item>
                  <el-dropdown-item v-if="row.type === 'input'" @click="showRelated(row)">查看关联</el-dropdown-item>
                  <el-dropdown-item divided @click="handleDelete(row)" style="color: #f56c6c">删除</el-dropdown-item>
                </el-dropdown-menu>
              </template>
            </el-dropdown>
          </template>
        </el-table-column>
      </el-table>

    <!-- 关联收入发票对话框 -->
    <el-dialog v-model="relationVisible" title="关联收入发票" width="80%">
      <div v-if="currentInvoice" style="margin-bottom: 16px; padding: 12px; background: #f5f7fa; border-radius: 4px;">
        <strong>支出发票：</strong>{{ currentInvoice.number }} - {{ currentInvoice.sellerName }} - ¥{{ formatMoney(currentInvoice.total) }}
      </div>
      <el-table :data="availableOutputInvoices" border v-loading="relationLoading" @selection-change="handleSelectionChange" style="width: 100%">
        <el-table-column type="selection" width="55" />
        <el-table-column prop="number" label="发票号码" min-width="150" />
        <el-table-column prop="buyerName" label="购买方" min-width="150" />
        <el-table-column prop="date" label="开票日期" width="110" />
        <el-table-column prop="total" label="价税合计" align="right" width="120">
          <template #default="{ row }">¥{{ formatMoney(row.total) }}</template>
        </el-table-column>
      </el-table>
      <template #footer>
        <el-button @click="relationVisible = false">取消</el-button>
        <el-button type="primary" @click="saveRelation">确认关联</el-button>
      </template>
    </el-dialog>

    <!-- 查看已关联发票对话框 -->
    <el-dialog v-model="relatedVisible" title="已关联的收入发票" width="80%">
      <div v-if="currentInvoice" style="margin-bottom: 16px; padding: 12px; background: #f5f7fa; border-radius: 4px;">
        <strong>支出发票：</strong>{{ currentInvoice.number }} - {{ currentInvoice.sellerName }} - ¥{{ formatMoney(currentInvoice.total) }}
      </div>
      <el-table :data="relatedInvoices" border v-loading="relatedLoading" style="width: 100%">
        <el-table-column type="index" label="序号" width="60" />
        <el-table-column prop="number" label="发票号码" min-width="150" />
        <el-table-column prop="buyerName" label="购买方" min-width="150" />
        <el-table-column prop="date" label="开票日期" width="110" />
        <el-table-column prop="total" label="价税合计" align="right" width="120">
          <template #default="{ row }">¥{{ formatMoney(row.total) }}</template>
        </el-table-column>
        <el-table-column label="操作" width="100">
          <template #default="{ row }">
            <el-button size="small" type="danger" @click="removeRelation(row)">取消关联</el-button>
          </template>
        </el-table-column>
      </el-table>
      <div v-if="relatedInvoices.length > 0" style="margin-top: 16px; padding: 12px; background: #f0f9eb; border-radius: 4px;">
        <strong>关联汇总：</strong>已关联 {{ relatedInvoices.length }} 张发票，合计 ¥{{ formatMoney(relatedTotal) }}
      </div>
    </el-dialog>

      <Pagination
        :total="total"
        :current-page="currentPage"
        :page-size="pageSize"
        @update:currentPage="currentPage = $event"
      />
    </el-card>

    <!-- 新增/编辑对话框 -->
    <el-dialog v-model="dialogVisible" :title="dialogTitle" width="600px">
      <el-form :model="formData" label-width="100px">
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item label="发票类型">
              <el-select v-model="formData.invoiceType" :disabled="isEdit">
                <el-option label="进项" value="input" />
                <el-option label="销项" value="output" />
              </el-select>
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="开票日期">
              <el-date-picker v-model="formData.date" type="date" value-format="YYYY-MM-DD" style="width: 100%" />
            </el-form-item>
          </el-col>
        </el-row>
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
            <el-form-item :label="formData.invoiceType === 'input' ? '供应商' : '客户'">
              <el-input v-model="formData.companyName" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="状态">
              <el-select v-model="formData.status">
                <template v-if="formData.invoiceType === 'input'">
                  <el-option label="未认证" value="未认证" />
                  <el-option label="已认证" value="已认证" />
                  <el-option label="不可抵扣" value="不可抵扣" />
                </template>
                <template v-else>
                  <el-option label="正常" value="正常" />
                </template>
                <el-option label="红冲" value="红冲" />
                <el-option label="作废" value="作废" />
              </el-select>
            </el-form-item>
          </el-col>
        </el-row>
        <el-row :gutter="20">
          <el-col :span="8">
            <el-form-item label="不含税金额">
              <el-input-number v-model="formData.amount" :precision="2" style="width: 100%" />
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="税额">
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
import { Plus, Download, ArrowDown } from '@element-plus/icons-vue'
import { InvoiceApi } from '../api/invoice'
import api from '../api/index'
import Pagination from '../components/Pagination.vue'

const loading = ref(false)
const tableData = ref([])
const searchKeyword = ref('')
const filterType = ref('')
const filterStatus = ref('')
const currentPage = ref(1)
const pageSize = ref(10)
const total = ref(0)

const stats = reactive({
  totalCount: 0,
  inputCount: 0,
  outputCount: 0,
  inputTax: 0,
  outputTax: 0
})

const dialogVisible = ref(false)
const isEdit = ref(false)
const editType = ref('') // 记录编辑的是进项还是销项

const formData = reactive({
  id: null,
  invoiceType: 'input',
  code: '',
  number: '',
  date: '',
  companyName: '',
  amount: 0,
  tax: 0,
  rate: '13%',
  status: '未认证'
})

// 关联相关
const relationVisible = ref(false)
const relationLoading = ref(false)
const currentInvoice = ref(null)
const availableOutputInvoices = ref([])
const selectedInvoices = ref([])

// 查看关联相关
const relatedVisible = ref(false)
const relatedLoading = ref(false)
const relatedInvoices = ref([])
const relatedTotal = ref(0)

const dialogTitle = computed(() => {
  if (isEdit.value) {
    return editType.value === 'input' ? '编辑进项发票' : '编辑销项发票'
  }
  return formData.invoiceType === 'input' ? '新增进项发票' : '新增销项发票'
})

const formatMoney = (num) => {
  return Number(num || 0).toLocaleString('zh-CN', { minimumFractionDigits: 2 })
}

const statusType = (status) => {
  const types = { '已认证': 'success', '未认证': 'info', '正常': 'success', '红冲': 'danger', '作废': 'warning', '不可抵扣': 'info' }
  return types[status] || 'info'
}

const exportData = () => {
  const headers = ['序号', '类型', '发票代码', '发票号码', '公司名称', '开票日期', '不含税金额', '税额', '价税合计', '税率', '状态']
  const rows = tableData.value.map((row, index) => [
    index + 1,
    row.invoiceType === 'input' ? '进项' : '销项',
    row.code,
    row.number,
    row.invoiceType === 'input' ? (row.supplier?.name || '') : (row.buyerName || ''),
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
  link.download = '发票台账_' + new Date().toISOString().slice(0, 10) + '.csv'
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
      search: searchKeyword.value || undefined,
      status: filterStatus.value || undefined
    }

    const data = await InvoiceApi.list(params)
    tableData.value = (data || []).map(item => ({
      ...item,
      invoiceType: item.type
    }))
    total.value = tableData.value.length
  } catch (e) {
    console.error(e)
  }
  loading.value = false
}

const loadStats = async () => {
  try {
    const data = await InvoiceApi.stats()
    stats.totalCount = data.totalCount || 0
    stats.inputCount = data.inputCount || 0
    stats.outputCount = data.outputCount || 0
    stats.inputTax = data.inputTax || 0
    stats.outputTax = data.outputTax || 0
  } catch (e) {
    console.error(e)
  }
}

// 关联相关方法
const showRelation = async (row) => {
  currentInvoice.value = row
  relationVisible.value = true
  relationLoading.value = true

  try {
    // 获取所有销项发票
    const allInvoices = await InvoiceApi.list({ type: 'output' })
    // 获取已关联的发票
    const relations = await api.get('/invoice-relations', { params: { invoiceNo: row.number } })
    const relatedNumbers = relations.map(r => r.targetInvoiceNo)

    // 过滤掉已关联的发票
    availableOutputInvoices.value = (allInvoices || []).filter(inv =>
      !relatedNumbers.includes(inv.number)
    )
  } catch (e) {
    console.error(e)
  }
  relationLoading.value = false
}

const handleSelectionChange = (selection) => {
  selectedInvoices.value = selection
}

const saveRelation = async () => {
  if (selectedInvoices.value.length === 0) {
    ElMessage.warning('请选择要关联的收入发票')
    return
  }

  try {
    const relations = selectedInvoices.value.map(inv => ({
      sourceInvoiceNo: currentInvoice.value.number,
      targetInvoiceNo: inv.number,
      sourceAmount: currentInvoice.value.total,
      targetAmount: inv.total,
      status: 'active'
    }))

    await api.post('/invoice-relations', relations)
    ElMessage.success('关联成功')
    relationVisible.value = false
    loadData()
  } catch (e) {
    ElMessage.error('关联失败')
  }
}

// 查看已关联发票
const showRelated = async (row) => {
  currentInvoice.value = row
  relatedVisible.value = true
  relatedLoading.value = true

  try {
    // 获取已关联的发票号
    const relations = await api.get('/invoice-relations', { params: { invoiceNo: row.number } })
    const targetNumbers = relations.map(r => r.targetInvoiceNo)

    if (targetNumbers.length === 0) {
      relatedInvoices.value = []
      relatedTotal.value = 0
      relatedLoading.value = false
      return
    }

    // 获取所有销项发票，过滤出已关联的
    const allInvoices = await InvoiceApi.list({ type: 'output' })
    relatedInvoices.value = (allInvoices || []).filter(inv =>
      targetNumbers.includes(inv.number)
    )
    relatedTotal.value = relatedInvoices.value.reduce((sum, inv) => sum + Number(inv.total || 0), 0)
  } catch (e) {
    console.error(e)
  }
  relatedLoading.value = false
}

// 取消关联
const removeRelation = async (row) => {
  await ElMessageBox.confirm(`确认取消与发票 ${row.number} 的关联？`, '提示', { type: 'warning' })

  try {
    // 查找关联记录
    const relations = await api.get('/invoice-relations', { params: { invoiceNo: currentInvoice.value.number } })
    const relation = relations.find(r => r.targetInvoiceNo === row.number)

    if (relation) {
      await api.delete(`/invoice-relations/${relation.id}`)
      ElMessage.success('取消关联成功')
      // 刷新列表
      showRelated(currentInvoice.value)
    }
  } catch (e) {
    ElMessage.error('取消关联失败')
  }
}

const showAdd = () => {
  isEdit.value = false
  Object.assign(formData, {
    id: null,
    invoiceType: filterType.value || 'input',
    code: '', number: '', date: '', companyName: '',
    amount: 0, tax: 0, rate: '13%', status: '未认证'
  })
  dialogVisible.value = true
}

const downloadInvoice = (row) => {
  if (!row.imageUrl) {
    ElMessage.warning('该发票没有附件')
    return
  }
  // 根据文件类型设置扩展名
  const url = row.imageUrl.toLowerCase()
  let ext = '.jpg'
  if (url.endsWith('.pdf')) {
    ext = '.pdf'
  } else if (url.endsWith('.png')) {
    ext = '.png'
  } else if (url.endsWith('.bmp')) {
    ext = '.bmp'
  }

  // 文件名：发票号码_购方_销方
  const invoiceNo = row.number || '未知'
  const buyer = row.buyerName || '未知'
  const seller = row.sellerName || '未知'
  const fileName = invoiceNo + '_' + buyer + '_' + seller + ext

  const link = document.createElement('a')
  link.href = row.imageUrl
  link.download = fileName
  link.target = '_blank'
  document.body.appendChild(link)
  link.click()
  document.body.removeChild(link)
  ElMessage.success('开始下载')
}

const showEdit = (row) => {
  isEdit.value = true
  editType.value = row.type || row.invoiceType
  Object.assign(formData, {
    id: row.id,
    invoiceType: row.type || row.invoiceType,
    code: row.code,
    number: row.number,
    date: row.date,
    companyName: (row.type || row.invoiceType) === 'input' ? (row.supplier?.name || '') : (row.buyerName || ''),
    amount: row.amount,
    tax: row.tax,
    rate: row.rate,
    status: row.status
  })
  dialogVisible.value = true
}

const handleSave = async () => {
  try {
    const saveData = {
      ...formData,
      type: formData.invoiceType
    }
    delete saveData.invoiceType
    delete saveData.companyName

    if (isEdit.value) {
      await InvoiceApi.update(formData.id, saveData)
      ElMessage.success('更新成功')
    } else {
      await InvoiceApi.create(saveData)
      ElMessage.success('创建成功')
    }
    dialogVisible.value = false
    loadData()
    loadStats()
  } catch (e) {
    ElMessage.error('保存失败')
  }
}

const handleDelete = async (row) => {
  await ElMessageBox.confirm('确认删除该发票？', '提示', { type: 'warning' })
  try {
    await InvoiceApi.delete(row.id)
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
.stat-value.green { color: #67c23a; }
.stat-value.blue { color: #409eff; }
.stat-value.orange { color: #e6a23c; }
.flex-between { display: flex; justify-content: space-between; align-items: center; }
.toolbar { display: flex; gap: 12px; margin-bottom: 16px; }

:deep(.el-table th) {
  text-align: center;
}
</style>
