<template>
  <div class="pagination-bar">
    <div class="page-info">
      共 <strong>{{ total }}</strong> 条
      <el-select v-model="currentPageSize" size="small" style="width: 100px; margin-left: 10px;" @change="onPageSizeChange">
        <el-option :value="10" label="10条/页" />
        <el-option :value="20" label="20条/页" />
        <el-option :value="50" label="50条/页" />
        <el-option :value="100" label="100条/页" />
      </el-select>
    </div>
    <div class="page-controls">
      <button class="page-btn" @click="changePage(1)" :disabled="currentPage === 1">
        <el-icon><DArrowLeft /></el-icon>
      </button>
      <button class="page-btn" @click="changePage(currentPage - 1)" :disabled="currentPage === 1">
        <el-icon><ArrowLeft /></el-icon>
      </button>
      <button class="page-btn active">{{ currentPage }}</button>
      <button class="page-btn" @click="changePage(currentPage + 1)" :disabled="currentPage >= totalPages">
        <el-icon><ArrowRight /></el-icon>
      </button>
      <button class="page-btn" @click="changePage(totalPages)" :disabled="currentPage >= totalPages">
        <el-icon><DArrowRight /></el-icon>
      </button>
    </div>
  </div>
</template>

<script setup>
import { computed, ref, watch } from 'vue'
import { ArrowLeft, ArrowRight, DArrowLeft, DArrowRight } from '@element-plus/icons-vue'

const props = defineProps({
  total: { type: Number, default: 0 },
  currentPage: { type: Number, default: 1 },
  pageSize: { type: Number, default: 10 }
})

const emit = defineEmits(['update:currentPage', 'update:pageSize'])

const currentPageSize = ref(props.pageSize)

watch(() => props.pageSize, (val) => {
  currentPageSize.value = val
})

const totalPages = computed(() => Math.ceil(props.total / currentPageSize.value) || 1)

const changePage = (page) => {
  if (page >= 1 && page <= totalPages.value) {
    emit('update:currentPage', page)
  }
}

const onPageSizeChange = (val) => {
  emit('update:pageSize', val)
  emit('update:currentPage', 1)
}
</script>

<style scoped>
.pagination-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-top: 16px;
}
.page-info {
  font-size: 13px;
  color: #999;
}
.page-info strong {
  color: #333;
  font-weight: 600;
}
.page-controls {
  display: flex;
  align-items: center;
}
.page-btn {
  min-width: 24px;
  height: 24px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  border: 1px solid #d9d9d9;
  background-color: #fff;
  color: rgba(0, 0, 0, 0.65);
  font-size: 12px;
  cursor: pointer;
  transition: all 0.3s;
  margin: 0 1px;
  padding: 0 4px;
}
.page-btn:hover:not(.disabled):not(.active) {
  color: #1890ff;
  border-color: #1890ff;
}
.page-btn.active {
  background-color: #1890ff;
  border-color: #1890ff;
  color: #fff;
}
.page-btn:disabled {
  cursor: not-allowed;
  color: rgba(0, 0, 0, 0.25);
  border-color: #d9d9d9;
  background-color: #f5f5f5;
}
</style>
