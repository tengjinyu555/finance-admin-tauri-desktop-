<template>
  <div>
    <!-- 欢迎横幅 -->
    <div class="welcome-banner">
      <div class="welcome-left">
        <div class="welcome-avatar">{{ (userStore.user?.nickname || userStore.user?.username || '').charAt(0) }}</div>
        <div class="welcome-text">
          <div class="greeting">{{ greeting }}，{{ userStore.user?.nickname || userStore.user?.username || '用户' }}！</div>
          <div class="sub">今天是 {{ todayStr }}，您有 <strong style="color:#60a5fa;">{{ todoList.filter(t => !t.done).length }}</strong> 项待办事项需要处理</div>
        </div>
      </div>
      <div class="welcome-right">
        <div class="welcome-stat">
          <div class="num" style="color:#60a5fa;">{{ todoList.filter(t => !t.done).length }}</div>
          <div class="lbl">待办事项</div>
        </div>
        <div class="welcome-stat">
          <div class="num" style="color:#34d399;">{{ stats.ongoingCount }}</div>
          <div class="lbl">进行中项目</div>
        </div>
        <div class="welcome-stat">
          <div class="num" style="color:#fbbf24;">{{ stats.monthInvoiceCount }}</div>
          <div class="lbl">本月新增发票</div>
        </div>
      </div>
    </div>

    <!-- 统计卡片 -->
    <div class="stats-row">
      <div class="stat-card-grad bg-blue" @click="router.push('/customer')">
        <div>
          <div class="stat-val">{{ stats.customerCount }}</div>
          <div class="stat-lbl">客户总数</div>
        </div>
        <div class="stat-icn"><el-icon size="32"><OfficeBuilding /></el-icon></div>
      </div>
      <div class="stat-card-grad bg-green" @click="router.push('/project')">
        <div>
          <div class="stat-val">{{ stats.projectCount }}</div>
          <div class="stat-lbl">项目总数</div>
        </div>
        <div class="stat-icn"><el-icon size="32"><Folder /></el-icon></div>
      </div>
      <div class="stat-card-grad bg-orange" @click="router.push('/invoice')">
        <div>
          <div class="stat-val">{{ stats.invoiceCount }}</div>
          <div class="stat-lbl">发票总数</div>
        </div>
        <div class="stat-icn"><el-icon size="32"><Document /></el-icon></div>
      </div>
      <div class="stat-card-grad bg-purple">
        <div>
          <div class="stat-val">{{ stats.employeeCount }}</div>
          <div class="stat-lbl">员工人数</div>
        </div>
        <div class="stat-icn"><el-icon size="32"><User /></el-icon></div>
      </div>
    </div>

    <!-- 快捷入口 + 待办事项 -->
    <div class="dash-grid">
      <el-card class="quick-card">
        <template #header>
          <span class="card-title"><el-icon style="margin-right: 6px;"><Lightning /></el-icon>快捷入口</span>
        </template>
        <div class="quick-actions" ref="quickActionsRef">
          <div class="quick-action" v-for="(item, index) in quickActions" :key="item.id"
               :ref="el => { if (el) qaItemRefs[index] = el }"
               :class="{ 'drag-over': qaDragOverIndex === index }"
               @mousedown.prevent="onQAMouseDown($event, index)"
               @click.prevent>
            <div class="qa-icon" :style="{ background: item.color }"><el-icon size="14"><component :is="item.icon" /></el-icon></div>
            <span class="qa-label">{{ item.label }}</span>
          </div>
          <div class="quick-action add-action" @click="showQuickCustomize = true">
            <div class="qa-icon add-icon"><el-icon size="14"><Plus /></el-icon></div>
            <span class="qa-label" style="color:#999;">添加</span>
          </div>
        </div>
        <!-- 拖拽浮动元素 -->
        <div v-if="qaDragging" class="drag-ghost qa-ghost" :style="qaGhostStyle">
          <div class="quick-action" style="margin:0;">
            <div class="qa-icon" :style="{ background: quickActions[qaDragFromIndex]?.color }">
              <el-icon size="14"><component :is="quickActions[qaDragFromIndex]?.icon" /></el-icon>
            </div>
            <span class="qa-label">{{ quickActions[qaDragFromIndex]?.label }}</span>
          </div>
        </div>
      </el-card>

      <el-card class="todo-card">
        <template #header>
          <div class="flex-between">
            <span class="card-title"><el-icon style="margin-right: 6px;"><CircleCheck /></el-icon>待办事项</span>
            <div>
              <span style="font-size:12px;color:#999;margin-right:8px;">今日待办 {{ todoList.filter(t => !t.done).length }} 项</span>
              <el-button type="primary" link size="small" @click="showTodoManage = true">
                <el-icon size="12"><Edit /></el-icon> 管理
              </el-button>
            </div>
          </div>
        </template>
        <div class="todo-body">
          <div v-if="todoList.length === 0" class="empty-hint">暂无待办事项</div>
          <div v-for="(todo, i) in todoList.slice(0, 5)" :key="todo.id" :class="['todo-item', { overdue: !todo.done && isOverdue(todo) }]" :style="{ borderLeftColor: !todo.done && isOverdue(todo) ? '#f56c6c' : todoColors[i % todoColors.length] }">
            <div :class="['todo-check', { done: todo.done }]" @click="toggleTodo(todo.id)">
              <el-icon v-if="todo.done" size="12"><Check /></el-icon>
            </div>
            <div class="todo-content">
              <div :class="['todo-title', { 'done-text': todo.done }]">{{ todo.title }}</div>
              <div class="todo-meta">{{ todo.module || '通用' }} · {{ formatDateTime(todo.startDate) }} ~ {{ formatDateTime(todo.endDate) }}</div>
            </div>
            <el-tag :type="todo.priority === 'high' ? 'danger' : todo.priority === 'medium' ? 'warning' : 'success'" size="small">
              {{ todo.priority === 'high' ? '高' : todo.priority === 'medium' ? '中' : '低' }}
            </el-tag>
            <div class="todo-bar-wrap">
              <div class="todo-bar" :class="getProgressClass(todo.done ? 100 : todo.progress)" :style="{ width: Math.max(todo.done ? 100 : (todo.progress || 0), 2) + '%' }"></div>
            </div>
          </div>
        </div>
      </el-card>
    </div>

    <!-- 项目概览 + 最近操作记录 -->
    <div class="dash-grid">
      <el-card class="section-card">
        <template #header>
          <div class="flex-between">
            <span class="card-title"><el-icon style="margin-right: 6px;"><Folder /></el-icon>项目概览</span>
            <el-button type="primary" link @click="router.push('/project')">
              查看更多 <el-icon><ArrowRight /></el-icon>
            </el-button>
          </div>
        </template>
        <el-table :data="projectList" style="width: 100%" size="small">
          <el-table-column type="index" label="序号" width="50" align="center" />
          <el-table-column prop="projectName" label="项目名称" min-width="120" align="center">
            <template #default="{ row }">
              <strong>{{ row.projectName }}</strong>
            </template>
          </el-table-column>
          <el-table-column prop="manager" label="负责人" width="80" align="center" />
          <el-table-column prop="budget" label="预算" align="right" width="120">
            <template #default="{ row }">¥{{ formatMoney(row.budget) }}</template>
          </el-table-column>
          <el-table-column prop="status" label="状态" width="80" align="center">
            <template #default="{ row }">
              <el-tag :type="row.status === '进行中' ? 'primary' : row.status === '已完成' ? 'success' : 'info'" size="small">
                {{ row.status }}
              </el-tag>
            </template>
          </el-table-column>
        </el-table>
      </el-card>

      <el-card class="section-card">
        <template #header>
          <div class="flex-between">
            <span class="card-title"><el-icon style="margin-right: 6px;"><Notebook /></el-icon>最近操作记录</span>
            <el-button type="primary" link @click="router.push('/operation-log')">
              查看更多 <el-icon><ArrowRight /></el-icon>
            </el-button>
          </div>
        </template>
        <el-table :data="recentLogs" style="width: 100%" size="small">
          <el-table-column type="index" label="序号" width="50" align="center" />
          <el-table-column prop="operator" label="操作人" width="80" align="center" />
          <el-table-column prop="action" label="类型" width="80" align="center">
            <template #default="{ row }">
              <el-tag :type="getActionType(row.action)" size="small">{{ row.action }}</el-tag>
            </template>
          </el-table-column>
          <el-table-column prop="module" label="模块" width="80" align="center" />
          <el-table-column prop="content" label="内容" min-width="150" show-overflow-tooltip />
          <el-table-column prop="time" label="时间" width="140" align="center" />
        </el-table>
      </el-card>
    </div>

    <!-- 待办管理对话框 -->
    <el-dialog v-model="showTodoManage" title="待办事项管理" width="650px" :close-on-click-modal="false" class="todo-manage-dialog">
      <div class="todo-manage-header">
        <div style="display:flex;align-items:center;gap:8px;">
          <span style="font-size:13px;color:#909399;">共 {{ todoList.length }} 项，已完成 {{ todoList.filter(t => t.done).length }} 项</span>
        </div>
        <el-button type="primary" size="small" @click="addTodo"><el-icon><Plus /></el-icon> 新增待办</el-button>
      </div>
      <div class="todo-manage-body">
        <div v-if="todoList.length === 0" class="empty-hint">
          <el-icon size="48" color="#dcdfe6"><CircleCheck /></el-icon>
          <div style="margin-top:12px;">暂无待办事项</div>
          <div style="font-size:12px;color:#c0c4cc;">点击「新增待办」添加任务</div>
        </div>
        <div v-for="todo in todoList" :key="todo.id" class="todo-manage-item">
          <div :class="['todo-check', { done: todo.done }]" @click="toggleTodo(todo.id)">
            <el-icon v-if="todo.done" size="12"><Check /></el-icon>
          </div>
          <div class="todo-content">
            <div :class="['todo-title', { 'done-text': todo.done }]">{{ todo.title }}</div>
            <div class="todo-meta">
              <el-icon size="12" style="margin-right:2px;"><Document /></el-icon> {{ todo.module || '通用' }}
              <span v-if="todo.startDate" style="margin:0 4px;">·</span>
              <el-icon v-if="todo.startDate" size="12" style="margin-right:2px;"><Clock /></el-icon>
              <span v-if="todo.startDate">{{ formatDateTime(todo.startDate) }} ~ {{ formatDateTime(todo.endDate) }}</span>
              <span style="margin:0 4px;">·</span>
              进度 {{ todo.done ? 100 : (todo.progress || 0) }}%
            </div>
            <div class="todo-progress-wrap">
              <div class="todo-progress-bar">
                <div class="todo-progress-fill" :style="{ width: (todo.done ? 100 : (todo.progress || 0)) + '%', background: todo.done ? '#67c23a' : (todo.progress || 0) >= 70 ? '#409eff' : (todo.progress || 0) >= 30 ? '#e6a23c' : '#f56c6c' }"></div>
              </div>
            </div>
          </div>
          <div class="todo-manage-right">
            <el-tag :type="todo.priority === 'high' ? 'danger' : todo.priority === 'medium' ? 'warning' : 'success'" size="small">
              {{ todo.priority === 'high' ? '高' : todo.priority === 'medium' ? '中' : '低' }}
            </el-tag>
            <div class="todo-actions">
              <el-button v-if="!todo.done" size="small" text type="primary" @click="editTodo(todo)"><el-icon size="14"><Edit /></el-icon></el-button>
              <el-button v-if="!todo.done" size="small" text type="danger" @click="deleteTodo(todo.id)"><el-icon size="14"><Delete /></el-icon></el-button>
            </div>
          </div>
        </div>
      </div>
    </el-dialog>

    <!-- 快捷入口自定义对话框 -->
    <el-dialog v-model="showQuickCustomize" title="自定义快捷入口" width="500px" :close-on-click-modal="false">
      <p style="font-size:13px;color:#999;margin-bottom:12px;">勾选显示的快捷入口，拖拽 ☰ 调整顺序。</p>
      <div v-for="(item, index) in getAvailableQuickActions()" :key="item.id"
           :ref="el => { if (el) qcItemRefs[index] = el }"
           :class="['qc-item', { checked: customActionIds.includes(item.id), 'drag-over': dragOverIndex === index }]"
           @click="toggleQuickCheck(item.id)">
        <div class="qc-left">
          <div class="qc-icon" :style="{ background: item.color }"><el-icon size="14"><component :is="item.icon" /></el-icon></div>
          <div class="qc-label">{{ item.label }}</div>
        </div>
        <div class="qc-right" @click.stop>
          <span class="drag-handle" @mousedown.prevent="onDragHandleMouseDown($event, index)">☰</span>
          <el-checkbox :model-value="customActionIds.includes(item.id)" />
        </div>
      </div>
      <!-- 拖拽时的浮动元素 -->
      <div v-if="dragging" class="drag-ghost" :style="ghostStyle">
        <div class="qc-item checked" style="margin:0;">
          <div class="qc-left">
            <div class="qc-icon" :style="{ background: getAvailableQuickActions()[dragFromIndex]?.color }">
              <el-icon size="14"><component :is="getAvailableQuickActions()[dragFromIndex]?.icon" /></el-icon>
            </div>
            <div class="qc-label">{{ getAvailableQuickActions()[dragFromIndex]?.label }}</div>
          </div>
        </div>
      </div>
      <template #footer>
        <el-button @click="showQuickCustomize = false">取消</el-button>
        <el-button type="primary" @click="saveQuickActions">保存</el-button>
      </template>
    </el-dialog>

    <!-- 新增/编辑待办对话框 -->
    <el-dialog v-model="showTodoForm" :title="editingTodo ? '编辑待办' : '新增待办'" width="520px" :close-on-click-modal="false" class="todo-form-dialog">
      <el-form :model="todoForm" label-width="80px" label-position="top">
        <el-form-item label="待办内容" required>
          <el-input v-model="todoForm.title" placeholder="请输入待办事项内容" />
        </el-form-item>
        <el-row :gutter="16">
          <el-col :span="12">
            <el-form-item label="所属模块">
              <el-autocomplete v-model="todoForm.module" :fetch-suggestions="queryModule" placeholder="输入或选择模块" style="width:100%;" :maxlength="4" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="优先级">
              <el-select v-model="todoForm.priority" style="width:100%;">
                <el-option label="高" value="high" />
                <el-option label="中" value="medium" />
                <el-option label="低" value="low" />
              </el-select>
            </el-form-item>
          </el-col>
        </el-row>
        <el-row :gutter="16">
          <el-col :span="12">
            <el-form-item label="开始时间">
              <el-date-picker v-model="todoForm.startDate" type="datetime" placeholder="选择开始时间" format="YYYY-MM-DD HH:mm" value-format="YYYY-MM-DD HH:mm" style="width:100%;" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="截止时间">
              <el-date-picker v-model="todoForm.endDate" type="datetime" placeholder="选择截止时间" format="YYYY-MM-DD HH:mm" value-format="YYYY-MM-DD HH:mm" style="width:100%;" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-form-item v-if="todoForm.startDate && todoForm.endDate">
          <div style="display:flex;align-items:center;gap:12px;width:100%;">
            <span style="font-size:13px;color:#909399;">自动进度：</span>
            <el-progress :percentage="calcAutoProgress()" :status="calcAutoProgress() >= 100 ? 'success' : ''" style="flex:1;" />
          </div>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showTodoForm = false">取消</el-button>
        <el-button type="primary" @click="saveTodo">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, reactive, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useUserStore } from '../stores/user'
