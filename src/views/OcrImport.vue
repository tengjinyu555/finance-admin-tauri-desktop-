<template>
  <div>
    <el-card>
      <template #header>
        <span><el-icon><Monitor /></el-icon> 发票OCR识别导入</span>
      </template>
      <el-upload
        class="upload-area"
        drag
        :auto-upload="false"
        :on-change="handleFileChange"
        :file-list="fileList"
        accept=".jpg,.jpeg,.png,.bmp,.pdf"
        multiple
        :show-file-list="false"
      >
        <el-icon class="el-icon--upload" size="48"><UploadFilled /></el-icon>
        <div class="el-upload__text">将文件拖到此处，或<em>点击上传</em></div>
        <template #tip>
          <div class="el-upload__tip">支持 JPG / PNG / PDF 格式的增值税专用发票、普通发票</div>
        </template>
      </el-upload>

      <!-- 自定义文件列表 -->
      <div v-if="fileList.length > 0" class="file-list-wrapper">
        <div class="file-list-header">
          <span>已选择文件 ({{ fileList.length }})</span>
          <el-button type="danger" text size="small" @click="fileList = []">清空</el-button>
        </div>
        <div class="file-grid">
          <div v-for="(file, index) in fileList" :key="index" class="file-item">
            <el-icon class="file-icon"><Document /></el-icon>
            <span class="file-name">{{ file.name }}</span>
            <el-button class="delete-btn" type="danger" :icon="Delete" circle size="small" @click="removeFile(index)" />
          </div>
        </div>
      </div>
      <div class="btn-center">
        <el-button type="primary" :loading="recognizing" @click="startRecognize" :disabled="fileList.length === 0">
          开始识别
        </el-button>
      </div>
    </el-card>

    <el-card v-if="ocrResults.length > 0" style="margin-top: 16px">
      <template #header>
        <div class="flex-between">
          <span>识别结果预览</span>
          <el-button type="success" @click="saveResults">
            <el-icon><Check /></el-icon> 一键保存
          </el-button>
        </div>
      </template>

      <el-table :data="ocrResults" border>
        <el-table-column label="类型" width="80">
          <template #default="{ row }">
            <el-tag :type="row.type === 'input' ? 'primary' : 'warning'" size="small">
              {{ row.type === 'input' ? '进项' : '销项' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="number" label="发票号码" width="180" />
        <el-table-column prop="date" label="开票日期" width="120" />
        <el-table-column label="销售方" prop="sellerName" />
        <el-table-column label="购买方" prop="buyerName" />
        <el-table-column prop="amount" label="不含税金额" align="right" width="120">
          <template #default="{ row }">¥{{ formatMoney(row.amount) }}</template>
        </el-table-column>
        <el-table-column prop="tax" label="税额" align="right" width="100">
          <template #default="{ row }">¥{{ formatMoney(row.tax) }}</template>
        </el-table-column>
        <el-table-column prop="total" label="价税合计" align="right" width="120">
          <template #default="{ row }">¥{{ formatMoney(row.total) }}</template>
        </el-table-column>
        <el-table-column prop="invoiceType" label="发票类型" width="120" />
        <el-table-column label="操作" width="80">
          <template #default="{ $index }">
            <el-button size="small" type="danger" @click="removeResult($index)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { ElMessage } from 'element-plus'
import { Monitor, UploadFilled, Check, Document, Delete } from '@element-plus/icons-vue'
import { OcrApi, FileApi, InvoiceApi } from '../api/invoice'
import api from '../api/index'
import { useUserStore } from '../stores/user'

const userStore = useUserStore()
const fileList = ref([])

const removeFile = (index) => {
  fileList.value.splice(index, 1)
}
const ocrResults = ref([])
const recognizing = ref(false)

const formatMoney = (num) => {
  return Number(num || 0).toLocaleString('zh-CN', { minimumFractionDigits: 2 })
}

const handleFileChange = (file) => {
  // 检查文件是否已存在
  const exists = fileList.value.some(f => f.name === file.name)
  if (exists) {
    ElMessage({
      message: '文件 ' + file.name + ' 已存在，请勿重复上传',
      type: 'warning',
      duration: 2000,
      offset: 200
    })
    return
  }
  fileList.value.push(file)
}

const startRecognize = async () => {
  recognizing.value = true
  ocrResults.value = []

  for (const file of fileList.value) {
    try {
      // 上传文件
      const uploadRes = await FileApi.upload(file.raw)
      const fileUrl = uploadRes.url

      // OCR识别
      const ocrRes = await OcrApi.recognize(fileUrl)
      if (ocrRes.success && ocrRes.parsed) {
        const d = ocrRes.parsed
        // 判断进项/销项
        const isInput = d.buyerTaxNo === userStore.myTaxNo
        ocrResults.value.push({
          ...d,
          type: isInput ? 'input' : 'output',
          imageUrl: fileUrl  // 保存图片路径
        })
      }
    } catch (e) {
      console.error('识别失败:', e)
    }
  }

  recognizing.value = false
  fileList.value = []
}

const removeResult = (index) => {
  ocrResults.value.splice(index, 1)
}

// 转换日期格式：2026年06月23日 → 2026-06-23
const convertDate = (dateStr) => {
  if (!dateStr) return ''
  // 如果已经是标准格式，直接返回
  if (/^\d{4}-\d{2}-\d{2}$/.test(dateStr)) return dateStr
  // 转换中文日期格式
  const match = dateStr.match(/(\d{4})年(\d{2})月(\d{2})日/)
  if (match) {
    return `${match[1]}-${match[2]}-${match[3]}`
  }
  return dateStr
}

const saveResults = async () => {
  let savedCount = 0
  let duplicateList = []

  for (const item of ocrResults.value) {
    try {
      const invoiceData = {
        // 不传type，让后端根据税号自动判断
        code: item.code || '',
        number: item.number || '',
        date: convertDate(item.date),
        amount: item.amount || 0,
        tax: item.tax || 0,
        total: item.total || 0,
        rate: '13%',
        invoiceType: item.invoiceType || '',
        buyerName: item.buyerName || '',
        buyerTaxNo: item.buyerTaxNo || '',
        sellerName: item.sellerName || '',
        sellerTaxNo: item.sellerTaxNo || '',
        status: '未认证',
        remark: 'OCR导入',
        imageUrl: item.imageUrl || ''  // 保存图片路径
      }

      await InvoiceApi.create(invoiceData)
      // 流水记录由后端自动创建

      savedCount++
    } catch (e) {
      // 如果是重复发票错误，记录下来
      if (e.response?.data?.error?.includes('已存在')) {
        duplicateList.push(item.number || item.code)
      } else {
        console.error('保存失败:', e)
      }
    }
  }

  if (savedCount > 0 || duplicateList.length > 0) {
    let msg = `保存成功 ${savedCount} 张`
    if (duplicateList.length > 0) {
      msg += `，重复 ${duplicateList.length} 张: ${duplicateList.join(', ')}`
    }
    ElMessage({
      message: msg,
      type: savedCount > 0 ? 'success' : 'warning',
      duration: 3000
    })
    ocrResults.value = []
  } else {
    ElMessage.warning('没有可保存的数据')
  }
}
</script>

<style scoped>
.upload-area {
  width: 100%;
}
.upload-area :deep(.el-upload) {
  width: 100%;
}
.upload-area :deep(.el-upload-dragger) {
  width: 100%;
  height: 200px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  border: 2px dashed #d9d9d9;
  border-radius: 8px;
  background: #fafafa;
  transition: all 0.3s;
}
.upload-area :deep(.el-upload-dragger:hover) {
  border-color: #1890ff;
  background: #e6f7ff;
}
.upload-area :deep(.el-upload-dragger .el-icon--upload) {
  font-size: 48px;
  color: #1890ff;
  margin-bottom: 12px;
}
.upload-area :deep(.el-upload-dragger .el-upload__text) {
  font-size: 14px;
  color: #666;
}
.upload-area :deep(.el-upload-dragger .el-upload__text em) {
  color: #1890ff;
  font-style: normal;
}
.upload-area :deep(.el-upload__tip) {
  font-size: 12px;
  color: #999;
  margin-top: 8px;
  text-align: center;
}
.upload-area :deep(.el-upload-list) {
  margin-top: 16px;
  border: 1px solid #e8e8e8;
  border-radius: 6px;
  padding: 8px;
  background: #fafafa;
}
.upload-area :deep(.el-upload-list__item) {
  margin: 4px 0;
  padding: 8px 12px;
  border: 1px solid #e8e8e8;
  border-radius: 4px;
  background: #fff;
  transition: all 0.3s;
}
.upload-area :deep(.el-upload-list__item:hover) {
  border-color: #1890ff;
  background: #e6f7ff;
}
.upload-area :deep(.el-upload-list__item .el-icon) {
  color: #1890ff;
}
.upload-area :deep(.el-upload-list__item .el-upload-list__item-name) {
  color: #333;
  font-size: 13px;
}
.file-list-wrapper {
  margin-top: 16px;
  border: 1px solid #e8e8e8;
  border-radius: 8px;
  overflow: hidden;
}
.file-list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 16px;
  background: #fafafa;
  border-bottom: 1px solid #e8e8e8;
  font-size: 13px;
  color: #666;
}
.file-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
}
.file-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 12px;
  background: #fafafa;
  border: 1px solid #e8e8e8;
  border-radius: 6px;
  transition: all 0.3s;
}
.file-item:hover {
  border-color: #1890ff;
  background: #e6f7ff;
}
.file-icon {
  font-size: 20px;
  color: #1890ff;
  flex-shrink: 0;
}
.file-name {
  flex: 1;
  font-size: 13px;
  color: #333;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.delete-btn {
  flex-shrink: 0;
  opacity: 0;
  transition: opacity 0.3s;
}
.file-item:hover .delete-btn {
  opacity: 1;
}
.flex-between {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.btn-center {
  display: flex;
  justify-content: center;
  margin-top: 16px;
}
</style>
