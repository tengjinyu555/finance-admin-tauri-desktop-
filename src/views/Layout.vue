<template>
  <el-container class="layout-container">
    <el-aside width="220px" class="sidebar">
      <div class="sidebar-logo">
        <el-icon size="22" color="#409eff"><Document /></el-icon>
        <span>业财一体化管理</span>
      </div>
      <el-menu
        :default-active="route.path"
        router
        background-color="#001529"
        text-color="rgba(255,255,255,0.65)"
        active-text-color="#fff"
        class="sidebar-menu"
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
      <!-- 底部企业切换 -->
      <div class="sidebar-footer">
        <el-popover placement="right" trigger="click" :width="'auto'">
          <template #reference>
            <div class="tenant-trigger">
              <el-icon><OfficeBuilding /></el-icon>
              <span class="tenant-label">我的主体</span>
            </div>
          </template>
          <div class="tenant-list">
            <div
              v-for="t in tenantList"
              :key="t.tenantId"
              class="tenant-item"
              :class="{ active: currentTenantId === t.tenantId }"
              @click="handleTenantChange(t.tenantId)"
            >
              <span>{{ t.tenantName }}</span>
              <el-icon v-if="currentTenantId === t.tenantId" class="check-icon"><Check /></el-icon>
            </div>
          </div>
        </el-popover>
      </div>
    </el-aside>

    <el-container>
      <el-header class="topbar">
        <div class="topbar-title">{{ route.meta.title }}</div>
        <!-- 全局搜索 -->
        <div class="header-search">
          <div class="search-box" :class="{ focused: searchFocused }">
            <el-icon><Search /></el-icon>
            <input
              v-model="searchKeyword"
              type="text"
              placeholder="搜索客户、项目、发票..."
              @focus="searchFocused = true"
              @blur="onSearchBlur"
              @input="onSearchInput"
              @keydown.enter="onSearchEnter"
              @keydown.esc="onSearchEsc"
            />
            <span class="search-shortcut">⌘K</span>
          </div>
          <div v-show="searchFocused && !searchKeyword" class="search-tips">
            <div v-for="m in getAvailableSearchModules()" :key="m.path" class="search-tip-item" @mousedown.prevent="goTo(m.path)">
              <el-icon><component :is="m.icon" /></el-icon> 转到 <strong>{{ m.label }}</strong>
            </div>
          </div>
          <!-- 搜索结果 -->
          <div v-show="searchFocused && searchKeyword && searchResults.length > 0" class="search-tips">
            <div
              v-for="item in searchResults"
              :key="item.path"
              class="search-tip-item"
              @mousedown.prevent="goTo(item.path)"
            >
              <el-icon><component :is="item.icon" /></el-icon>
              {{ item.label }}
            </div>
          </div>
        </div>
        <div class="topbar-right">
          <!-- 用户下拉菜单 -->
          <el-dropdown trigger="click">
            <div class="user-info">
              <span class="user-avatar">{{ (userStore.user?.nickname || userStore.user?.username || '').charAt(0) }}</span>
              <span class="user-name">{{ userStore.user?.nickname || userStore.user?.username }}</span>
              <el-icon class="el-icon--right"><ArrowDown /></el-icon>
            </div>
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item disabled>
                  <el-tag size="small" type="info">{{ (userStore.user?.roles || []).join(', ') || '未知角色' }}</el-tag>
                </el-dropdown-item>
                <el-dropdown-item @click="showProfile">
                  <el-icon><User /></el-icon> 个人信息
                </el-dropdown-item>
                <el-dropdown-item @click="handleLock">
                  <el-icon><Lock /></el-icon> 锁屏
                </el-dropdown-item>
                <el-dropdown-item divided @click="handleLogout">
                  <el-icon><SwitchButton /></el-icon> 登出
                </el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
        </div>
      </el-header>

      <el-main class="main-content">
        <!-- 更新提示 -->
        <div v-if="updateState.show" class="update-overlay">
          <div class="update-dialog">
            <div class="update-icon" v-if="updateState.status === 'downloading'">
              <el-icon size="48" color="#409eff"><Loading /></el-icon>
            </div>
            <div class="update-icon" v-else-if="updateState.status === 'complete'">
              <el-icon size="48" color="#67c23a"><CircleCheck /></el-icon>
            </div>
            <div class="update-icon" v-else-if="updateState.status === 'error'">
              <el-icon size="48" color="#f56c6c"><CircleClose /></el-icon>
            </div>
            <div v-else class="update-icon">
              <el-icon size="48" color="#e6a23c"><Download /></el-icon>
            </div>

            <h3 v-if="updateState.status === 'available'" style="margin: 12px 0 8px;">发现新版本 {{ updateState.version }}</h3>
            <h3 v-else-if="updateState.status === 'downloading'" style="margin: 12px 0 8px;">正在下载更新...</h3>
            <h3 v-else-if="updateState.status === 'complete'" style="margin: 12px 0 8px;">更新下载完成</h3>
            <h3 v-else-if="updateState.status === 'error'" style="margin: 12px 0 8px;color:#f56c6c;">更新失败</h3>

            <p v-if="updateState.status === 'available'" style="color:#999;font-size:13px;">是否立即更新？</p>
            <p v-else-if="updateState.status === 'downloading'" style="color:#999;font-size:13px;">
              {{ updateState.loaded > 0 ? ((updateState.loaded / updateState.total * 100).toFixed(1) + '%') : '准备中...' }}
            </p>
            <p v-else-if="updateState.status === 'complete'" style="color:#999;font-size:13px;">点击「立即重启」完成更新</p>
            <p v-else-if="updateState.status === 'error'" style="color:#f56c6c;font-size:13px;">{{ updateState.errorMsg }}</p>

            <el-progress
              v-if="updateState.status === 'downloading' && updateState.total > 0"
              :percentage="Math.round(updateState.loaded / updateState.total * 100)"
              :stroke-width="8"
              style="margin: 12px 0;"
            />

            <div class="update-btns">
              <el-button v-if="updateState.status === 'available'" @click="updateState.show = false">稍后更新</el-button>
              <el-button v-if="updateState.status === 'available'" type="primary" @click="startUpdateDownload">立即更新</el-button>
              <el-button v-if="updateState.status === 'downloading'" disabled>下载中...</el-button>
              <el-button v-if="updateState.status === 'complete'" type="primary" @click="restartApp">立即重启</el-button>
              <el-button v-if="updateState.status === 'error'" type="primary" @click="updateState.show = false">关闭</el-button>
            </div>
          </div>
        </div>
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
        <el-form-item label="昵称">
          <div class="email-row">
            <el-input v-model="profileForm.nickname" :placeholder="userStore.user?.nickname || '未设置'" :disabled="!nicknameEditing" />
            <el-button v-if="!nicknameEditing" type="primary" size="small" @click="startEditNickname">修改</el-button>
            <el-button v-else type="success" size="small" @click="saveNickname">保存</el-button>
          </div>
        </el-form-item>
        <el-form-item label="所属企业">
          <el-input :value="userStore.user?.tenantName || '默认企业'" disabled />
        </el-form-item>
        <el-form-item label="登录账户">
          <el-input :value="userStore.user?.username" disabled />
        </el-form-item>
        <el-form-item label="邮箱">
          <div class="email-row">
            <el-input v-model="profileForm.email" :placeholder="userStore.user?.email || '未设置'" :disabled="!emailEditing" />
            <el-button v-if="!emailEditing" type="primary" size="small" @click="startEditEmail">
              {{ userStore.user?.email ? '换绑' : '绑定' }}
            </el-button>
            <el-button v-else type="success" size="small" @click="bindEmail">确认绑定</el-button>
          </div>
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
      <div class="lock-card">
        <div class="lock-avatar">
          <el-icon size="40" color="#fff"><Lock /></el-icon>
        </div>
        <h2 class="lock-title">屏幕已锁定</h2>
        <p class="lock-user">{{ userStore.user?.nickname || userStore.user?.username }}</p>
        <el-input
          v-model="unlockPassword"
          type="password"
          placeholder="请输入密码解锁"
          show-password
          @keyup.enter="handleUnlock"
          class="lock-input"
        />
        <el-button type="primary" @click="handleUnlock" class="lock-btn">解锁</el-button>
        <p v-if="unlockError" class="lock-error">{{ unlockError }}</p>
      </div>
    </div>
  </el-container>