import {
  OfficeBuilding, Folder, Document, ArrowRight, User, Plus, Edit, Delete,
  Check, CircleCheck, Lightning, Notebook, Filter, Monitor, List, Clock
} from '@element-plus/icons-vue'
import api from '../api/index'
import { InvoiceApi } from '../api/invoice'

const router = useRouter()
const userStore = useUserStore()

// 欢迎语
const now = new Date()
const hour = now.getHours()
const greeting = hour < 12 ? '上午好' : hour < 18 ? '下午好' : '晚上好'
const todayStr = now.getFullYear() + '年' + (now.getMonth() + 1) + '月' + now.getDate() + '日'

// 统计
const stats = reactive({
  customerCount: 0,
  projectCount: 0,
  invoiceCount: 0,
  employeeCount: 0,
  ongoingCount: 0,
  monthInvoiceCount: 0
})

const projectList = ref([])
const recentLogs = ref([])

// 快捷入口定义（含权限控制）
const allQuickActionDefs = ref([
  { id: 'customer', label: '客户管理', icon: 'OfficeBuilding', color: 'linear-gradient(135deg,#409eff,#66b1ff)', path: '/customer', roles: ['admin', 'finance', 'viewer'] },
  { id: 'project', label: '项目管理', icon: 'Folder', color: 'linear-gradient(135deg,#67c23a,#85ce61)', path: '/project', roles: ['admin', 'finance', 'viewer'] },
  { id: 'invoice', label: '发票台账', icon: 'Document', color: 'linear-gradient(135deg,#e6a23c,#ebb563)', path: '/invoice', roles: ['admin', 'finance', 'viewer'] },
  { id: 'ocr', label: 'OCR导入', icon: 'Monitor', color: 'linear-gradient(135deg,#9b59b6,#bb86fc)', path: '/ocr-import', roles: ['admin', 'finance'] },
  { id: 'transaction', label: '收支流水', icon: 'List', color: 'linear-gradient(135deg,#f56c6c,#f89898)', path: '/transaction', roles: ['admin', 'finance', 'viewer'] },
  { id: 'employee', label: '员工管理', icon: 'User', color: 'linear-gradient(135deg,#00bcd4,#4dd0e1)', path: '/employee', roles: ['admin'] },
  { id: 'log', label: '操作日志', icon: 'Notebook', color: 'linear-gradient(135deg,#607d8b,#90a4ae)', path: '/operation-log', roles: ['admin', 'finance', 'viewer'] }
])

