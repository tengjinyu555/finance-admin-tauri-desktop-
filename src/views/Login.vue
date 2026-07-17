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
            <span class="brand-name">业财一体化管理</span>
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
          <!-- 公告铃铛 -->
          <el-popover v-if="announcements.length > 0" placement="top" :width="350" trigger="click" @after-leave="markAnnouncementViewed">
            <template #reference>
              <el-badge is-dot class="announcement-badge">
                <el-icon class="bell-icon"><Bell /></el-icon>
              </el-badge>
            </template>
            <div class="announcement-popover">
              <div class="announcement-popover-title">系统公告</div>
              <div v-for="(item, index) in announcements" :key="index" class="announcement-popover-item">
                <div class="announcement-popover-item-title">{{ item.title }}</div>
                <div class="announcement-popover-item-content">{{ item.content }}</div>
              </div>
            </div>
          </el-popover>

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
              <div class="form-options">
                <el-checkbox v-model="rememberMe">记住密码</el-checkbox>
                <el-button type="primary" link @click="forgotPasswordVisible = true; forgotPasswordError = ''; resetCodeError = ''">忘记密码？</el-button>
              </div>
            </el-form-item>
            <el-form-item>
              <el-checkbox v-model="agreePrivacyLogin">我已阅读并同意<el-button type="primary" link @click.prevent="privacyDialogVisible = true">《隐私政策》</el-button></el-checkbox>
            </el-form-item>
            <el-button type="primary" :loading="loading" @click="handleLogin" class="login-btn">登 录</el-button>
          </el-form>

          <div class="login-footer">
            <span>业财一体化管理</span>
            <div style="margin-top: 12px;">
              <el-button type="primary" link @click="registerDialogVisible = true">注册</el-button>
            </div>
          </div>
        </el-card>
      </div>
    </div>
  </div>

  <!-- 忘记密码对话框 -->
  <el-dialog v-model="forgotPasswordVisible" width="420px" destroy-on-close :show-close="true" :close-on-click-modal="false" class="register-dialog">
    <div class="register-container">
      <div class="register-header">
        <div class="register-logo">
          <span class="logo-text">密</span>
        </div>
        <h2>找回密码</h2>
        <p>请输入注册时使用的邮箱地址</p>
      </div>

      <el-form :model="forgotPasswordForm" :rules="forgotPasswordRules" ref="forgotPasswordFormRef" label-position="top" class="register-form">
        <el-form-item label="邮箱地址" prop="email" :error="forgotPasswordError">
          <el-input v-model="forgotPasswordForm.email" placeholder="请输入邮箱地址" :prefix-icon="Message" @input="forgotPasswordError = ''" :disabled="resetCodeSent" />
        </el-form-item>
        <el-button v-if="!resetCodeSent" type="primary" @click="handleSendResetCode" :loading="forgotPasswordLoading" class="register-btn">发送验证码</el-button>
      </el-form>

      <!-- 验证码输入和重置密码 -->
      <el-form v-if="resetCodeSent" :model="resetPasswordForm" :rules="resetPasswordRules" ref="resetPasswordFormRef" label-position="top" class="register-form" style="margin-top: 20px;">
        <el-form-item label="验证码" prop="code" :error="resetCodeError">
          <div class="code-row">
            <el-input v-model="resetPasswordForm.code" placeholder="请输入6位验证码" @input="resetCodeError = ''" />
            <el-button size="small" @click="handleResendCode" :disabled="resendCooldown > 0" class="resend-btn">
              {{ resendCooldown > 0 ? `${resendCooldown}s后重发` : '重新发送' }}
            </el-button>
          </div>
        </el-form-item>
        <el-form-item label="新密码" prop="newPassword">
          <el-input v-model="resetPasswordForm.newPassword" type="password" placeholder="至少6位" show-password />
        </el-form-item>
        <el-form-item label="确认密码" prop="confirmPassword">
          <el-input v-model="resetPasswordForm.confirmPassword" type="password" placeholder="请确认密码" show-password />
        </el-form-item>
        <el-button type="primary" @click="handleResetPassword" :loading="resetPasswordLoading" class="register-btn">重置密码</el-button>
      </el-form>

      <div class="register-footer">
        已有账号？<el-button type="primary" link @click="forgotPasswordVisible = false; forgotPasswordError = ''; resetCodeError = ''">返回登录</el-button>
      </div>
    </div>
  </el-dialog>

  <!-- 注册对话框 -->
  <el-dialog v-model="registerDialogVisible" width="420px" destroy-on-close :show-close="true" class="register-dialog">
    <div class="register-container">
      <div class="register-header">
        <div class="register-logo">
          <span class="logo-text">业</span>
        </div>
        <h2>注册</h2>
        <p>创建您的账号，开始使用业财一体化管理</p>
      </div>

      <el-form :model="registerForm" :rules="registerRules" ref="registerFormRef" label-position="top" class="register-form">
        <el-form-item label="企业/个人名称" prop="tenantName">
          <el-input v-model="registerForm.tenantName" placeholder="请输入企业或个人名称" :prefix-icon="OfficeBuilding" />
        </el-form-item>
        <el-form-item label="管理员账号" prop="username">
          <el-input v-model="registerForm.username" placeholder="请输入管理员账号" :prefix-icon="User" />
        </el-form-item>
        <el-form-item label="邮箱（用于找回密码）" prop="email">
          <el-input v-model="registerForm.email" placeholder="请输入邮箱地址" :prefix-icon="Message" />
        </el-form-item>
        <el-row :gutter="16">
          <el-col :span="12">
            <el-form-item label="密码" prop="password">
              <el-input v-model="registerForm.password" type="password" placeholder="至少6位" :prefix-icon="Lock" show-password />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="确认密码" prop="password2">
              <el-input v-model="registerForm.password2" type="password" placeholder="请确认密码" :prefix-icon="Lock" show-password />
            </el-form-item>
          </el-col>
        </el-row>
        <el-form-item>
          <el-checkbox v-model="agreePrivacy">我已阅读并同意<el-button type="primary" link @click.prevent="privacyDialogVisible = true">《隐私政策》</el-button></el-checkbox>
        </el-form-item>
        <el-button type="primary" @click="handleRegister" :loading="registerLoading" class="register-btn">立即注册</el-button>
      </el-form>

      <div class="register-footer">
        已有账号？<el-button type="primary" link @click="registerDialogVisible = false">返回登录</el-button>
      </div>
    </div>
  </el-dialog>

  <!-- 隐私政策弹窗 -->
  <el-dialog v-model="privacyDialogVisible" title="业财一体化管理系统隐私政策" width="700px" :close-on-click-modal="false" class="privacy-dialog">
    <div class="privacy-content">
      <h3>一、前言</h3>
      <p>本《隐私政策》适用于您使用【业财一体化管理系统】（下称 "本系统"），系统由开发者独立开发运营，仅供个人、个体工商户日常记账、发票台账、收支管理自用。</p>
      <p>您注册、登录、使用本系统即代表您已完整阅读、理解并同意本隐私政策全部条款；若不同意，请勿注册及使用本系统。</p>

      <h3>二、我们收集哪些信息</h3>
      <p><strong>（一）您主动提交的注册登录信息</strong></p>
      <p>注册账号：您填写的手机号（用于登录、找回密码、账号核验）；</p>
      <p>可选补充：您自行填写的昵称、个体户名称、经营地址（仅用于台账单据展示，不强制填写）。</p>
      <p><strong>（二）记账业务数据（核心经营数据）</strong></p>
      <p>您自愿录入的全部业务信息，仅存储用于您自身记账核算：客户信息、项目信息、发票信息、收支流水、操作记录。</p>
      <p><strong>（三）设备与访问基础信息（无追踪采集）</strong></p>
      <p>仅记录基础访问日志保障系统稳定，不含追踪类信息。</p>
      <p><strong>（四）不主动采集的信息</strong></p>
      <p>本系统不会强制索要身份证、银行卡、对公账户、人脸、语音等敏感隐私信息。</p>

      <h3>三、信息使用规则（仅限自用记账，绝不滥用）</h3>
      <p>我们仅基于以下目的使用您的信息，不会超出范围使用：账号服务、记账核心功能、系统维护、安全风控、合规义务。</p>
      <p><strong>重要承诺：</strong>您所有经营、发票、收支数据，仅为您个人记账使用，开发者不会将您的记账数据出售、出租、共享、泄露给任何第三方。</p>

      <h3>四、信息存储与安全保护</h3>
      <p>存储期限：您的账号及记账数据，在您账号存续期间持续保存；若您申请注销账号，我们将在 7 个工作日内彻底删除您全部数据，无备份留存。</p>

      <h3>五、信息共享与对外披露限制</h3>
      <p>无第三方商业共享：绝不向财税中介、广告公司、企业平台出售、交换您的任何记账、客户、发票数据。</p>

      <h3>六、您拥有的个人信息权利</h3>
      <p>访问/查看权、修改更正权、删除权、撤回同意权、咨询反馈权。</p>

      <h3>七、未成年人使用说明</h3>
      <p>本系统面向个体户、成年个人经营记账使用，不面向未成年人；未满 18 周岁请勿注册、使用本系统。</p>

      <h3>八、Cookie 与本地缓存说明</h3>
      <p>系统仅在本地浏览器缓存登录状态（记住密码功能），不会使用追踪 Cookie 采集浏览行为。</p>

      <h3>九、隐私政策更新说明</h3>
      <p>若后续系统功能迭代，需调整本隐私政策，会在登录页公告提示更新。</p>

      <h3>十、联系方式与纠纷说明</h3>
      <p>数据/隐私问题对接：开发者</p>
      <p>适用法律：本政策适用中华人民共和国《个人信息保护法》《网络安全法》《电子商务法》相关法律法规。</p>

      <h3>十一、特别提示（个体户记账重点）</h3>
      <p>本系统仅为记账辅助工具，系统内生成的报表、税额仅作参考，不替代专业财税机构报税建议。</p>
    </div>
    <template #footer>
      <el-button type="primary" @click="privacyDialogVisible = false">我知道了</el-button>
    </template>
  </el-dialog>
