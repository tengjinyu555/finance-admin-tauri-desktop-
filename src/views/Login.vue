<template>
  <div class="login-page">
    <div class="bg-layer"><div class="bg-grid"></div></div>

    <div class="container">
      <!-- 左侧品牌区 -->
      <div class="brand-section">
        <div class="brand-header">
          <div class="logo-wrapper">
            <span class="logo-text">税</span>
          </div>
          <div class="brand-title">
            <span class="brand-name">财税管理平台</span>
            <span class="brand-subtitle">FINANCE ADMIN PLATFORM</span>
          </div>
        </div>
        <p class="brand-tagline">
          专业的供应商进销项发票台账管理系统，<br>
          为企业提供安全、高效、智能的财税解决方案。
        </p>
        <div class="brand-features">
          <div class="feature-item">
            <div class="feature-icon"><el-icon><Document /></el-icon></div>
            <span>进销项发票全生命周期管理</span>
          </div>
          <div class="feature-item">
            <div class="feature-icon"><el-icon><Monitor /></el-icon></div>
            <span>OCR智能识别，一键导入</span>
          </div>
          <div class="feature-item">
            <div class="feature-icon"><el-icon><DataAnalysis /></el-icon></div>
            <span>多维度税金报表分析</span>
          </div>
          <div class="feature-item">
            <div class="feature-icon"><el-icon><Lock /></el-icon></div>
            <span>多租户数据安全隔离</span>
          </div>
        </div>
      </div>

      <!-- 右侧登录区 -->
      <div class="login-section">
        <el-card class="login-card">
          <div class="login-header">
            <h2>系统登录</h2>
            <p>请使用账号密码登录系统</p>
          </div>

          <el-tabs v-model="activeTab">
            <el-tab-pane label="登录" name="login">
              <el-form :model="loginForm" :rules="loginRules" ref="loginFormRef" label-position="top" @submit.prevent="handleLogin">
                <el-form-item label="所属企业" required>
                  <el-select
                    v-model="loginForm.tenantId"
                    filterable
                    remote
                    :remote-method="searchTenant"
                    :loading="tenantLoading"
                    placeholder="输入企业名称搜索..."
                    style="width: 100%"
                  >
                    <el-option
                      v-for="t in tenantList"
                      :key="t.id"
                      :label="t.name"
                      :value="t.id"
                    >
                      <span>{{ t.name }}</span>
                      <span style="float: right; color: #999; font-size: 12px">{{ t.code }}</span>
                    </el-option>
                  </el-select>
                </el-form-item>
                <el-form-item label="用户名" prop="username">
                  <el-input v-model="loginForm.username" placeholder="请输入用户名" :prefix-icon="User" />
                </el-form-item>
                <el-form-item label="密码" prop="password">
                  <el-input v-model="loginForm.password" type="password" placeholder="请输入密码" :prefix-icon="Lock" show-password />
                </el-form-item>
                <el-form-item>
                  <el-checkbox v-model="rememberMe">记住密码</el-checkbox>
                </el-form-item>
                <el-button type="primary" :loading="loading" @click="handleLogin" class="login-btn">登 录</el-button>
              </el-form>
            </el-tab-pane>

            <el-tab-pane label="注册" name="register">
              <el-form :model="registerForm" :rules="registerRules" ref="registerFormRef" label-position="top">
                <el-form-item label="所属企业">
                  <el-select
                    v-model="registerForm.tenantId"
                    filterable
                    remote
                    :remote-method="searchTenant"
                    :loading="tenantLoading"
                    placeholder="输入租户名称搜索..."
                    style="width: 100%"
                  >
                    <el-option
                      v-for="t in tenantList"
                      :key="t.id"
                      :label="t.name"
                      :value="t.id"
                    />
                  </el-select>
                </el-form-item>
                <el-form-item label="用户名" prop="username">
                  <el-input v-model="registerForm.username" placeholder="请输入用户名" />
                </el-form-item>
                <el-form-item label="昵称" prop="nickname">
                  <el-input v-model="registerForm.nickname" placeholder="请输入昵称" />
                </el-form-item>
                <el-form-item label="密码" prop="password">
                  <el-input v-model="registerForm.password" type="password" placeholder="请输入密码" show-password />
                </el-form-item>
                <el-form-item label="确认密码" prop="password2">
                  <el-input v-model="registerForm.password2" type="password" placeholder="请确认密码" show-password />
                </el-form-item>
                <el-form-item label="角色">
                  <el-checkbox-group v-model="registerForm.roles">
                    <el-checkbox label="admin">管理员</el-checkbox>
                    <el-checkbox label="finance">财务录入</el-checkbox>
                    <el-checkbox label="viewer">普通查看</el-checkbox>
                  </el-checkbox-group>
                </el-form-item>
                <el-button type="primary" :loading="loading" @click="handleRegister" class="login-btn">注 册</el-button>
              </el-form>
            </el-tab-pane>
          </el-tabs>

          <div class="login-footer">
            <span>财税管理平台</span>
          </div>
        </el-card>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { User, Lock, Document, Monitor, DataAnalysis } from '@element-plus/icons-vue'