// 获取用户有权限的快捷入口
const getAvailableQuickActions = () => {
  const userRoles = userStore.user?.roleCodes || userStore.user?.roles || []
  return allQuickActionDefs.value.filter(action => {
    return action.roles.some(role => userRoles.includes(role))
  })
}
const quickActions = ref([])
const showQuickCustomize = ref(false)
const customActionIds = ref([])

const loadQuickActions = async () => {
  try {
    const available = getAvailableQuickActions()
    const data = await api.get('/quick-actions')
    if (data && data.length > 0) {
      customActionIds.value = data.map(a => a.actionId)
      // 过滤掉用户无权限的
      customActionIds.value = customActionIds.value.filter(id => available.some(a => a.id === id))
      quickActions.value = customActionIds.value.map(id => available.find(d => d.id === id)).filter(Boolean)
    } else {
      customActionIds.value = available.slice(0, 5).map(a => a.id)
      quickActions.value = available.slice(0, 5)
    }
  } catch (e) {
    quickActions.value = getAvailableQuickActions().slice(0, 5)
  }
}

const toggleQuickCheck = (id) => {
  // 检查是否有权限
  const available = getAvailableQuickActions()
  if (!available.some(a => a.id === id)) return

  const index = customActionIds.value.indexOf(id)
  if (index > -1) {
    customActionIds.value.splice(index, 1)
  } else {
    customActionIds.value.push(id)
  }
}

