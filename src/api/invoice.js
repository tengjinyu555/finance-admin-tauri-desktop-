import api from './index'

// 统一发票 API
export const InvoiceApi = {
  list: (params) => api.get('/invoices', { params }),
  get: (id) => api.get(`/invoices/${id}`),
  create: (data) => api.post('/invoices', data),
  update: (id, data) => api.put(`/invoices/${id}`, data),
  delete: (id) => api.delete(`/invoices/${id}`),
  stats: () => api.get('/invoices/stats')
}

// 客户 API
export const CustomerApi = {
  list: () => api.get('/suppliers'),
  get: (id) => api.get(`/suppliers/${id}`),
  create: (data) => api.post('/suppliers', data),
  update: (id, data) => api.put(`/suppliers/${id}`, data),
  delete: (id) => api.delete(`/suppliers/${id}`)
}

// OCR API
export const OcrApi = {
  recognize: (url) => api.post('/ocr/recognize', { url })
}

// 文件上传 API
export const FileApi = {
  upload: (file) => {
    const formData = new FormData()
    formData.append('file', file)
    return api.post('/file/upload', formData, {
      headers: { 'Content-Type': 'multipart/form-data' }
    })
  }
}

// 仪表盘 API
export const DashboardApi = {
  summary: () => api.get('/dashboard/summary')
}

// 租户 API
export const TenantApi = {
  search: (keyword) => api.get('/tenants/search', { params: { keyword } }),
  get: (id) => api.get(`/tenants/${id}`)
}

// 认证 API
export const AuthApi = {
  login: (data, tenantId) => api.post('/auth/login', data, { headers: { 'X-Tenant-Id': tenantId } }),
  register: (data, tenantId) => api.post('/auth/register', data, { headers: { 'X-Tenant-Id': tenantId } }),
  refresh: () => api.post('/auth/refresh'),
  verifyPassword: (data, tenantId) => api.post('/auth/verify-password', data, { headers: { 'X-Tenant-Id': tenantId } })
}

// 用户管理 API
export const UserApi = {
  list: () => api.get('/users'),
  get: (id) => api.get(`/users/${id}`),
  create: (data) => api.post('/users', data),
  updateStatus: (id, status) => api.put(`/users/${id}/status`, { status }),
  delete: (id) => api.delete(`/users/${id}`)
}
