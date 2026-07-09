<template>
  <el-card>
    <template #header>
      <div class="flex-between">
        <span>客户档案</span>
        <div>
          <el-button @click="exportData">
            <el-icon><Download /></el-icon> 导出Excel
          </el-button>
          <el-button type="primary" @click="showAdd">
            <el-icon><Plus /></el-icon> 新增客户
          </el-button>
        </div>
      </div>
    </template>

    <el-table :data="tableData" border v-loading="loading" header-cell-class-name="header-center" cell-class-name="cell-center" style="width: 100%" :header-cell-style="{ padding: '8px 0' }" :cell-style="{ padding: '6px 0' }">
      <el-table-column type="index" label="序号" width="60" align="center" />
      <el-table-column prop="supplierId" label="客户编号" min-width="150" align="center" />
      <el-table-column prop="name" label="名称" min-width="200" align="center">
        <template #default="{ row }">
          <strong>{{ row.name }}</strong>
        </template>
      </el-table-column>
      <el-table-column prop="partyType" label="类型" width="90" align="center">
        <template #default="{ row }">
          <el-tag :type="row.partyType === 'supplier' ? 'primary' : 'success'" size="small">
            {{ row.partyType === 'supplier' ? '供应商' : '客户' }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column prop="taxType" label="纳税人类型" min-width="120" align="center" />
      <el-table-column prop="creditCode" label="统一社会信用代码" min-width="220" align="center" />
      <el-table-column prop="bankName" label="开户行" min-width="200" align="center" />
      <el-table-column prop="contactPerson" label="联系人" min-width="100" align="center" />
      <el-table-column prop="phone" label="电话" min-width="130" align="center" />
      <el-table-column label="操作" width="130" fixed="right" align="center">
        <template #default="{ row }">
          <div class="btn-group">
            <el-button size="small" @click="showEdit(row)">编辑</el-button>
            <el-button size="small" type="danger" @click="handleDelete(row.id)">删除</el-button>
          </div>
        </template>
      </el-table-column>
    </el-table>

    <Pagination
      :total="total"
      :current-page="currentPage"
      :page-size="pageSize"
      @update:currentPage="currentPage = $event"
    />

    <!-- 新增/编辑对话框 -->
    <el-dialog v-model="dialogVisible" :title="isEdit ? '编辑客户' : '新增客户'" width="500px">
      <el-form :model="formData" label-width="100px">
        <el-form-item label="客户编号">
          <el-input v-model="formData.supplierId" :disabled="isEdit" />
        </el-form-item>
        <el-form-item label="名称">
          <el-input v-model="formData.name" />
        </el-form-item>
        <el-form-item label="类型">
          <el-select v-model="formData.partyType">
            <el-option label="供应商（上游）" value="supplier" />
            <el-option label="客户（下游）" value="customer" />
          </el-select>
        </el-form-item>
        <el-form-item label="纳税人类型">
          <el-select v-model="formData.taxType">
            <el-option label="一般纳税人" value="一般纳税人" />
            <el-option label="小规模纳税人" value="小规模" />
          </el-select>
        </el-form-item>
        <el-form-item label="信用代码">
          <el-input v-model="formData.creditCode" />
        </el-form-item>
        <el-form-item label="开户银行">
          <el-input v-model="formData.bankName" />
        </el-form-item>
        <el-form-item label="联系人">
          <el-input v-model="formData.contactPerson" />
        </el-form-item>
        <el-form-item label="联系电话">
          <el-input v-model="formData.phone" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleSave">保存</el-button>
      </template>
    </el-dialog>
  </el-card>
</template>

<script setup>
import { ref, reactive, computed, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Plus, Download } from '@element-plus/icons-vue'
import { CustomerApi } from '../api/invoice'
import Pagination from '../components/Pagination.vue'

const loading = ref(false)
const tableData = ref([])
const dialogVisible = ref(false)
const isEdit = ref(false)
const currentPage = ref(1)
const pageSize = ref(10)
const total = computed(() => tableData.value.length)

const formData = reactive({
  id: null,
  supplierId: '',
  name: '',
  partyType: 'customer',
  taxType: '一般纳税人',
  creditCode: '',
  bankName: '',
  contactPerson: '',
  phone: ''
})

const exportData = () => {
  const headers = ['序号', '客户ID', '名称', '类型', '纳税人类型', '统一社会信用代码', '开户行', '联系人', '电话']
  const rows = tableData.value.map((row, index) => [
    index + 1,
    row.supplierId,
    row.name,
    row.partyType === 'supplier' ? '供应商' : '客户',
    row.taxType,
    row.creditCode,
    row.bankName,
    row.contactPerson,
    row.phone
  ])

  const csvContent = [headers, ...rows].map(row => row.join(',')).join('\n')
  const blob = new Blob(['\uFEFF' + csvContent], { type: 'text/csv;charset=utf-8;' })
  const link = document.createElement('a')
  link.href = URL.createObjectURL(blob)
  link.download = '客户档案_' + new Date().toISOString().slice(0, 10) + '.csv'
  link.click()
}

let loadDataTimer = null
const loadData = async () => {
  // 防止重复调用
  if (loadDataTimer) return
  loadDataTimer = setTimeout(() => { loadDataTimer = null }, 100)

  loading.value = true
  try {
    tableData.value = await CustomerApi.list()
  } catch (e) {
    console.error(e)
  }
  loading.value = false
}

const showAdd = () => {
  isEdit.value = false
  Object.assign(formData, {
    id: null,
    supplierId: 'C' + Date.now().toString(36).toUpperCase().slice(-6),
    name: '',
    partyType: 'customer',
    taxType: '一般纳税人',
    creditCode: '',
    bankName: '',
    contactPerson: '',
    phone: ''
  })
  dialogVisible.value = true
}

const showEdit = (row) => {
  isEdit.value = true
  Object.assign(formData, row)
  dialogVisible.value = true
}

const handleSave = async () => {
  if (!formData.name) {
    ElMessage.warning('请输入客户名称')
    return
  }
  try {
    if (isEdit.value) {
      await CustomerApi.update(formData.id, formData)
      ElMessage.success('更新成功')
    } else {
      await CustomerApi.create(formData)
      ElMessage.success('创建成功')
    }
    dialogVisible.value = false
    loadData()
  } catch (e) {
    ElMessage.error('保存失败')
  }
}

const handleDelete = async (id) => {
  await ElMessageBox.confirm('确认删除该客户？', '提示', { type: 'warning' })
  try {
    await CustomerApi.delete(id)
    ElMessage.success('删除成功')
    loadData()
  } catch (e) {
    ElMessage.error('删除失败')
  }
}

onMounted(loadData)
</script>

<style scoped>
.flex-between {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.btn-group {
  display: flex;
  gap: 8px;
  justify-content: center;
  white-space: nowrap;
}
:deep(.header-center .cell) {
  text-align: center;
}
:deep(.cell-center .cell) {
  text-align: center;
  white-space: nowrap;
}
</style>