</template>

<script setup>
import { ref, reactive, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { User, Lock, Document, Monitor, DataAnalysis, OfficeBuilding, Message, Bell } from '@element-plus/icons-vue'
import { useUserStore } from '../stores/user'
import api from '../api/index'

const router = useRouter()
const userStore = useUserStore()

const loading = ref(false)
const rememberMe = ref(false)
const particlesRef = ref(null)
const announcements = ref([])
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

// 注册相关
const registerDialogVisible = ref(false)
const registerLoading = ref(false)
const registerFormRef = ref()
const agreePrivacy = ref(false)
const agreePrivacyLogin = ref(false)
const privacyDialogVisible = ref(false)

const registerForm = reactive({
  tenantName: '',
  username: '',
  email: '',
  password: '',
  password2: ''
})

const registerRules = {
  tenantName: [{ required: true, message: '请输入企业/个人名称', trigger: 'blur' }],
  username: [{ required: true, message: '请输入管理员账号', trigger: 'blur' }],
  email: [
    { required: true, message: '请输入邮箱地址', trigger: 'blur' },
    { type: 'email', message: '请输入正确的邮箱格式', trigger: 'blur' }
  ],
  password: [
    { required: true, message: '请输入密码', trigger: 'blur' },
    { min: 6, message: '密码至少6位', trigger: 'blur' }
  ],
  password2: [{ required: true, message: '请确认密码', trigger: 'blur' }]
}

// 忘记密码相关
const forgotPasswordVisible = ref(false)
const forgotPasswordLoading = ref(false)
const forgotPasswordError = ref('')
const resetCodeSent = ref(false)
const resetPasswordLoading = ref(false)
const resetCodeError = ref('')
const resendCooldown = ref(0)
let resendTimer = null
const forgotPasswordFormRef = ref()
const resetPasswordFormRef = ref()

const forgotPasswordForm = reactive({
  email: ''
})

const resetPasswordForm = reactive({
  code: '',
  newPassword: '',
  confirmPassword: ''
})

const forgotPasswordRules = {
  email: [
    { required: true, message: '请输入邮箱地址', trigger: 'blur' },
    { type: 'email', message: '请输入正确的邮箱格式', trigger: 'blur' }
  ]
}

const resetPasswordRules = {
  code: [{ required: true, message: '请输入验证码', trigger: 'blur' }],
  newPassword: [
    { required: true, message: '请输入新密码', trigger: 'blur' },
    { min: 6, message: '密码至少6位', trigger: 'blur' }
  ],
  confirmPassword: [{ required: true, message: '请确认密码', trigger: 'blur' }]
}

const handleSendResetCode = async () => {
  if (forgotPasswordLoading.value) return
  forgotPasswordError.value = ''
  try {
    await forgotPasswordFormRef.value.validate()
  } catch (e) {
    return
  }
  forgotPasswordLoading.value = true
  try {
    await api.post('/auth/forgot-password', { email: forgotPasswordForm.email })
    ElMessage.success('验证码已发送到您的邮箱')
    resetCodeSent.value = true
    startResendCooldown()
  } catch (e) {
    forgotPasswordError.value = e.response?.data?.error || '发送验证码失败'
  }
  forgotPasswordLoading.value = false
}

const startResendCooldown = () => {
  resendCooldown.value = 60
  if (resendTimer) clearInterval(resendTimer)
  resendTimer = setInterval(() => {
    resendCooldown.value--
    if (resendCooldown.value <= 0) {
      clearInterval(resendTimer)
    }
  }, 1000)
}

const handleResendCode = async () => {
  if (resendCooldown.value > 0) return
  forgotPasswordError.value = ''
  resetCodeError.value = ''
  try {
    await api.post('/auth/forgot-password', { email: forgotPasswordForm.email })
    ElMessage.success('验证码已重新发送到您的邮箱')
    startResendCooldown()
  } catch (e) {
    resetCodeError.value = e.response?.data?.error || '发送验证码失败'
  }
}

const handleResetPassword = async () => {
  resetCodeError.value = ''
  try {
    await resetPasswordFormRef.value.validate()
  } catch (e) {
    return
  }
  if (resetPasswordForm.newPassword !== resetPasswordForm.confirmPassword) {
    resetCodeError.value = '两次密码不一致'
    return
  }
  resetPasswordLoading.value = true
  try {
    await api.post('/auth/reset-password', {
      code: resetPasswordForm.code,
      newPassword: resetPasswordForm.newPassword
    })
    ElMessage.success('密码重置成功，请使用新密码登录')
    forgotPasswordVisible.value = false
    // 清空表单
    forgotPasswordForm.email = ''
    resetPasswordForm.code = ''
    resetPasswordForm.newPassword = ''
    resetPasswordForm.confirmPassword = ''
    resetCodeSent.value = false
  } catch (e) {
    resetCodeError.value = e.response?.data?.error || '重置密码失败'
  }
  resetPasswordLoading.value = false
}

const handleRegister = async () => {
  if (!agreePrivacy.value) {
    ElMessage.warning('请阅读并勾选隐私政策')
    return
  }
  await registerFormRef.value.validate()
  if (registerForm.password !== registerForm.password2) {
    ElMessage.error('两次密码不一致')
    return
  }
  registerLoading.value = true
  try {
    await api.post('/auth/register-tenant', {
      tenantName: registerForm.tenantName,
      username: registerForm.username,
      email: registerForm.email,
      password: registerForm.password
    })
    ElMessage.success('注册成功，请登录')
    registerDialogVisible.value = false
    loginForm.username = registerForm.username
    loginForm.password = registerForm.password
  } catch (e) {
    ElMessage.error(e.response?.data?.error || '注册失败')
  }
  registerLoading.value = false
}

// 页面加载时读取保存的凭证
onMounted(async () => {
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
  // 加载系统公告
  loadAnnouncements()
})

// 加载系统公告
const loadAnnouncements = async () => {
  try {
    const data = await api.get('/announcements')
    const activeAnnouncements = (data || []).filter(a => a.status === 'active')
    // 检查是否已经查看过
    const lastViewTime = localStorage.getItem('announcementViewTime')
    const lastAnnouncementCount = localStorage.getItem('announcementCount')
    if (lastViewTime && lastAnnouncementCount === String(activeAnnouncements.length)) {
      announcements.value = []
    } else {
      announcements.value = activeAnnouncements
    }
  } catch (e) {
    console.error('加载公告失败:', e)
  }
}

// 点击铃铛后标记为已查看
const markAnnouncementViewed = () => {
  localStorage.setItem('announcementViewTime', Date.now())
  localStorage.setItem('announcementCount', String(announcements.value.length))
  announcements.value = []
}

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
  if (!agreePrivacyLogin.value) {
    ElMessage.warning('请阅读并勾选隐私政策')
    return
  }
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
  flex-direction: column;
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
  position: relative;
}

