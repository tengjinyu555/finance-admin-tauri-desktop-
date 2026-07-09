<template>
  <div class="login-page">
    <div class="bg-layer">
      <div class="particles" ref="particlesRef"></div>
      <div class="light-rays">
        <div class="ray ray-1"></div>
        <div class="ray ray-2"></div>
        <div class="ray ray-3"></div>
      </div>
      <div class="gradient-orb orb-1"></div>
      <div class="gradient-orb orb-2"></div>
      <div class="gradient-orb orb-3"></div>
    </div>

    <div class="container">
      <!-- 左侧品牌区 -->
      <div class="brand-section">
        <div class="brand-header">
          <div class="logo-wrapper">
            <div class="logo-ring"></div>
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

          <el-form :model="loginForm" :rules="loginRules" ref="loginFormRef" label-position="top" @submit.prevent="handleLogin">
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

          <div class="login-footer">
            <span>财税管理平台</span>
          </div>
        </el-card>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { User, Lock, Document, Monitor, DataAnalysis } from '@element-plus/icons-vue'
import { useUserStore } from '../stores/user'

const router = useRouter()
const userStore = useUserStore()

const loading = ref(false)
const rememberMe = ref(false)
const particlesRef = ref(null)
let animationFrame = null

const loginFormRef = ref()

const loginForm = reactive({
  username: '',
  password: ''
})

const loginRules = {
  username: [{ required: true, message: '请输入用户名', trigger: 'blur' }],
  password: [{ required: true, message: '请输入密码', trigger: 'blur' }]
}

// 页面加载时读取保存的凭证
onMounted(() => {
  const saved = localStorage.getItem('rememberedLogin')
  if (saved) {
    try {
      const data = JSON.parse(saved)
      loginForm.username = data.username
      loginForm.password = data.password
      rememberMe.value = true
    } catch (e) {}
  }
  initParticles()
})

onUnmounted(() => {
  if (animationFrame) {
    cancelAnimationFrame(animationFrame)
  }
})

// 粒子动画
const initParticles = () => {
  if (!particlesRef.value) return
  const container = particlesRef.value
  const particleCount = 50

  for (let i = 0; i < particleCount; i++) {
    const particle = document.createElement('div')
    particle.className = 'particle'
    const size = Math.random() * 4 + 2
    particle.style.width = size + 'px'
    particle.style.height = size + 'px'
    particle.style.left = Math.random() * 100 + '%'
    particle.style.top = Math.random() * 100 + '%'
    particle.style.animationDuration = (Math.random() * 20 + 10) + 's'
    particle.style.animationDelay = (Math.random() * 10) + 's'
    container.appendChild(particle)
  }
}

