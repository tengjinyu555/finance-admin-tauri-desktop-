import axios from 'axios'
import { ElMessage } from 'element-plus'
import router from '../router'

const baseURL = 'https://finance.52youran.top/api'

const api = axios.create({
  baseURL,
  timeout: 120000
})

// 刷新token的锁，防止并发刷新
let isRefreshing = false
let failedQueue = []
let refreshPromise = null
// 使用window对象以便在其他地方重置
if (!window.__isRedirectingToLogin) {
  window.__isRedirectingToLogin = false
}

const processQueue = (error, token = null) => {
  failedQueue.forEach(prom => {
    if (error) {
      prom.reject(error)
    } else {
      prom.resolve(token)
    }
  })
  failedQueue = []
}

// 解析JWT token获取过期时间
const getTokenExpireTime = (token) => {
  try {
    const payload = JSON.parse(atob(token.split('.')[1]))
    return payload.exp * 1000 // 转换为毫秒
  } catch (e) {
    return 0
  }
}

// 检查token是否即将过期（5分钟内）
const isTokenExpiringSoon = (token) => {
  if (!token) return true
  const expireTime = getTokenExpireTime(token)
  const now = Date.now()
  return expireTime - now < 5 * 60 * 1000 // 5分钟
}

// 刷新token
const refreshAccessToken = async () => {
  // 如果已经在刷新，复用同一个Promise
  if (refreshPromise) {
    return refreshPromise
  }

  refreshPromise = (async () => {
    try {
      const response = await axios.post(`${baseURL}/auth/refresh`, null, {
        headers: {
          'Authorization': `Bearer ${localStorage.getItem('token')}`
        }
      })
      const newToken = response.data.token
      localStorage.setItem('token', newToken)
      window.__isRedirectingToLogin = false
      return newToken
    } catch (e) {
      throw e
    } finally {
      refreshPromise = null
    }
  })()

  return refreshPromise
}

// 跳转到登录页的统一方法
const redirectToLogin = () => {
  if (window.__isRedirectingToLogin) return
  window.__isRedirectingToLogin = true
  localStorage.removeItem('user')
  localStorage.removeItem('token')
  localStorage.removeItem('tenantId')
  router.push('/login')
  ElMessage.error('登录已过期，请重新登录')
}

// 请求拦截器
api.interceptors.request.use(async config => {
  let token = localStorage.getItem('token')

  // 如果token即将过期，先刷新
  if (token && isTokenExpiringSoon(token) && !config.url.includes('/auth/')) {
    if (!isRefreshing) {
      isRefreshing = true
      try {
        token = await refreshAccessToken()
        processQueue(null, token)
      } catch (e) {
        processQueue(e, null)
        redirectToLogin()
        return Promise.reject(e)
      } finally {
        isRefreshing = false
      }
    } else {
      // 正在刷新，加入队列等待
      return new Promise((resolve, reject) => {
        failedQueue.push({ resolve, reject })
      }).then(newToken => {
        config.headers['Authorization'] = `Bearer ${newToken}`
        return config
      })
    }
  }

  if (token) {
    config.headers['Authorization'] = `Bearer ${token}`
  }
  const tenantId = localStorage.getItem('tenantId')
  if (tenantId) {
    config.headers['X-Tenant-Id'] = tenantId
  }
  return config
})

// 响应拦截器
api.interceptors.response.use(
  response => response.data,
  error => {
    if (error.response?.status === 401) {
      redirectToLogin()
    } else {
      // 找回密码/重置密码等公开接口不自动弹错误，由页面自行处理
      const skipErrorPopup = ['/auth/forgot-password', '/auth/reset-password']
      const url = error.config?.url || ''
      const shouldSkip = skipErrorPopup.some(path => url.includes(path))
      if (!shouldSkip) {
        const message = error.response?.data?.error || '请求失败'
        ElMessage.error(message)
      }
    }
    return Promise.reject(error)
  }
)

export default api