// 鼠标拖拽排序
const qcItemRefs = ref({})
const dragging = ref(false)
const dragFromIndex = ref(-1)
const dragOverIndex = ref(-1)
const ghostStyle = ref({})
let dragStartY = 0
let dragOffsetY = 0

const onDragHandleMouseDown = (e, index) => {
  dragging.value = true
  dragFromIndex.value = index
  dragOverIndex.value = index
  dragStartY = e.clientY
  const el = qcItemRefs.value[index]
  if (el) {
    const rect = el.getBoundingClientRect()
    dragOffsetY = e.clientY - rect.top
    ghostStyle.value = {
      position: 'fixed',
      left: rect.left + 'px',
      top: (e.clientY - dragOffsetY) + 'px',
      width: rect.width + 'px',
      zIndex: 9999,
      opacity: 0.9,
      pointerEvents: 'none'
    }
  }
  document.addEventListener('mousemove', onDragMouseMove)
  document.addEventListener('mouseup', onDragMouseUp)
}

const onDragMouseMove = (e) => {
  if (!dragging.value) return
  ghostStyle.value = {
    ...ghostStyle.value,
    top: (e.clientY - dragOffsetY) + 'px'
  }
  // 判断当前鼠标在哪个元素上方
  const available = getAvailableQuickActions()
  for (let i = 0; i < available.length; i++) {
    const el = qcItemRefs.value[i]
    if (!el) continue
    const rect = el.getBoundingClientRect()
    if (e.clientY >= rect.top && e.clientY <= rect.bottom) {
      dragOverIndex.value = i
      break
    }
  }
}