</template>

<script setup>
import { ref, reactive, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useUserStore } from '../stores/user'
import { Document, Odometer, OfficeBuilding, Monitor, User, Lock, Money, List, DataAnalysis, Folder, Notebook, Bell, ArrowDown, SwitchButton, ArrowRight, Check, Search, Loading, CircleCheck, CircleClose, Download } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'
import { AuthApi } from '../api/invoice'
import api from '../api/index'

const route = useRoute()
const router = useRouter()
const userStore = useUserStore()

// 自动更新
import { listen } from '@tauri-apps/api/event'
import { emit } from '@tauri-apps/api/event'

const updateState = reactive({
  show: false,
  status: 'available', // available, downloading, complete, error
  version: '',
  loaded: 0,
  total: 0,
  errorMsg: ''
})

const startUpdateDownload = () => {
  updateState.status = 'downloading'
  updateState.loaded = 0
  updateState.total = 0
}

const restartApp = () => {
  emit('update-restart')
}

// 监听 Tauri 后端事件
listen('update-available', (event) => {
  updateState.version = event.payload
  updateState.status = 'available'
  updateState.show = true
})

listen('update-progress', (event) => {
  updateState.loaded = event.payload.loaded
  updateState.total = event.payload.total
})

listen('update-complete', () => {
  updateState.status = 'complete'
})

