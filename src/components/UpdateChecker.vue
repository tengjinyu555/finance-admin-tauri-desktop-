<template>
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
</template>

<script setup>
import { reactive } from 'vue'
import { listen, emit } from '@tauri-apps/api/event'
import { CircleCheck, CircleClose, Download, Loading } from '@element-plus/icons-vue'

const updateState = reactive({
  show: false,
  status: 'available',
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

listen('update-available', (event) => {
  console.log('[UpdateChecker] 收到新版本通知:', event.payload)
  updateState.version = event.payload
  updateState.status = 'available'
  updateState.show = true
})

listen('update-progress', (event) => {
  console.log('[UpdateChecker] 下载进度:', event.payload)
  updateState.loaded = event.payload.loaded
  updateState.total = event.payload.total
})

listen('update-complete', () => {
  console.log('[UpdateChecker] 下载完成')
  updateState.status = 'complete'
})

listen('update-error', (event) => {
  console.log('[UpdateChecker] 下载错误:', event.payload)
  updateState.status = 'error'
  updateState.errorMsg = event.payload
})
</script>

<style scoped>
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
</style>