const onDragMouseUp = () => {
  document.removeEventListener('mousemove', onDragMouseMove)
  document.removeEventListener('mouseup', onDragMouseUp)
  if (dragging.value && dragFromIndex.value !== -1 && dragOverIndex.value !== -1 && dragFromIndex.value !== dragOverIndex.value) {
    // 重新排列 customActionIds
    const available = getAvailableQuickActions()
    const tempId = customActionIds.value[dragFromIndex.value]
    customActionIds.value.splice(dragFromIndex.value, 1)
    customActionIds.value.splice(dragOverIndex.value, 0, tempId)
    customActionIds.value = [...customActionIds.value]
  }
  dragging.value = false
  dragFromIndex.value = -1
  dragOverIndex.value = -1
}

const saveQuickActions = async () => {
  try {
    const actions = customActionIds.value.map((id, i) => ({ actionId: id, sortOrder: i }))
    await api.post('/quick-actions', actions)
    await loadQuickActions()
    showQuickCustomize.value = false
  } catch (e) {
    console.error(e)
  }
}

// 工作台快捷入口拖拽排序（长按触发）
const quickActionsRef = ref(null)
const qaItemRefs = ref({})
const qaDragging = ref(false)
const qaDragFromIndex = ref(-1)
const qaDragOverIndex = ref(-1)
const qaGhostStyle = ref({})
let qaDragOffsetY = 0
let qaLongPressTimer = null
let qaMouseDownPos = { x: 0, y: 0 }
let qaClickItem = null

const onQAMouseDown = (e, index) => {
  if (e.button !== 0) return
  qaMouseDownPos = { x: e.clientX, y: e.clientY }
  qaClickItem = quickActions.value[index]

  // 长按 300ms 触发拖拽
  qaLongPressTimer = setTimeout(() => {
    const el = qaItemRefs.value[index]
    if (!el) return
    qaDragging.value = true
    qaDragFromIndex.value = index
    qaDragOverIndex.value = index
    const rect = el.getBoundingClientRect()
    qaDragOffsetY = e.clientY - rect.top
    qaGhostStyle.value = {
      position: 'fixed',
      left: rect.left + 'px',
      top: (e.clientY - qaDragOffsetY) + 'px',
      width: rect.width + 'px',
      zIndex: 9999,
      opacity: 0.9,
      pointerEvents: 'none'
    }
    document.addEventListener('mousemove', onQADragMouseMove)
    document.addEventListener('mouseup', onQADragMouseUp)
  }, 300)

  document.addEventListener('mouseup', onQAMouseUp)
}

const onQADragMouseMove = (e) => {
  if (!qaDragging.value) return
  qaGhostStyle.value = {
    ...qaGhostStyle.value,
    left: (e.clientX - 30) + 'px',
    top: (e.clientY - qaDragOffsetY) + 'px'
  }
  // 判断当前鼠标在哪个元素上方
  for (let i = 0; i < quickActions.value.length; i++) {
    const el = qaItemRefs.value[i]
    if (!el) continue
    const rect = el.getBoundingClientRect()
    if (e.clientX >= rect.left && e.clientX <= rect.right &&
        e.clientY >= rect.top && e.clientY <= rect.bottom) {
      qaDragOverIndex.value = i
      break
    }
  }
}

// 短点击跳转
const onQAMouseUp = (e) => {
  clearTimeout(qaLongPressTimer)
  document.removeEventListener('mouseup', onQAMouseUp)
  // 如果没有进入拖拽模式，且移动距离很小，视为点击
  if (!qaDragging.value && qaClickItem) {
    const dx = Math.abs(e.clientX - qaMouseDownPos.x)
    const dy = Math.abs(e.clientY - qaMouseDownPos.y)
    if (dx < 5 && dy < 5) {
      router.push(qaClickItem.path)
    }
  }
  qaClickItem = null
}

const onQADragMouseUp = () => {
  document.removeEventListener('mousemove', onQADragMouseMove)
  document.removeEventListener('mouseup', onQADragMouseUp)
  if (qaDragging.value && qaDragFromIndex.value !== -1 && qaDragOverIndex.value !== -1 && qaDragFromIndex.value !== qaDragOverIndex.value) {
    const temp = quickActions.value[qaDragFromIndex.value]
    quickActions.value.splice(qaDragFromIndex.value, 1)
    quickActions.value.splice(qaDragOverIndex.value, 0, temp)
    quickActions.value = [...quickActions.value]
    // 保存新顺序到后端
    saveQuickActionsOrder()
  }
  qaDragging.value = false
  qaDragFromIndex.value = -1
  qaDragOverIndex.value = -1
}

const saveQuickActionsOrder = async () => {
  try {
    const actions = quickActions.value.map((item, i) => ({ actionId: item.id, sortOrder: i }))
    await api.post('/quick-actions', actions)
  } catch (e) {
    console.error(e)
  }
}

// 待办事项
const todoColors = ['#409eff', '#67c23a', '#e6a23c', '#9b59b6', '#f56c6c']
const moduleOptions = ['客户', '项目', '发票', '流水', '员工', '登录', '回款', '支出']

const queryModule = (queryString, cb) => {
  const results = queryString
    ? moduleOptions.filter(m => m.includes(queryString))
    : moduleOptions
  cb(results.map(m => ({ value: m })))
}
const showTodoManage = ref(false)
const showTodoForm = ref(false)
const editingTodo = ref(null)
const todoForm = reactive({ title: '', module: '', priority: 'medium', startDate: '', endDate: '' })

