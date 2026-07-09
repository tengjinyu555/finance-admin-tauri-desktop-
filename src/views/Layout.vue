<template>
  <el-container class="layout-container">
    <el-aside width="220px" class="sidebar">
      <div class="sidebar-logo">
        <el-icon size="22" color="#409eff"><Document /></el-icon>
        <span>财税管理平台</span>
      </div>
      <el-menu
        :default-active="route.path"
        router
        background-color="#001529"
        text-color="rgba(255,255,255,0.65)"
        active-text-color="#fff"
      >
        <el-menu-item index="/dashboard">
          <el-icon><Odometer /></el-icon>
          <span>工作台</span>
        </el-menu-item>
        <el-menu-item index="/customer">
          <el-icon><OfficeBuilding /></el-icon>
          <span>客户管理</span>
        </el-menu-item>
        <el-menu-item index="/project">
          <el-icon><Folder /></el-icon>
          <span>项目管理</span>
        </el-menu-item>
        <el-menu-item index="/invoice">
          <el-icon><Document /></el-icon>
          <span>发票台账</span>
        </el-menu-item>
        <el-menu-item index="/transaction">
          <el-icon><List /></el-icon>
          <span>收支流水</span>
        </el-menu-item>
        <el-menu-item index="/ocr-import">
          <el-icon><Monitor /></el-icon>
          <span>OCR识别导入</span>
        </el-menu-item>
        <el-menu-item v-if="isAdmin" index="/employee">
          <el-icon><User /></el-icon>
          <span>员工管理</span>
        </el-menu-item>
        <el-menu-item index="/operation-log">
          <el-icon><Notebook /></el-icon>
          <span>操作日志</span>
        </el-menu-item>
      </el-menu>
    </el-aside>

    <el-container>
      <el-header class="topbar">
        <div class="topbar-title">{{ route.meta.title }}</div>
        <div class="topbar-right">
          <!-- 企业切换 -->
          <div class="tenant-wrapper">
            <el-icon class="tenant-icon"><OfficeBuilding /></el-icon>
            <el-select
              v-model="currentTenantId"
              placeholder="选择企业"
              style="width: 220px;"
              @change="handleTenantChange"
              class="tenant-select"
            >
              <el-option
                v-for="t in tenantList"
                :key="t.tenantId"
                :label="t.tenantName"
                :value="t.tenantId"
              />
            </el-select>
          </div>
          <el-tag type="primary">
            <el-icon><User /></el-icon>
            {{ (userStore.user?.roles || []).join(', ') || '未知角色' }}
          </el-tag>
          <el-tooltip content="个人信息" placement="bottom">
            <span class="user-avatar" @click="showProfile">{{ (userStore.user?.nickname || userStore.user?.username || '').charAt(0) }}</span>
          </el-tooltip>
          <el-button text @click="handleLock">
            <el-icon><Lock /></el-icon> 锁屏
          </el-button>
          <el-button text @click="handleLogout">登出</el-button>
        </div>
      </el-header>

      <el-main class="main-content">
        <router-view />
      </el-main>
    </el-container>

    <!-- 个人信息弹窗 -->
    <el-dialog v-model="profileVisible" title="个人信息" width="450px">
      <div class="profile-header">
        <div class="profile-avatar">{{ (userStore.user?.nickname || userStore.user?.username || '').charAt(0) }}</div>
        <div class="profile-info">
          <div class="profile-name">{{ userStore.user?.nickname || userStore.user?.username }}</div>
          <div class="profile-role">{{ (userStore.user?.roles || []).join(', ') || '未知角色' }}</div>
        </div>
      </div>
      <el-form :model="profileForm" label-width="80px" style="margin-top: 20px;">
        <el-form-item label="所属企业">
          <el-input :value="userStore.user?.tenantName || '默认企业'" disabled />
        </el-form-item>
        <el-form-item label="登录账户">
          <el-input :value="userStore.user?.username" disabled />
        </el-form-item>
        <el-divider>修改密码</el-divider>
        <el-form-item label="原密码">
          <el-input v-model="profileForm.oldPassword" type="password" placeholder="请输入原密码" show-password />
        </el-form-item>
        <el-form-item label="新密码">
          <el-input v-model="profileForm.newPassword" type="password" placeholder="请输入新密码" show-password />
        </el-form-item>
        <el-form-item label="确认密码">
          <el-input v-model="profileForm.confirmPassword" type="password" placeholder="请再次输入新密码" show-password />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="profileVisible = false">取消</el-button>
        <el-button type="primary" @click="changePassword">修改密码</el-button>
      </template>
    </el-dialog>

    <!-- 锁屏遮罩 -->
    <div v-if="isLocked" class="lock-overlay">
      <div class="lock-content">
        <el-icon size="64" color="#fff"><Lock /></el-icon>
        <h2>屏幕已锁定</h2>
        <p>{{ userStore.user?.nickname || userStore.user?.username }}</p>
        <el-input
          v-model="unlockPassword"
          type="password"
          placeholder="请输入密码解锁"
          show-password
          @keyup.enter="handleUnlock"
          style="width: 240px; margin-top: 20px"
        />
        <el-button type="primary" @click="handleUnlock" style="margin-top: 16px; width: 240px">
          解锁
        </el-button>
        <p v-if="unlockError" style="color: #ff6b6b; margin-top: 10px">{{ unlockError }}</p>
      </div>
    </div>
  </el-container>
