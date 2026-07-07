import { createRouter, createWebHashHistory } from 'vue-router'

const routes = [
  {
    path: '/login',
    name: 'Login',
    component: () => import('../views/Login.vue')
  },
  {
    path: '/',
    component: () => import('../views/Layout.vue'),
    redirect: '/dashboard',
    children: [
      {
        path: 'dashboard',
        name: 'Dashboard',
        component: () => import('../views/Dashboard.vue'),
        meta: { title: '工作台' }
      },
      {
        path: 'customer',
        name: 'Customer',
        component: () => import('../views/Customer.vue'),
        meta: { title: '客户管理' }
      },
      {
        path: 'project',
        name: 'Project',
        component: () => import('../views/Project.vue'),
        meta: { title: '项目管理' }
      },
      {
        path: 'invoice',
        name: 'Invoice',
        component: () => import('../views/Invoice.vue'),
        meta: { title: '发票台账' }
      },
      {
        path: 'transaction',
        name: 'Transaction',
        component: () => import('../views/Transaction.vue'),
        meta: { title: '收支流水' }
      },
      {
        path: 'ocr-import',
        name: 'OcrImport',
        component: () => import('../views/OcrImport.vue'),
        meta: { title: 'OCR识别导入' }
      },
      {
        path: 'employee',
        name: 'Employee',
        component: () => import('../views/Employee.vue'),
        meta: { title: '员工管理', requireAdmin: true }
      },
      {
        path: 'operation-log',
        name: 'OperationLog',
        component: () => import('../views/OperationLog.vue'),
        meta: { title: '操作日志' }
      }
    ]
  }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

// 路由守卫
router.beforeEach((to, from, next) => {
  const user = localStorage.getItem('user')
  const token = localStorage.getItem('token')

  if (to.path !== '/login' && (!user || !token)) {
    next('/login')
  } else {
    next()
  }
})

export default router