const handleLogin = async () => {
  await loginFormRef.value.validate()
  loading.value = true
  try {
    await userStore.login(loginForm.username, loginForm.password, null)
    // 记住密码
    if (rememberMe.value) {
      localStorage.setItem('rememberedLogin', JSON.stringify({
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
</script>

<style scoped>
.login-page {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #0f172a 0%, #1e293b 50%, #0f172a 100%);
  position: relative;
  padding: 20px;
  overflow: hidden;
}

.bg-layer {
  position: fixed;
  inset: 0;
  z-index: 0;
  pointer-events: none;
}

.particles {
  position: absolute;
  inset: 0;
  overflow: hidden;
}

:deep(.particle) {
  position: absolute;
  background: radial-gradient(circle, rgba(59, 130, 246, 0.8) 0%, transparent 70%);
  border-radius: 50%;
  animation: particleFloat linear infinite;
}

@keyframes particleFloat {
  0% { transform: translateY(100vh) rotate(0deg); opacity: 0; }
  10% { opacity: 1; }
  90% { opacity: 1; }
  100% { transform: translateY(-100px) rotate(720deg); opacity: 0; }
}

.light-rays {
  position: absolute;
  inset: 0;
  overflow: hidden;
}

.ray {
  position: absolute;
  width: 2px;
  height: 150%;
  background: linear-gradient(to bottom, transparent, rgba(59, 130, 246, 0.3), transparent);
  transform-origin: top center;
  animation: rayMove 8s ease-in-out infinite;
}

.ray-1 {
  left: 20%;
  animation-delay: 0s;
  transform: rotate(-15deg);
}

.ray-2 {
  left: 50%;
  animation-delay: -3s;
  transform: rotate(0deg);
}

.ray-3 {
  left: 80%;
  animation-delay: -6s;
  transform: rotate(15deg);
}

@keyframes rayMove {
  0%, 100% { opacity: 0.3; transform: rotate(-15deg) translateX(0); }
  50% { opacity: 0.6; transform: rotate(15deg) translateX(50px); }
}

.gradient-orb {
  position: absolute;
  border-radius: 50%;
  filter: blur(80px);
  animation: orbFloat 15s ease-in-out infinite;
}

.orb-1 {
  width: 400px;
  height: 400px;
  background: radial-gradient(circle, rgba(59, 130, 246, 0.4) 0%, transparent 70%);
  top: -100px;
  left: -100px;
  animation-delay: 0s;
}

.orb-2 {
  width: 300px;
  height: 300px;
  background: radial-gradient(circle, rgba(168, 85, 247, 0.3) 0%, transparent 70%);
  bottom: -50px;
  right: -50px;
  animation-delay: -5s;
}

.orb-3 {
  width: 250px;
  height: 250px;
  background: radial-gradient(circle, rgba(236, 72, 153, 0.25) 0%, transparent 70%);
  top: 50%;
  left: 60%;
  animation-delay: -10s;
}

@keyframes orbFloat {
  0%, 100% { transform: translate(0, 0) scale(1); }
  33% { transform: translate(50px, -50px) scale(1.2); }
  66% { transform: translate(-30px, 30px) scale(0.8); }
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
  background: linear-gradient(135deg, #3b82f6, #8b5cf6);
  border-radius: 14px;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 4px 20px rgba(59, 130, 246, 0.4);
  position: relative;
  animation: logoGlow 3s ease-in-out infinite;
}

.logo-ring {
  position: absolute;
  inset: -4px;
  border-radius: 18px;
  border: 2px solid rgba(59, 130, 246, 0.5);
  animation: ringPulse 2s ease-in-out infinite;
}

@keyframes logoGlow {
  0%, 100% { box-shadow: 0 4px 20px rgba(59, 130, 246, 0.4); }
  50% { box-shadow: 0 8px 40px rgba(59, 130, 246, 0.6); }
}

@keyframes ringPulse {
  0%, 100% { transform: scale(1); opacity: 1; }
  50% { transform: scale(1.1); opacity: 0.5; }
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
  color: #ffffff;
  letter-spacing: 2px;
  font-family: 'Noto Serif SC', serif;
  text-shadow: 0 2px 10px rgba(59, 130, 246, 0.3);
}

.brand-subtitle {
  font-size: 11px;
  color: rgba(255, 255, 255, 0.5);
  font-weight: 500;
  letter-spacing: 4px;
  text-transform: uppercase;
}

.brand-tagline {
  font-size: 14px;
  color: rgba(255, 255, 255, 0.6);
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
  color: rgba(255, 255, 255, 0.7);
  animation: fadeInLeft 0.6s ease-out forwards;
  opacity: 0;
  transition: all 0.3s ease;
}

.feature-item:hover {
  color: #ffffff;
  transform: translateX(10px);
}

.feature-item:nth-child(1) { animation-delay: 0.2s; }
.feature-item:nth-child(2) { animation-delay: 0.4s; }
.feature-item:nth-child(3) { animation-delay: 0.6s; }
.feature-item:nth-child(4) { animation-delay: 0.8s; }

@keyframes fadeInLeft {
  0% { opacity: 0; transform: translateX(-30px); }
  100% { opacity: 1; transform: translateX(0); }
}

.feature-icon {
  width: 36px;
  height: 36px;
  background: linear-gradient(135deg, rgba(59, 130, 246, 0.2), rgba(139, 92, 246, 0.2));
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #60a5fa;
  transition: all 0.3s ease;
  border: 1px solid rgba(59, 130, 246, 0.3);
}

.feature-item:hover .feature-icon {
  background: linear-gradient(135deg, rgba(59, 130, 246, 0.4), rgba(139, 92, 246, 0.4));
  transform: scale(1.15) rotate(5deg);
  box-shadow: 0 4px 20px rgba(59, 130, 246, 0.4);
}

.login-section {
  width: 100%;
  max-width: 420px;
  margin-left: auto;
}

.login-card {
  border-radius: 20px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  background: rgba(15, 23, 42, 0.8);
  backdrop-filter: blur(20px);
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
  transition: all 0.4s ease;
  animation: slideIn 0.8s ease-out;
}

.login-card:hover {
  box-shadow: 0 30px 60px -15px rgba(59, 130, 246, 0.3);
  transform: translateY(-8px);
  border-color: rgba(59, 130, 246, 0.3);
}

@keyframes slideIn {
  0% { opacity: 0; transform: translateY(40px) scale(0.95); }
  100% { opacity: 1; transform: translateY(0) scale(1); }
}

.login-card :deep(.el-card__body) {
  padding: 40px 36px 32px;
}

.login-header {
  text-align: center;
  margin-bottom: 32px;
}

.login-header h2 {
  font-size: 24px;
  font-weight: 700;
  color: #ffffff;
  margin-bottom: 8px;
  background: linear-gradient(135deg, #ffffff, #60a5fa);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.login-header p {
  font-size: 13px;
  color: rgba(255, 255, 255, 0.5);
}

.login-btn {
  width: 100%;
  height: 48px;
  font-size: 15px;
  font-weight: 600;
  letter-spacing: 2px;
  background: linear-gradient(135deg, #3b82f6, #8b5cf6);
  border: none;
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;
}

.login-btn::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
  transition: left 0.5s ease;
}

.login-btn:hover::before {
  left: 100%;
}

.login-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 10px 30px rgba(59, 130, 246, 0.4);
}

.login-footer {
  text-align: center;
  margin-top: 20px;
  font-size: 12px;
  color: rgba(255, 255, 255, 0.4);
}

.form-tip {
  font-size: 12px;
  color: #999;
  margin-top: 4px;
}

:deep(.el-form-item__label) {
  color: rgba(255, 255, 255, 0.7) !important;
}

:deep(.el-input__wrapper) {
  background: rgba(30, 41, 59, 0.8) !important;
  border: 1px solid rgba(255, 255, 255, 0.1) !important;
  box-shadow: none !important;
}

:deep(.el-input__wrapper:hover) {
  border-color: rgba(59, 130, 246, 0.5) !important;
}

:deep(.el-input__wrapper.is-focus) {
  border-color: #3b82f6 !important;
  box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.2) !important;
}

:deep(.el-input__inner) {
  color: #ffffff !important;
}

:deep(.el-input__inner::placeholder) {
  color: rgba(255, 255, 255, 0.4) !important;
}

:deep(.el-checkbox__label) {
  color: rgba(255, 255, 255, 0.7) !important;
}

:deep(.el-checkbox__inner) {
  background: rgba(30, 41, 59, 0.8) !important;
  border-color: rgba(255, 255, 255, 0.2) !important;
}

:deep(.el-checkbox__input.is-checked .el-checkbox__inner) {
  background: #3b82f6 !important;
  border-color: #3b82f6 !important;
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
  .orb-1, .orb-2, .orb-3 {
    display: none;
  }
}
</style>
