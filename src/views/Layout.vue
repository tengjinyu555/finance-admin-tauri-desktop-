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
          <el-tag type="primary">
            <el-icon><User /></el-icon>
            {{ (userStore.user?.roles || []).join(', ') || '未知角色' }}
          </el-tag>
          <span class="user-avatar">{{ (userStore.user?.nickname || userStore.user?.username || '').charAt(0) }}</span>
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
import { ref, computed } from 'vue'
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
</style>
