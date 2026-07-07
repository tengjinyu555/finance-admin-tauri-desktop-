<template>
  <el-card>
    <template #header>
      <div class="flex-between">
        <span>员工管理</span>
        <el-button type="primary" @click="showDialog()">新增员工</el-button>
      </div>
    </template>

    <el-table :data="tableData" border v-loading="loading" header-cell-class-name="header-center" cell-class-name="cell-center">
      <el-table-column type="index" label="序号" width="60" />
      <el-table-column prop="username" label="用户名" width="120" />
      <el-table-column prop="nickname" label="昵称" width="120" />
      <el-table-column label="角色">
        <template #default="{ row }">
          <el-tag v-for="role in (row.roles || [])" :key="role" size="small" style="margin-right: 4px">
            {{ role }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column prop="status" label="状态" width="100">
        <template #default="{ row }">
          <el-tag :type="row.status === 'active' ? 'success' : 'danger'" size="small">
            {{ row.status === 'active' ? '启用' : '禁用' }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column prop="lastLoginAt" label="最后登录时间" width="180">
        <template #default="{ row }">
          {{ row.lastLoginAt ? formatTime(row.lastLoginAt) : '-' }}
        </template>
      </el-table-column>
      <el-table-column label="操作" width="150" fixed="right">
        <template #default="{ row }">
          <el-button size="small" :type="row.status === 'active' ? 'warning' : 'success'" @click="toggleStatus(row)" :disabled="row.id === currentUserId">
            {{ row.status === 'active' ? '禁用' : '启用' }}
          </el-button>
          <el-button size="small" type="danger" @click="handleDelete(row.id)" :disabled="row.id === currentUserId">
            删除
          </el-button>
        </template>
      </el-table-column>
    </el-table>
  </el-card>

  <!-- 新增员工对话框 -->
  <el-dialog v-model="dialogVisible" title="新增员工" width="480px" destroy-on-close>
    <el-form :model="form" :rules="rules" ref="formRef" label-width="80px">
      <el-form-item label="用户名" prop="username">
        <el-input v-model="form.username" placeholder="请输入用户名" />
      </el-form-item>
      <el-form-item label="密码" prop="password">
        <el-input v-model="form.password" type="password" placeholder="请输入密码" show-password />
      </el-form-item>
      <el-form-item label="昵称" prop="nickname">
        <el-input v-model="form.nickname" placeholder="请输入昵称" />
      </el-form-item>
      <el-form-item label="角色" prop="roleCode">
        <el-select v-model="form.roleCode" placeholder="请选择角色">
          <el-option label="管理员" value="admin" />
          <el-option label="财务录入" value="finance" />
          <el-option label="普通查看" value="viewer" />
        </el-select>
      </el-form-item>
    </el-form>
    <template #footer>
      <el-button @click="dialogVisible = false">取消</el-button>
      <el-button type="primary" @click="handleSubmit" :loading="submitting">确定</el-button>
    </template>
  </el-dialog>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { UserApi } from '../api/invoice'
import { useUserStore } from '../stores/user'

const userStore = useUserStore()
const loading = ref(false)
const tableData = ref([])

const dialogVisible = ref(false)
const submitting = ref(false)
const formRef = ref(null)
const form = ref({
  username: '',
  password: '',
  nickname: '',
  roleCode: 'finance'
})

const rules = {
  username: [{ required: true, message: '请输入用户名', trigger: 'blur' }],
  password: [{ required: true, message: '请输入密码', trigger: 'blur' }, { min: 6, message: '密码至少6位', trigger: 'blur' }],
  nickname: [{ required: true, message: '请输入昵称', trigger: 'blur' }],
  roleCode: [{ required: true, message: '请选择角色', trigger: 'change' }]
}

const currentUserId = computed(() => userStore.user?.id)

const formatTime = (time) => {
  if (!time) return '-'
  return new Date(time).toLocaleString('zh-CN')
}

let loadDataTimer = null
const loadData = async () => {
  // 防止重复调用
  if (loadDataTimer) return
  loadDataTimer = setTimeout(() => { loadDataTimer = null }, 100)

  loading.value = true
  try {
    tableData.value = await UserApi.list()
  } catch (e) {
    console.error(e)
  }
  loading.value = false
}

const showDialog = () => {
  form.value = { username: '', password: '', nickname: '', roleCode: 'finance' }
  dialogVisible.value = true
}

const handleSubmit = async () => {
  await formRef.value.validate()
  submitting.value = true
  try {
    await UserApi.create(form.value)
    ElMessage.success('新增成功')
    dialogVisible.value = false
    loadData()
  } catch (e) {
    ElMessage.error(e.response?.data?.error || '新增失败')
  }
  submitting.value = false
}

const toggleStatus = async (row) => {
  const newStatus = row.status === 'active' ? 'disabled' : 'active'
  const action = newStatus === 'disabled' ? '禁用' : '启用'

  await ElMessageBox.confirm(`确认${action}用户 ${row.username}？`, '提示', { type: 'warning' })
  try {
    await UserApi.updateStatus(row.id, newStatus)
    ElMessage.success(`${action}成功`)
    loadData()
  } catch (e) {
    ElMessage.error(`${action}失败`)
  }
}

const handleDelete = async (id) => {
  await ElMessageBox.confirm('确认删除该用户？', '提示', { type: 'warning' })
  try {
    await UserApi.delete(id)
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
:deep(.header-center .cell) { text-align: center; }
:deep(.cell-center .cell) { text-align: center; }
</style>