import { useUserStore } from '../stores/user'
import { AuthApi, TenantApi } from '../api/invoice'

const router = useRouter()
const userStore = useUserStore()

const activeTab = ref('login')
const loading = ref(false)
const tenantLoading = ref(false)
const tenantList = ref([])
const rememberMe = ref(false)

const loginFormRef = ref()
const registerFormRef = ref()

const loginForm = reactive({
  tenantId: '',
  username: '',
  password: ''
})

const registerForm = reactive({
  tenantId: '',
  username: '',
  nickname: '',
  password: '',
  password2: '',
  roles: ['viewer']
})

const loginRules = {
  username: [{ required: true, message: '请输入用户名', trigger: 'blur' }],
  password: [{ required: true, message: '请输入密码', trigger: 'blur' }]
}

const registerRules = {
  username: [{ required: true, message: '请输入用户名', trigger: 'blur' }],
  nickname: [{ required: true, message: '请输入昵称', trigger: 'blur' }],
  password: [{ required: true, message: '请输入密码', trigger: 'blur' }]
}

// 页面加载时读取保存的凭证
onMounted(() => {
  const saved = localStorage.getItem('rememberedLogin')
  if (saved) {
    try {
      const data = JSON.parse(saved)
      loginForm.tenantId = data.tenantId
      loginForm.username = data.username
      loginForm.password = data.password
      rememberMe.value = true
      // 恢复企业选项显示
      if (data.tenantName) {
        tenantList.value = [{ id: data.tenantId, name: data.tenantName, code: data.tenantCode || '' }]
      }
    } catch (e) {}
  }
})

const searchTenant = async (query) => {
  if (!query) return
  tenantLoading.value = true
  try {
    tenantList.value = await TenantApi.search(query)
  } catch (e) {
    console.error(e)
  }
  tenantLoading.value = false
}

const handleLogin = async () => {
  if (!loginForm.tenantId) {
    ElMessage.warning('请选择企业')
    return
  }
  await loginFormRef.value.validate()
  loading.value = true
  try {
    await userStore.login(loginForm.username, loginForm.password, loginForm.tenantId)
    // 记住密码
    if (rememberMe.value) {
      const selectedTenant = tenantList.value.find(t => t.id === loginForm.tenantId)
      localStorage.setItem('rememberedLogin', JSON.stringify({
        tenantId: loginForm.tenantId,
        tenantName: selectedTenant?.name || '',
        tenantCode: selectedTenant?.code || '',
        username: loginForm.username,
        password: loginForm.password
      }))
    } else {
      localStorage.removeItem('rememberedLogin')
    }
    ElMessage.success('登录成功')
    router.push('/')
  } catch (e) {
    ElMessage.error(e.response?.data?.error || '登录失败')
  }
  loading.value = false
}