</template>

<script setup>
import { ref, reactive, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useUserStore } from '../stores/user'
import { Document, Odometer, OfficeBuilding, Monitor, User, Lock, Money, List, DataAnalysis, Folder, Notebook } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'
import { AuthApi } from '../api/invoice'

const route = useRoute()
const router = useRouter()
const userStore = useUserStore()

const isLocked = ref(false)
const unlockPassword = ref('')
const unlockError = ref('')

// 企业切换相关
const currentTenantId = ref(Number(userStore.tenantId) || '')
const tenantList = ref([])

// 初始化时先把当前企业加入列表，确保下拉框能显示
if (userStore.user?.tenantId) {
  tenantList.value = [{ tenantId: Number(userStore.user.tenantId), tenantName: userStore.user.tenantName || '当前企业' }]
}

// 获取用户所属企业列表
const loadUserTenants = async () => {
  try {
    const tenants = await AuthApi.getUserTenants()
    if (tenants && tenants.length > 0) {
      tenantList.value = tenants
    }
  } catch (e) {
    console.error('获取企业列表失败:', e)
  }
}

const handleTenantChange = async (tenantId) => {
  try {
    // 调用后端接口更新最后使用时间并获取新token
    const res = await AuthApi.switchTenant(tenantId)
    // 更新本地存储
    userStore.tenantId = tenantId
    localStorage.setItem('tenantId', tenantId)
    if (res.token) {
      localStorage.setItem('token', res.token)
    }
    // 刷新页面以加载新企业的数据
    window.location.reload()
  } catch (e) {
    ElMessage.error('切换企业失败')
    console.error(e)
  }
}

// 页面加载时获取企业列表
loadUserTenants()

// 个人信息相关
const profileVisible = ref(false)
const profileForm = reactive({
  oldPassword: '',
  newPassword: '',
  confirmPassword: ''
})

const showProfile = () => {
  profileForm.oldPassword = ''
  profileForm.newPassword = ''
  profileForm.confirmPassword = ''
  profileVisible.value = true
}

const changePassword = async () => {
  if (!profileForm.oldPassword || !profileForm.newPassword) {
    ElMessage.warning('请填写完整')
    return
  }
  if (profileForm.newPassword !== profileForm.confirmPassword) {
    ElMessage.warning('两次密码不一致')
    return
  }
  if (profileForm.newPassword.length < 6) {
    ElMessage.warning('密码至少6位')
    return
  }
  try {
    await AuthApi.changePassword({
      username: userStore.user?.username,
      oldPassword: profileForm.oldPassword,
      newPassword: profileForm.newPassword
    }, localStorage.getItem('tenantId'))
    ElMessage.success('密码修改成功')
    profileVisible.value = false
  } catch (e) {
    ElMessage.error(e.response?.data?.error || '修改失败')
  }
}