const calcAutoProgress = () => {
  if (!todoForm.startDate || !todoForm.endDate) return 0
  const start = new Date(todoForm.startDate).getTime()
  const end = new Date(todoForm.endDate).getTime()
  const now = Date.now()
  if (now >= end) return 100
  if (now <= start) return 0
  return Math.round(((now - start) / (end - start)) * 100)
}

const todoList = ref([])

const loadTodos = async () => {
  try {
    const todos = await api.get('/todos')
    todoList.value = (todos || []).sort((a, b) => {
      if (a.done === b.done) return 0
      return a.done ? 1 : -1
    })
  } catch (e) {
    console.error(e)
  }
}

const toggleTodo = async (id) => {
  const t = todoList.value.find(x => x.id === id)
  if (!t) return
  t.done = !t.done
  try {
    await api.put(`/todos/${id}`, t)
  } catch (e) {
    console.error(e)
  }
}

const addTodo = () => {
  editingTodo.value = null
  const now = new Date()
  const pad = (n) => String(n).padStart(2, '0')
  const currentTime = `${now.getFullYear()}-${pad(now.getMonth() + 1)}-${pad(now.getDate())} ${pad(now.getHours())}:${pad(now.getMinutes())}`
  Object.assign(todoForm, { title: '', module: '', priority: 'medium', startDate: currentTime, endDate: currentTime })
  showTodoForm.value = true
}

const editTodo = (todo) => {
  editingTodo.value = todo
  Object.assign(todoForm, { title: todo.title, module: todo.module, priority: todo.priority, startDate: todo.startDate || '', endDate: todo.endDate || '' })
  showTodoForm.value = true
  showTodoManage.value = false
}

const saveTodo = async () => {
  if (!todoForm.title) return
  try {
    const data = {
      title: todoForm.title,
      module: todoForm.module,
      priority: todoForm.priority,
      startDate: todoForm.startDate,
      endDate: todoForm.endDate,
      progress: calcAutoProgress()
    }
    if (editingTodo.value) {
      await api.put(`/todos/${editingTodo.value.id}`, { ...editingTodo.value, ...data })
    } else {
      await api.post('/todos', { ...data, done: false })
    }
    await loadTodos()
    showTodoForm.value = false
    showTodoManage.value = true
  } catch (e) {
    console.error(e)
  }
}

const deleteTodo = async (id) => {
  try {
    await api.delete(`/todos/${id}`)
    await loadTodos()
  } catch (e) {
    console.error(e)
  }
}

const formatMoney = (num) => {
  return Number(num || 0).toLocaleString('zh-CN', { minimumFractionDigits: 2 })
}

const getActionType = (action) => {
  const types = { '新增': 'success', '编辑': '', '删除': 'danger', '导入': 'warning', '登录': 'info' }
  return types[action] || 'info'
}

const getActionColor = (action) => {
  const colors = { '新增': '#67c23a', '编辑': '#409eff', '删除': '#f56c6c', '导入': '#e6a23c', '登录': '#909399' }
  return colors[action] || '#909399'
}

const formatDateTime = (str) => {
  if (!str) return ''
  return str.slice(5, 16).replace(' ', ' ')
}

const getProgressClass = (pct) => {
  if (pct >= 100) return 'success'
  if (pct >= 70) return 'primary'
  if (pct >= 30) return 'warning'
  return 'danger'
}

const isOverdue = (todo) => {
  if (!todo.endDate) return false
  return new Date(todo.endDate) < new Date()
}

const loadData = async () => {
  try {
    const customers = await api.get('/suppliers')
    stats.customerCount = (customers || []).length

    const projects = await api.get('/projects')
    stats.projectCount = (projects || []).length
    stats.ongoingCount = (projects || []).filter(p => p.status === '进行中').length
    projectList.value = (projects || []).slice(0, 5)

    const invoices = await InvoiceApi.list()
    stats.invoiceCount = (invoices || []).length
    // 本月发票数
    const thisMonth = (now.getFullYear()) + '-' + String(now.getMonth() + 1).padStart(2, '0')
    stats.monthInvoiceCount = (invoices || []).filter(i => i.date && i.date.startsWith(thisMonth)).length

    const users = await api.get('/users')
    stats.employeeCount = (users || []).length
  } catch (e) {
    console.error(e)
  }
}

const loadRecentLogs = async () => {
  try {
    const logs = await api.get('/operation-logs')
    recentLogs.value = (logs || []).slice(0, 5).map(log => ({
      operator: log.operator,
      action: log.action,
      module: log.module,
      content: log.content,
      time: log.operationTime ? log.operationTime.replace('T', ' ').slice(0, 16) : ''
    }))
  } catch (e) {
    console.error(e)
  }
}

onMounted(async () => {
  await loadQuickActions()
  loadTodos()
  await loadData()
  await loadRecentLogs()
})
</script>

<style scoped>
/* 字体优化 */
:deep(.el-table) {
  font-family: -apple-system, BlinkMacSystemFont, 'PingFang SC', 'Microsoft YaHei', sans-serif;
}
:deep(.el-table th) {
  font-weight: 600;
  color: #303133;
}
:deep(.el-table td) {
  color: #606266;
  font-size: 13px;
}
:deep(.el-table td strong) {
  font-weight: 600;
}
:deep(.el-tag) {
  font-weight: 500;
}
:deep(.el-card) {
  font-family: -apple-system, BlinkMacSystemFont, 'PingFang SC', 'Microsoft YaHei', sans-serif;
}
:deep(.el-card__header) {
  font-weight: 600;
}