listen('update-error', (event) => {
  updateState.status = 'error'
  updateState.errorMsg = event.payload
})

const isLocked = ref(false)
const unlockPassword = ref('')
const unlockError = ref('')

// 全局搜索
const searchKeyword = ref('')
const searchFocused = ref(false)
const searchResults = ref([])

const searchModules = [
  { label: '客户管理', path: '/customer', icon: 'OfficeBuilding', keywords: ['客户', '供应商', 'customer'], roles: ['admin', 'finance', 'viewer'] },
  { label: '项目管理', path: '/project', icon: 'Folder', keywords: ['项目', 'project'], roles: ['admin', 'finance', 'viewer'] },
  { label: '发票台账', path: '/invoice', icon: 'Document', keywords: ['发票', '进项', '销项', 'invoice'], roles: ['admin', 'finance', 'viewer'] },
  { label: '收支流水', path: '/transaction', icon: 'List', keywords: ['流水', '收支', 'transaction'], roles: ['admin', 'finance', 'viewer'] },
  { label: 'OCR识别导入', path: '/ocr-import', icon: 'Monitor', keywords: ['ocr', '识别', '导入'], roles: ['admin', 'finance'] },
  { label: '员工管理', path: '/employee', icon: 'User', keywords: ['员工', '用户', 'employee'], roles: ['admin'] },
  { label: '操作日志', path: '/operation-log', icon: 'Notebook', keywords: ['日志', '操作', 'log'], roles: ['admin', 'finance', 'viewer'] }
]

const getAvailableSearchModules = () => {
  const userRoles = userStore.user?.roleCodes || userStore.user?.roles || []
  return searchModules.filter(m => m.roles.some(r => userRoles.includes(r)))
}

const onSearchInput = () => {
  if (!searchKeyword.value.trim()) {
    searchResults.value = []
    return
  }
  const kw = searchKeyword.value.toLowerCase()
  const available = getAvailableSearchModules()
  searchResults.value = available.filter(m =>
    m.label.toLowerCase().includes(kw) ||
    m.keywords.some(k => k.includes(kw))
  )
}

const onSearchBlur = () => {
  setTimeout(() => {
    searchFocused.value = false
  }, 200)
}