const isAdmin = computed(() => {
  const roles = userStore.user?.roleCodes || userStore.user?.roles || []
  return roles.includes('admin')
})

const handleLock = () => {
  isLocked.value = true
  unlockPassword.value = ''
  unlockError.value = ''
}

const handleUnlock = async () => {
  if (!unlockPassword.value) {
    unlockError.value = '请输入密码'
    return
  }
  try {
    await AuthApi.verifyPassword({
      username: userStore.user?.username,
      password: unlockPassword.value
    }, userStore.tenantId)
    isLocked.value = false
    unlockPassword.value = ''
    unlockError.value = ''
    ElMessage.success('解锁成功')
  } catch (e) {
    unlockError.value = '密码错误'
  }
}

const handleLogout = () => {
  userStore.logout()
  router.push('/login')
}
</script>

<style scoped>
.layout-container {
  height: 100vh;
}

.sidebar {
  background: #001529;
  color: #fff;
}

.sidebar-logo {
  height: 56px;
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 0 20px;
  font-size: 16px;
  font-weight: 600;
  border-bottom: 1px solid rgba(255,255,255,0.1);
}

.topbar {
  height: 56px;
  background: #fff;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 24px;
  box-shadow: 0 1px 4px rgba(0,0,0,0.08);
}

.topbar-title {
  font-size: 16px;
  font-weight: 600;
  color: #1a1a2e;
}

.topbar-right {
  display: flex;
  align-items: center;
  gap: 16px;
}

.user-avatar {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background: #409eff;
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 14px;
  cursor: pointer;
  transition: transform 0.2s;
}

.user-avatar:hover {
  transform: scale(1.1);
  box-shadow: 0 2px 8px rgba(64, 158, 255, 0.4);
}

.profile-header {
  display: flex;
  align-items: center;
  gap: 16px;
}

.profile-avatar {
  width: 64px;
  height: 64px;
  border-radius: 50%;
  background: linear-gradient(135deg, #409eff, #66b1ff);
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 28px;
  font-weight: 600;
}

.profile-info {
  flex: 1;
}

.profile-name {
  font-size: 18px;
  font-weight: 600;
  color: #303133;
  margin-bottom: 4px;
}

.profile-role {
  font-size: 13px;
  color: #909399;
}

.main-content {
  background: #f0f2f5;
  padding: 20px;
}

:deep(.el-menu) {
  border-right: none;
}

.lock-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
}

.lock-content {
  text-align: center;
  color: #fff;
}

.lock-content h2 {
  margin: 20px 0 10px;
  font-size: 24px;
  font-weight: 600;
}

.lock-content p {
  color: rgba(255, 255, 255, 0.7);
  font-size: 14px;
}

/* 企业选择框样式优化 */
.tenant-wrapper {
  display: flex;
  align-items: center;
  background: linear-gradient(135deg, #f5f7fa 0%, #e4e7ed 100%);
  border-radius: 20px;
  padding: 0 12px 0 0;
  border: 1px solid #dcdfe6;
  transition: all 0.3s;
}

.tenant-wrapper:hover {
  border-color: #409eff;
  box-shadow: 0 2px 8px rgba(64, 158, 255, 0.2);
}

.tenant-icon {
  color: #409eff;
  margin-right: 8px;
  font-size: 16px;
}

:deep(.tenant-select .el-input__wrapper) {
  background: transparent;
  box-shadow: none !important;
  border: none;
  padding-left: 0;
}

:deep(.tenant-select .el-input__inner) {
  font-weight: 500;
  color: #303133;
}

:deep(.el-select-dropdown) {
  border-radius: 12px;
  box-shadow: 0 6px 20px rgba(0, 0, 0, 0.12);
  border: 1px solid #ebeef5;
  padding: 6px 0;
}

:deep(.el-select-dropdown__item) {
  padding: 12px 20px;
  font-size: 14px;
  color: #606266;
  transition: all 0.2s;
}

:deep(.el-select-dropdown__item.hover) {
  background: linear-gradient(135deg, #ecf5ff 0%, #d9ecff 100%);
  color: #409eff;
}

:deep(.el-select-dropdown__item.selected) {
  color: #409eff;
  font-weight: 600;
}
</style>