/* 欢迎横幅 */
.welcome-banner {
  background: linear-gradient(135deg, #1a1a4e 0%, #2d1b69 50%, #1a1a4e 100%);
  border-radius: 12px;
  padding: 28px 32px;
  color: #fff;
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  position: relative;
  overflow: hidden;
}
.welcome-banner::before {
  content: '';
  position: absolute;
  top: -50%;
  right: -10%;
  width: 250px;
  height: 250px;
  border-radius: 50%;
  background: rgba(255,255,255,0.03);
  pointer-events: none;
}
.welcome-banner::after {
  content: '';
  position: absolute;
  bottom: -30%;
  right: 20%;
  width: 180px;
  height: 180px;
  border-radius: 50%;
  background: rgba(255,255,255,0.02);
  pointer-events: none;
}
.welcome-left {
  display: flex;
  align-items: center;
  gap: 20px;
  position: relative;
  z-index: 1;
}
.welcome-avatar {
  width: 56px;
  height: 56px;
  border-radius: 50%;
  background: linear-gradient(135deg, #409eff, #7c3aed);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 24px;
  font-weight: 700;
  color: #fff;
  box-shadow: 0 4px 16px rgba(64,158,255,0.3);
}
.welcome-text .greeting {
  font-size: 22px;
  font-weight: 700;
}
.welcome-text .sub {
  font-size: 13px;
  color: rgba(255,255,255,0.65);
  margin-top: 4px;
}
.welcome-right {
  display: flex;
  gap: 32px;
  position: relative;
  z-index: 1;
}
.welcome-stat {
  text-align: center;
}
.welcome-stat .num {
  font-size: 28px;
  font-weight: 700;
}
.welcome-stat .lbl {
  font-size: 12px;
  color: rgba(255,255,255,0.55);
  margin-top: 2px;
}

/* 统计卡片 */
.stats-row {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
  margin-bottom: 16px;
}
.stat-card-grad {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-radius: 8px;
  color: #fff;
  cursor: pointer;
  transition: transform 0.2s, box-shadow 0.2s;
}
.stat-card-grad:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 24px rgba(0,0,0,0.15);
}
.stat-card-grad .stat-val {
  font-size: 26px;
  font-weight: 700;
  line-height: 1.2;
}
.stat-card-grad .stat-lbl {
  font-size: 13px;
  opacity: 0.9;
  margin-top: 2px;
}
.stat-card-grad .stat-icn {
  opacity: 0.8;
}
.bg-blue { background: linear-gradient(135deg, #409eff 0%, #66b1ff 100%); }
.bg-green { background: linear-gradient(135deg, #67c23a 0%, #85ce61 100%); }
.bg-orange { background: linear-gradient(135deg, #e6a23c 0%, #ebb563 100%); }
.bg-purple { background: linear-gradient(135deg, #9b59b6 0%, #bb86fc 100%); }

/* 双列布局 */
.dash-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
  margin-bottom: 16px;
}

/* 快捷入口 */
.quick-actions {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}
.quick-action {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 4px;
  padding: 10px 14px;
  background: #fff;
  border-radius: 8px;
  border: 1px solid #f0f0f0;
  cursor: pointer;
  min-height: 66px;
  min-width: 66px;
  transition: all 0.2s;
}
.quick-action:hover {
  transform: translateY(-1px);
  box-shadow: 0 3px 8px rgba(0,0,0,0.08);
  border-color: #409eff;
}
.quick-action.drag-over {
  border-color: #409eff;
  background: #ecf5ff;
}
.quick-action {
  position: relative;
  user-select: none;
}
.qa-icon {
  width: 26px;
  height: 26px;
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
}
.add-icon {
  background: #f0f2f5 !important;
  border: 1px dashed #ccc;
}
.add-icon .el-icon {
  color: #999;
}
.qa-label {
  font-size: 11px;
  font-weight: 500;
  color: #333;
}

/* 待办事项 */
.todo-body {
  min-height: 60px;
  max-height: 188px;
  overflow-y: auto;
}
.todo-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 0 16px 10px;
  border-bottom: 1px solid #f0f0f0;
  border-left: 3px solid transparent;
  margin-bottom: 2px;
  position: relative;
}
.todo-item:last-child {
  border-bottom: none;
}
.todo-item.overdue {
  background: #fef0f0;
  border-radius: 6px;
}
.todo-item.overdue .todo-title {
  color: #f56c6c;
  font-weight: 600;
}
.todo-bar-wrap {
  position: absolute;
  bottom: -1px;
  left: 0;
  right: 0;
  height: 4px;
  background: #f0f0f0;
  border-radius: 2px;
  overflow: hidden;
  margin: 0 4px;
}
.todo-bar {
  height: 100%;
  border-radius: 2px;
  transition: width 0.5s;
}
.todo-bar.danger { background: #f56c6c; }
.todo-bar.warning { background: #e6a23c; }
.todo-bar.success { background: #67c23a; }
.todo-bar.primary { background: #409eff; }
.todo-check {
  width: 20px;
  height: 20px;
  border-radius: 50%;
  border: 2px solid #dcdfe6;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
  flex-shrink: 0;
}
.todo-check:hover {
  border-color: #409eff;
}
.todo-check.done {
  background: #67c23a;
  border-color: #67c23a;
}
.todo-check .el-icon {
  color: #fff;
}
.todo-content {
  flex: 1;
  font-size: 14px;
  min-width: 0;
}
.todo-title {
  color: #333;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.todo-title.done-text {
  text-decoration: line-through;
  color: #ccc;
}
.todo-meta {
  font-size: 12px;
  color: #999;
  margin-top: 2px;
}
.empty-hint {
  text-align: center;
  padding: 20px;
  color: #ccc;
  font-size: 13px;
}

/* 待办管理 */
.todo-form-dialog :deep(.el-dialog__header) {
  border-bottom: 1px solid #ebeef5;
  padding-bottom: 12px;
}
.todo-form-dialog :deep(.el-dialog__body) {
  padding: 20px 24px;
}
.todo-manage-dialog :deep(.el-dialog__header) {
  border-bottom: 1px solid #ebeef5;
  padding-bottom: 12px;
}
.todo-manage-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
  padding-bottom: 12px;
  border-bottom: 1px solid #f5f5f5;
}
.todo-manage-body {
  max-height: 450px;
  overflow-y: auto;
}
.todo-manage-body::-webkit-scrollbar {
  width: 4px;
}
.todo-manage-body::-webkit-scrollbar-thumb {
  background: #dcdfe6;
  border-radius: 2px;
}
.todo-manage-item {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  padding: 14px 12px;
  border-bottom: 1px solid #f5f5f5;
  border-radius: 8px;
  margin-bottom: 4px;
  transition: background 0.2s;
}
.todo-manage-item:hover {
  background: #f8f9fa;
}
.todo-manage-item:last-child {
  border-bottom: none;
}
.todo-manage-right {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 8px;
  flex-shrink: 0;
}
.todo-actions {
  display: flex;
  gap: 2px;
}
.todo-progress-wrap {
  margin-top: 6px;
}
.todo-progress-bar {
  height: 4px;
  background: #f0f0f0;
  border-radius: 2px;
  overflow: hidden;
  width: 120px;
}
.todo-progress-fill {
  height: 100%;
  border-radius: 2px;
  transition: width 0.3s;
}

/* 操作记录 */
.log-list {
  max-height: 300px;
  overflow-y: auto;
}
.log-item {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  padding: 12px 0;
  border-bottom: 1px solid #f5f5f5;
}
.log-item:last-child {
  border-bottom: none;
}
.log-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  margin-top: 6px;
  flex-shrink: 0;
}
.log-content {
  flex: 1;
  min-width: 0;
}
.log-main {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
}
.log-operator {
  font-size: 14px;
  font-weight: 600;
  color: #1a1a2e;
  letter-spacing: 0.3px;
}
.log-module {
  font-size: 13px;
  color: #606266;
  font-weight: 500;
}
.log-meta {
  font-size: 12px;
  color: #909399;
  margin-top: 4px;
  letter-spacing: 0.2px;
}

/* 快捷入口自定义 */
.qc-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 12px;
  border: 1px solid #ebeef5;
  border-radius: 6px;
  margin-bottom: 8px;
  background: #fff;
  cursor: pointer;
  transition: all 0.2s;
}
.qc-item:hover {
  border-color: #409eff;
}
.qc-item[draggable="true"]:active {
  cursor: grabbing;
  opacity: 0.8;
  background: #f5f7fa;
}
.qc-item.drag-over {
  border-color: #409eff;
  background: #ecf5ff;
}
.drag-handle {
  cursor: grab;
  font-size: 18px;
  color: #c0c4cc;
  padding: 4px;
  user-select: none;
}
.drag-handle:active {
  cursor: grabbing;
}
.qc-item.drag-over {
  border-color: #409eff;
  background: #ecf5ff;
  transform: translateY(2px);
}
.drag-ghost .qc-item {
  border: 2px solid #409eff;
  background: #ecf5ff;
  box-shadow: 0 8px 24px rgba(0,0,0,0.15);
}
.qc-item.checked {
  background: #ecf5ff;
  border-color: #409eff;
}
.qc-left {
  display: flex;
  align-items: center;
  gap: 12px;
}
.qc-icon {
  width: 32px;
  height: 32px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
  flex-shrink: 0;
}
.qc-label {
  font-size: 14px;
  font-weight: 500;
}
.qc-desc {
  font-size: 12px;
  color: #999;
}
.qc-right {
  display: flex;
  align-items: center;
  gap: 4px;
}

/* 通用 */
.card-title {
  font-size: 15px;
  font-weight: 600;
  color: #303133;
  display: flex;
  align-items: center;
}
.flex-between {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.section-card :deep(.el-card__header) {
  padding: 14px 20px;
  background: #fafafa;
  border-bottom: 1px solid #f0f0f0;
}
.quick-card :deep(.el-card__header) {
  padding: 14px 20px;
  background: #fafafa;
  border-bottom: 1px solid #f0f0f0;
}
.todo-card :deep(.el-card__header) {
  padding: 14px 20px;
  background: #fafafa;
  border-bottom: 1px solid #f0f0f0;
}
</style>