.announcement-badge {
  position: absolute;
  top: 16px;
  right: 16px;
  z-index: 10;
}

.bell-icon {
  font-size: 18px;
  color: rgba(255, 255, 255, 0.7);
  cursor: pointer;
  transition: all 0.3s ease;
}

.bell-icon:hover {
  color: #93c5fd;
  transform: scale(1.1);
}

:deep(.announcement-badge .el-badge__content) {
  background-color: #f56c6c;
}

.announcement-popover {
  max-height: 300px;
  overflow-y: auto;
}

.announcement-popover-title {
  font-size: 15px;
  font-weight: 600;
  color: #303133;
  padding-bottom: 10px;
  border-bottom: 1px solid #ebeef5;
  margin-bottom: 10px;
}

.announcement-popover-item {
  padding: 10px 0;
  border-bottom: 1px solid #f5f5f5;
}

.announcement-popover-item:last-child {
  border-bottom: none;
}

.announcement-popover-item-title {
  font-size: 14px;
  font-weight: 500;
  color: #303133;
  margin-bottom: 6px;
}

.announcement-popover-item-content {
  font-size: 13px;
  color: #606266;
  line-height: 1.6;
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

.form-options {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
}

.code-row {
  display: flex;
  gap: 8px;
  width: 100%;
  align-items: center;
}

.code-row .el-input {
  flex: 1;
}

.resend-btn {
  white-space: nowrap;
  min-width: 100px;
  height: 32px;
  font-size: 13px;
  padding: 0 12px;
  border-radius: 6px;
  background: transparent;
  border: 1px solid #409eff;
  color: #409eff;
}

.resend-btn:hover {
  background: rgba(64, 158, 255, 0.1);
  border-color: #66b1ff;
  color: #66b1ff;
}

.resend-btn:disabled {
  background: transparent;
  border-color: #4c4d4f;
  color: #4c4d4f;
  cursor: not-allowed;
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

/* 隐私政策弹窗样式 */
:deep(.privacy-dialog .el-dialog__header) {
  display: flex;
  justify-content: center;
}

:deep(.privacy-dialog .el-dialog__title) {
  font-size: 18px;
  font-weight: 600;
  color: #1a1a2e;
}
.privacy-content {
  max-height: 500px;
  overflow-y: auto;
  padding: 8px 16px;
  line-height: 1.8;
}

.privacy-content h3 {
  font-size: 15px;
  font-weight: 600;
  color: #1a1a2e;
  margin: 20px 0 10px;
  padding-bottom: 6px;
  border-bottom: 1px solid #ebeef5;
}

.privacy-content h3:first-child {
  margin-top: 0;
}

.privacy-content p {
  font-size: 14px;
  color: #606266;
  line-height: 2;
  margin: 6px 0;
  text-indent: 2em;
}

.privacy-content p strong {
  color: #303133;
  text-indent: 0;
}

/* 滚动条美化 */
.privacy-content::-webkit-scrollbar {
  width: 6px;
}

.privacy-content::-webkit-scrollbar-track {
  background: #f1f1f1;
  border-radius: 3px;
}

.privacy-content::-webkit-scrollbar-thumb {
  background: #c0c4cc;
  border-radius: 3px;
}

.privacy-content::-webkit-scrollbar-thumb:hover {
  background: #909399;
}

/* 注册对话框样式 */
.register-container {
  padding: 32px;
}

.register-header {
  text-align: center;
  margin-bottom: 28px;
}

.register-logo {
  width: 56px;
  height: 56px;
  background: linear-gradient(135deg, #3b82f6, #8b5cf6);
  border-radius: 14px;
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 0 auto 16px;
  box-shadow: 0 4px 20px rgba(59, 130, 246, 0.4);
}

.register-logo .logo-text {
  font-size: 24px;
  font-weight: 700;
  color: #fff;
}

.register-header h2 {
  font-size: 22px;
  font-weight: 700;
  color: #ffffff;
  margin: 0 0 8px;
  background: linear-gradient(135deg, #ffffff, #60a5fa);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.register-header p {
  font-size: 13px;
  color: rgba(255, 255, 255, 0.5);
  margin: 0;
}

.register-form :deep(.el-form-item__label) {
  color: rgba(255, 255, 255, 0.7) !important;
  font-weight: 500;
}

.register-form :deep(.el-input__wrapper) {
  background: rgba(30, 41, 59, 0.8) !important;
  border: 1px solid rgba(255, 255, 255, 0.1) !important;
  box-shadow: none !important;
}

.register-form :deep(.el-input__wrapper:hover) {
  border-color: rgba(59, 130, 246, 0.5) !important;
}

.register-form :deep(.el-input__wrapper.is-focus) {
  border-color: #3b82f6 !important;
  box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.2) !important;
}

.register-form :deep(.el-input__inner) {
  color: #ffffff !important;
}

.register-form :deep(.el-input__inner::placeholder) {
  color: rgba(255, 255, 255, 0.4) !important;
}

.register-form :deep(.el-input__prefix .el-icon) {
  color: rgba(255, 255, 255, 0.4);
}

.register-btn {
  width: 100%;
  height: 44px;
  font-size: 15px;
  font-weight: 600;
  letter-spacing: 2px;
  background: linear-gradient(135deg, #3b82f6, #8b5cf6);
  border: none;
  margin-top: 8px;
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;
}

.register-btn::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
  transition: left 0.5s ease;
}

.register-btn:hover::before {
  left: 100%;
}

.register-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 10px 30px rgba(59, 130, 246, 0.4);
}

.register-footer {
  text-align: center;
  margin-top: 20px;
  font-size: 13px;
  color: rgba(255, 255, 255, 0.5);
}

.register-footer :deep(.el-button--primary.is-link) {
  color: #60a5fa;
}
</style>

<!-- 全局样式，覆盖dialog默认背景 -->
<style>
.el-dialog.register-dialog {
  background: #0f172a !important;
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 16px !important;
}

.el-dialog.register-dialog .el-dialog__header {
  display: none !important;
}

.el-dialog.register-dialog .el-dialog__body {
  padding: 0 !important;
  background: #0f172a !important;
}

.el-dialog.register-dialog .el-dialog__headerbtn {
  top: 16px;
  right: 16px;
  z-index: 10;
}

.el-dialog.register-dialog .el-dialog__headerbtn .el-dialog__close {
  color: rgba(255, 255, 255, 0.6);
}

.el-dialog.register-dialog .el-dialog__headerbtn:hover .el-dialog__close {
  color: #fff;
}
</style>