const onSearchEsc = () => {
  searchKeyword.value = ''
  searchFocused.value = false
}

const onSearchEnter = () => {
  if (searchResults.value.length > 0) {
    goTo(searchResults.value[0].path)
  }
}

const goTo = (path) => {
  searchKeyword.value = ''
  searchFocused.value = false
  router.push(path)
}

// 键盘快捷键
document.addEventListener('keydown', (e) => {
  if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
    e.preventDefault()
    searchFocused.value = true
  }
})

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
  nickname: '',
  email: '',
  oldPassword: '',
  newPassword: '',
  confirmPassword: ''
})
const nicknameEditing = ref(false)
const emailEditing = ref(false)

const showProfile = () => {
  profileForm.nickname = ''
  profileForm.email = ''
  profileForm.oldPassword = ''
  profileForm.newPassword = ''
  profileForm.confirmPassword = ''
  nicknameEditing.value = false
  emailEditing.value = false
  profileVisible.value = true
}

const startEditNickname = () => {
  profileForm.nickname = userStore.user?.nickname || ''
  nicknameEditing.value = true
}

const saveNickname = async () => {
  if (!profileForm.nickname) {
    ElMessage.warning('请输入昵称')
    return
  }
  try {
    await api.put('/users/update-nickname', {
      nickname: profileForm.nickname
    })
    ElMessage.success('昵称修改成功')
    const user = JSON.parse(localStorage.getItem('user') || '{}')
    user.nickname = profileForm.nickname
    localStorage.setItem('user', JSON.stringify(user))
    userStore.user.nickname = profileForm.nickname
    nicknameEditing.value = false
  } catch (e) {
    ElMessage.error(e.response?.data?.error || '修改失败')
  }
}

const startEditEmail = () => {
  profileForm.email = userStore.user?.email || ''
  emailEditing.value = true
}

const bindEmail = async () => {
  console.log('bindEmail called, email:', profileForm.email)
  if (!profileForm.email) {
    ElMessage.warning('请输入邮箱地址')
    return
  }
  // 简单的邮箱格式验证
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
  if (!emailRegex.test(profileForm.email)) {
    ElMessage.warning('请输入正确的邮箱格式')
    return
  }
  try {
    console.log('Sending request...')
    await api.put('/users/bind-email', {
      email: profileForm.email
    })
    console.log('Request success')
    ElMessage.success('邮箱绑定成功')
    // 更新本地存储的用户信息
    const user = JSON.parse(localStorage.getItem('user') || '{}')
    user.email = profileForm.email
    localStorage.setItem('user', JSON.stringify(user))
    userStore.user.email = profileForm.email
    emailEditing.value = false
    profileForm.email = ''
  } catch (e) {
    console.error('Request failed:', e)
    ElMessage.error(e.response?.data?.error || '绑定失败')
  }
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
/* 更新提示 */
.update-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
}
.update-dialog {
  background: #fff;
  border-radius: 12px;
  padding: 32px;
  width: 400px;
  text-align: center;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
}
.update-icon {
  margin-bottom: 8px;
}
.update-btns {
  display: flex;
  justify-content: center;
  gap: 12px;
  margin-top: 16px;
}

.layout-container {
  height: 100vh;
}

.sidebar {
  background: #001529;
  color: #fff;
  display: flex;
  flex-direction: column;
}

.sidebar-menu {
  flex: 1;
}

.sidebar-footer {
  padding: 12px 16px;
  border-top: 1px solid rgba(255,255,255,0.1);
  background: rgba(0,0,0,0.2);
}

.tenant-trigger {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  border-radius: 6px;
  cursor: pointer;
  transition: background-color 0.2s;
  color: rgba(255,255,255,0.8);
}

.tenant-trigger:hover {
  background: rgba(255,255,255,0.1);
  color: #fff;
}

.tenant-label {
  font-size: 13px;
  flex: 1;
}

.tenant-list {
  display: flex;
  flex-direction: column;
}

