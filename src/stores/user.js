import { defineStore } from 'pinia'
import { ref } from 'vue'
import { AuthApi } from '../api/invoice'

export const useUserStore = defineStore('user', () => {
  const user = ref(JSON.parse(localStorage.getItem('user') || 'null'))
  const token = ref(localStorage.getItem('token') || '')
  const tenantId = ref(localStorage.getItem('tenantId') || '')
  const myTaxNo = ref(user.value?.taxNo || '')

  async function login(username, password, tenantIdValue) {
    const data = await AuthApi.login({ username, password }, tenantIdValue)
    user.value = data
    token.value = data.token
    tenantId.value = data.tenantId || tenantIdValue
    myTaxNo.value = data.taxNo || ''
    localStorage.setItem('user', JSON.stringify(data))
    localStorage.setItem('token', data.token)
    localStorage.setItem('tenantId', data.tenantId || tenantIdValue)
    // 登录成功后重置跳转标志
    window.__isRedirectingToLogin = false
    return data
  }

  function logout() {
    user.value = null
    token.value = ''
    tenantId.value = ''
    localStorage.removeItem('user')
    localStorage.removeItem('token')
    localStorage.removeItem('tenantId')
  }

  function setMyTaxNo(taxNo) {
    myTaxNo.value = taxNo
  }

  return { user, token, tenantId, myTaxNo, login, logout, setMyTaxNo }
})