const handleRegister = async () => {
  if (!registerForm.tenantId) {
    ElMessage.warning('请选择租户')
    return
  }
  await registerFormRef.value.validate()
  if (registerForm.password !== registerForm.password2) {
    ElMessage.error('两次密码不一致')
    return
  }
  loading.value = true
  try {
    await AuthApi.register({
      username: registerForm.username,
      password: registerForm.password,
      nickname: registerForm.nickname,
      role: registerForm.roles
    }, registerForm.tenantId)
    ElMessage.success('注册成功，请登录')
    activeTab.value = 'login'
  } catch (e) {
    ElMessage.error(e.response?.data?.error || '注册失败')
  }
  loading.value = false
}
</script>

<style scoped>
.login-page {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #f0f2f5;
  position: relative;
  padding: 20px;
}

.bg-layer {
  position: fixed;
  inset: 0;
  z-index: 0;
  pointer-events: none;
  background:
    linear-gradient(135deg, rgba(30, 64, 175, 0.04) 0%, transparent 50%),
    linear-gradient(225deg, rgba(217, 119, 6, 0.03) 0%, transparent 50%);
}

.bg-grid {
  position: absolute;
  inset: 0;
  background-image:
    linear-gradient(rgba(30, 64, 175, 0.04) 1px, transparent 1px),
    linear-gradient(90deg, rgba(30, 64, 175, 0.04) 1px, transparent 1px);
  background-size: 72px 72px;
  mask-image: radial-gradient(ellipse 60% 50% at 50% 50%, black 30%, transparent 70%);
}

.container {
  width: 100%;
  max-width: 1120px;
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 64px;
  align-items: center;
  position: relative;
  z-index: 1;
}

.brand-section {
  display: flex;
  flex-direction: column;
  gap: 28px;
}

.brand-header {
  display: flex;
  align-items: center;
  gap: 16px;
}

.logo-wrapper {
  width: 56px;
  height: 56px;
  background: linear-gradient(135deg, #1e40af, #3b82f6);
  border-radius: 14px;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 4px 16px rgba(30, 64, 175, 0.15);
}

.logo-text {
  font-size: 24px;
  font-weight: 700;
  color: #fff;
  font-family: 'Noto Serif SC', serif;
}

.brand-title {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.brand-name {
  font-size: 28px;
  font-weight: 700;
  color: #111827;
  letter-spacing: 2px;
  font-family: 'Noto Serif SC', serif;
}

.brand-subtitle {
  font-size: 11px;
  color: #9ca3af;
  font-weight: 500;
  letter-spacing: 4px;
  text-transform: uppercase;
}

.brand-tagline {
  font-size: 14px;
  color: #6b7280;
  line-height: 1.8;
}

.brand-features {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.feature-item {
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: 13px;
  color: #6b7280;
}

.feature-icon {
  width: 32px;
  height: 32px;
  background: rgba(30, 64, 175, 0.06);
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #3b82f6;
}

.login-section {
  width: 100%;
  max-width: 420px;
  margin-left: auto;
}

.login-card {
  border-radius: 16px;
  border: 1px solid #e5e7eb;
  box-shadow: 0 8px 40px rgba(0,0,0,0.08);
}

.login-card :deep(.el-card__body) {
  padding: 40px 36px 32px;
}

.login-header {
  text-align: center;
  margin-bottom: 32px;
}

.login-header h2 {
  font-size: 22px;
  font-weight: 700;
  color: #111827;
  margin-bottom: 6px;
}

.login-header p {
  font-size: 13px;
  color: #9ca3af;
}

.login-btn {
  width: 100%;
  height: 48px;
  font-size: 15px;
  font-weight: 600;
  letter-spacing: 2px;
}

.login-footer {
  text-align: center;
  margin-top: 20px;
  font-size: 12px;
  color: #9ca3af;
}

.form-tip {
  font-size: 12px;
  color: #999;
  margin-top: 4px;
}

@media (max-width: 968px) {
  .container {
    grid-template-columns: 1fr;
    gap: 40px;
    max-width: 440px;
  }
  .brand-section {
    text-align: center;
    align-items: center;
  }
  .brand-header {
    flex-direction: column;
    align-items: center;
  }
  .brand-title {
    align-items: center;
  }
  .brand-tagline {
    text-align: center;
  }
  .brand-features {
    display: none;
  }
}
</style>