.tenant-item {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 8px;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
  font-size: 12px;
  white-space: nowrap;
}

.tenant-item:hover {
  background: #f5f7fa;
}

.tenant-item.active {
  background: linear-gradient(135deg, #ecf5ff, #d9ecff);
  color: #409eff;
  font-weight: 500;
}

.check-icon {
  color: #409eff;
  font-size: 16px;
  flex-shrink: 0;
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
  gap: 20px;
}

/* 全局搜索 */
.header-search {
  flex: 1;
  max-width: 400px;
  position: relative;
}
.search-box {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 12px;
  border: 1px solid #e8e8e8;
  border-radius: 6px;
  background: #f5f7fa;
  transition: all 0.2s;
}
.search-box.focused {
  border-color: #409eff;
  background: #fff;
  box-shadow: 0 0 0 3px rgba(64,158,255,0.1);
}
.search-box input {
  flex: 1;
  border: none;
  background: none;
  outline: none;
  font-size: 13px;
  color: #333;
  font-family: inherit;
}
.search-box input::placeholder {
  color: #bbb;
}
.search-shortcut {
  font-size: 11px;
  color: #ccc;
  background: #fff;
  border: 1px solid #e8e8e8;
  border-radius: 3px;
  padding: 1px 5px;
  white-space: nowrap;
}
.search-tips {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  margin-top: 4px;
  background: #fff;
  border-radius: 6px;
  box-shadow: 0 8px 24px rgba(0,0,0,0.12);
  padding: 4px 0;
  z-index: 100;
  max-height: 300px;
  overflow-y: auto;
}
.search-tip-item {
  padding: 8px 14px;
  font-size: 13px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 8px;
  transition: background 0.15s;
}
.search-tip-item:hover {
  background: #f5f7fa;
}
.search-tip-item strong {
  color: #409eff;
}

.user-info {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  padding: 4px 8px;
  border-radius: 6px;
  transition: background-color 0.2s;
}

.user-info:hover {
  background-color: #f5f7fa;
}

.user-avatar {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background: linear-gradient(135deg, #409eff, #66b1ff);
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 14px;
  font-weight: 600;
}

.user-name {
  font-size: 14px;
  color: #303133;
  font-weight: 500;
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

.email-row {
  display: flex;
  gap: 8px;
  width: 100%;
}

.email-row .el-input {
  flex: 1;
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
  background: linear-gradient(135deg, #0f172a 0%, #1e293b 50%, #0f172a 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
}

.lock-card {
  text-align: center;
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(20px);
  border-radius: 20px;
  padding: 40px 50px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
}

.lock-avatar {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  background: linear-gradient(135deg, #3b82f6, #8b5cf6);
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 0 auto 20px;
  box-shadow: 0 4px 20px rgba(59, 130, 246, 0.4);
}

.lock-title {
  margin: 0 0 8px;
  font-size: 22px;
  font-weight: 600;
  color: #fff;
}

.lock-user {
  margin: 0 0 24px;
  font-size: 14px;
  color: rgba(255, 255, 255, 0.6);
}

.lock-input {
  width: 260px;
}

:deep(.lock-input .el-input__wrapper) {
  background: rgba(30, 41, 59, 0.8) !important;
  border: 1px solid rgba(255, 255, 255, 0.1) !important;
  box-shadow: none !important;
  height: 40px;
}

:deep(.lock-input .el-input__inner) {
  color: #fff !important;
}

:deep(.lock-input .el-input__inner::placeholder) {
  color: rgba(255, 255, 255, 0.4) !important;
}

.lock-btn {
  width: 260px;
  margin-top: 16px;
  height: 40px;
  background: linear-gradient(135deg, #3b82f6, #8b5cf6);
  border: none;
  font-size: 14px;
  font-weight: 500;
  padding: 0;
  line-height: 40px;
}

.lock-btn:hover {
  background: linear-gradient(135deg, #60a5fa, #a78bfa);
}

.lock-error {
  color: #ff6b6b;
  margin: 12px 0 0;
  font-size: 13px;
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
