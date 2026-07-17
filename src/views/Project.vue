<template>
  <div>
    <el-row :gutter="16" class="stats-row">
      <el-col :span="6">
        <el-card shadow="never" class="stat-card">
          <div class="stat-label">进行中项目</div>
          <div class="stat-value blue">{{ stats.ongoingCount }}</div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="never" class="stat-card">
          <div class="stat-label">已完成项目</div>
          <div class="stat-value green">{{ stats.completedCount }}</div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="never" class="stat-card">
          <div class="stat-label">项目总预算</div>
          <div class="stat-value orange">¥{{ formatMoney(stats.totalBudget) }}</div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="never" class="stat-card">
          <div class="stat-label">已开票金额</div>
          <div class="stat-value purple">¥{{ formatMoney(stats.totalInvoiced) }}</div>
        </el-card>
      </el-col>
    </el-row>

    <el-card>
      <template #header>
        <div class="flex-between">
          <span>项目列表</span>
          <div class="btn-group">
            <el-button @click="exportData">
              <el-icon><Download /></el-icon> 导出Excel
            </el-button>
            <el-button type="primary" @click="showAdd">
              <el-icon><Plus /></el-icon> 新建项目
            </el-button>
          </div>
        </div>
      </template>

      <el-table :data="tableData" border v-loading="loading" style="width: 100%">
        <el-table-column type="index" label="序号" width="60" align="center" />
        <el-table-column prop="projectNo" label="项目编号" min-width="180" align="center">
          <template #default="{ row }">
            <span class="clickable-text" @click="showDetail(row)">{{ row.projectNo }}</span>
          </template>
        </el-table-column>
        <el-table-column prop="projectName" label="项目名称" min-width="250" align="center">
          <template #default="{ row }">
            <span class="clickable-text" @click="showDetail(row)"><strong>{{ row.projectName }}</strong></span>
          </template>
        </el-table-column>
        <el-table-column prop="manager" label="负责人" width="100" align="center" />
        <el-table-column prop="budget" label="预算金额" align="right" width="140">
          <template #default="{ row }">¥{{ formatMoney(row.budget) }}</template>
        </el-table-column>
        <el-table-column prop="invoicedAmount" label="已开票金额" align="right" width="140">
          <template #default="{ row }">¥{{ formatMoney(row.invoicedAmount) }}</template>
        </el-table-column>
        <el-table-column prop="status" label="状态" width="100" align="center">
          <template #default="{ row }">
            <el-tag :type="statusType(row.status)" size="small">{{ row.status }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column label="操作" width="200" align="center">
          <template #default="{ row }">
            <el-button size="small" @click="showDetail(row)">详情</el-button>
            <el-button size="small" @click="showEdit(row)">编辑</el-button>
            <el-button v-if="row.status === '待启动'" size="small" type="danger" @click="handleDelete(row)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>

    <!-- 新增/编辑对话框 -->
    <el-dialog v-model="dialogVisible" :title="isEdit ? '编辑项目' : '新建项目'" width="600px">
      <el-form :model="formData" label-width="100px">
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item label="项目编号">
              <el-input v-model="formData.projectNo" disabled />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="项目名称">
              <el-input v-model="formData.projectName" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item label="负责人">
              <el-input v-model="formData.manager" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="状态">
              <el-select v-model="formData.status">
                <el-option label="待启动" value="待启动" />
                <el-option label="进行中" value="进行中" />
                <el-option label="已完成" value="已完成" />
              </el-select>
            </el-form-item>
          </el-col>
        </el-row>
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item label="预算金额">
              <el-input-number v-model="formData.budget" :precision="2" :min="0" style="width: 100%" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="已开票金额">
              <el-input-number v-model="formData.invoicedAmount" :precision="2" :min="0" style="width: 100%" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item label="回款合同额">
              <el-input-number v-model="formData.receiveContractAmount" :precision="2" :min="0" style="width: 100%" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="支出合同额">
              <el-input-number v-model="formData.expenseContractAmount" :precision="2" :min="0" style="width: 100%" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-form-item label="备注">
          <el-input v-model="formData.remark" type="textarea" :rows="2" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleSave">保存</el-button>
      </template>
    </el-dialog>

    <!-- 项目详情对话框 -->
    <el-dialog v-model="detailVisible" :title="currentProject ? currentProject.projectName + ' - 项目详情' : '项目详情'" width="90%" top="5vh">
      <div v-if="currentProject">
        <!-- 顶部Header -->
        <div class="pd-header">
          <div class="pd-header-left">
            <span class="pd-name">{{ currentProject.projectName }}</span>
            <span class="pd-meta">负责人：{{ currentProject.manager }}</span>
            <el-tag size="small" :type="statusType(currentProject.status)">{{ currentProject.status }}</el-tag>
          </div>
          <div class="pd-header-right">
            <span class="pd-received-label">已回款金额：</span>
            <span class="pd-received-num">¥{{ formatMoney(detailStats.totalReceipt) }}</span>
          </div>
        </div>

        <!-- 汇总区域 -->
        <div class="section-box">
          <!-- 概览卡片 -->
          <div class="summary-grid">
            <div class="summary-item border-purple">
              <div class="s-icon-wrap bg-purple"><el-icon size="18"><Document /></el-icon></div>
              <div class="s-content">
                <div class="s-label">合同总额（回款端）</div>
                <div class="s-value purple">¥{{ formatMoney(currentProject.receiveContractAmount) }}</div>
              </div>
            </div>
            <div class="summary-item border-orange">
              <div class="s-icon-wrap bg-orange"><el-icon size="18"><Money /></el-icon></div>
              <div class="s-content">
                <div class="s-label">已回款金额</div>
                <div class="s-value orange">¥{{ formatMoney(detailStats.totalReceipt) }}</div>
              </div>
            </div>
            <div class="summary-item border-blue">
              <div class="s-icon-wrap bg-blue"><el-icon size="18"><Document /></el-icon></div>
              <div class="s-content">
                <div class="s-label">合同总额（支出端）</div>
                <div class="s-value blue">¥{{ formatMoney(currentProject.expenseContractAmount) }}</div>
              </div>
            </div>
            <div class="summary-item border-green">
              <div class="s-icon-wrap bg-green"><el-icon size="18"><Money /></el-icon></div>
              <div class="s-content">
                <div class="s-label">已支付金额</div>
                <div class="s-value green">¥{{ formatMoney(detailStats.paidAmount) }}</div>
              </div>
            </div>
            <div class="summary-item">
              <div class="s-icon-wrap"><el-icon size="18"><DataAnalysis /></el-icon></div>
              <div class="s-content">
                <div class="s-label">剩余回款金额</div>
                <div class="s-value" :class="(detailStats.totalReceipt - detailStats.paidAmount) >= 0 ? 'green' : 'red'">¥{{ formatMoney(detailStats.totalReceipt - detailStats.paidAmount) }}</div>
              </div>
            </div>
            <div class="summary-item border-red">
              <div class="s-icon-wrap bg-red"><el-icon size="18"><Money /></el-icon></div>
              <div class="s-content">
                <div class="s-label">垫资金额</div>
                <div class="s-value red">¥{{ formatMoney(detailStats.totalAdvance) }}</div>
              </div>
            </div>
            <div class="summary-item border-cyan">
              <div class="s-icon-wrap bg-cyan"><el-icon size="18"><Document /></el-icon></div>
              <div class="s-content">
                <div class="s-label">进项金额</div>
                <div class="s-value cyan">¥{{ formatMoney(inputInvoiceStats?.totalAmount || 0) }}</div>
              </div>
            </div>
            <div class="summary-item border-pink">
              <div class="s-icon-wrap bg-pink"><el-icon size="18"><Document /></el-icon></div>
              <div class="s-content">
                <div class="s-label">销项金额</div>
                <div class="s-value pink">¥{{ formatMoney(outputInvoiceStats?.totalAmount || 0) }}</div>
              </div>
            </div>
          </div>

          <!-- 进度条 -->
          <div class="progress-row">
            <div class="progress-card">
              <div class="progress-header">
                <span class="progress-title">支出进度</span>
                <span class="progress-sub">已付 / 合同总额</span>
              </div>
              <div class="progress-bar">
                <div class="progress-track">
                  <div class="progress-fill blue" :style="{ width: expenseProgress + '%' }"></div>
                </div>
                <span class="progress-label blue">{{ expenseProgress }}%</span>
              </div>
            </div>
            <div class="progress-card">
              <div class="progress-header">
                <span class="progress-title">回款进度</span>
                <span class="progress-sub">已回款 / 合同总额</span>
              </div>
              <div class="progress-bar">
                <div class="progress-track">
                  <div class="progress-fill green" :style="{ width: receiptProgress + '%' }"></div>
                </div>
                <span class="progress-label green">{{ receiptProgress }}%</span>
              </div>
            </div>
          </div>
        </div>

        <!-- 数据区域 -->
        <div class="section-box" style="margin-top: 16px;">
          <!-- Tabs -->
          <el-tabs v-model="detailTab">
          <!-- 垫资Tab -->
          <el-tab-pane label="垫资" name="advances">
            <div class="tab-header">
              <div class="tab-header-left">
                <h3 class="tab-title"><el-icon style="color: #e6a23c; margin-right: 6px;"><Money /></el-icon>垫资记录</h3>
                <div class="tab-subtitle">合计垫资：<strong style="color: #f56c6c;">¥{{ formatMoney(detailStats.totalAdvance) }}</strong></div>
              </div>
              <el-button type="primary" size="small" @click="showAdvanceForm">
                <el-icon><Plus /></el-icon> 新增垫资
              </el-button>
            </div>
            <div class="table-wrapper">
              <el-table :data="pagedAdvances" style="width: 100%">
                <el-table-column type="index" label="序号" width="60" align="center" />
                <el-table-column prop="customerName" label="垫资主体" min-width="120" align="center">
                  <template #header>
                    <div class="filter-header">
                      <span>垫资主体</span>
                      <el-popover placement="bottom" :width="200" trigger="click">
                        <template #reference>
                          <el-icon class="filter-icon" size="22"><Filter /></el-icon>
                        </template>
                        <el-input v-model="advanceFilter.customerName" placeholder="搜索主体" clearable size="small" @input="handleAdvanceFilter" />
                      </el-popover>
                    </div>
                  </template>
                </el-table-column>
                <el-table-column prop="amount" label="垫资金额" align="right" width="120">
                  <template #default="{ row }"><span style="color: #f56c6c; font-weight: 600;">¥{{ formatMoney(row.amount) }}</span></template>
                </el-table-column>
                <el-table-column prop="advanceDate" label="垫资日期" width="110" align="center" />
                <el-table-column prop="purpose" label="用途" min-width="150" show-overflow-tooltip>
                  <template #header>
                    <div class="filter-header">
                      <span>用途</span>
                      <el-popover placement="bottom" :width="200" trigger="click">
                        <template #reference>
                          <el-icon class="filter-icon" size="22"><Filter /></el-icon>
                        </template>
                        <el-input v-model="advanceFilter.purpose" placeholder="搜索用途" clearable size="small" @input="handleAdvanceFilter" />
                      </el-popover>
                    </div>
                  </template>
                </el-table-column>
                <el-table-column prop="invoiceType" label="发票" width="100" align="center">
                  <template #default="{ row }">
                    <el-button v-if="row.invoiceType === '有票'" type="primary" link size="small" @click="viewInvoice(row.invoiceNo)">
                      有票 <el-icon><View /></el-icon>
                    </el-button>
                    <el-tag v-else-if="row.invoiceType === '无票'" size="small" type="warning">无票</el-tag>
                    <span v-else>-</span>
                  </template>
                </el-table-column>
                <el-table-column label="操作" width="150" align="center">
                  <template #default="{ row }">
                    <el-button size="small" @click="showEditAdvanceForm(row)">编辑</el-button>
                    <el-button size="small" type="danger" @click="handleDeleteAdvance(row.id)">删除</el-button>
                  </template>
                </el-table-column>
              </el-table>
              <div class="pagination-wrapper" v-if="filteredAdvances.length > pageSize">
                <el-pagination background layout="total, prev, pager, next" :total="filteredAdvances.length" :page-size="pageSize" v-model:current-page="advancePage" />
              </div>
            </div>
          </el-tab-pane>

          <!-- 支出Tab -->
          <el-tab-pane label="支出" name="expenses">
            <div class="tab-header">
              <div class="tab-header-left">
                <h3 class="tab-title"><el-icon style="color: #409eff; margin-right: 6px;"><Document /></el-icon>支出记录</h3>
                <div class="tab-subtitle">合计支出：<strong style="color: #f56c6c;">¥{{ formatMoney(detailStats.totalExpense) }}</strong></div>
              </div>
              <el-button type="primary" size="small" @click="showExpenseForm">
                <el-icon><Plus /></el-icon> 新增支出
              </el-button>
            </div>
            <div class="table-wrapper">
              <el-table :data="pagedExpenses" style="width: 100%">
                <el-table-column type="index" label="序号" width="60" align="center" />
                <el-table-column prop="customerName" label="客户" min-width="120" align="center">
                  <template #header>
                    <div class="filter-header">
                      <span>客户</span>
                      <el-popover placement="bottom" :width="200" trigger="click">
                        <template #reference>
                          <el-icon class="filter-icon" size="22"><Filter /></el-icon>
                        </template>
                        <el-input v-model="expenseFilter.customerName" placeholder="搜索客户" clearable size="small" @input="handleExpenseFilter" />
                      </el-popover>
                    </div>
                  </template>
                </el-table-column>
                <el-table-column prop="expenseName" label="支出名称" min-width="120" show-overflow-tooltip>
                  <template #header>
                    <div class="filter-header">
                      <span>支出名称</span>
                      <el-popover placement="bottom" :width="200" trigger="click">
                        <template #reference>
                          <el-icon class="filter-icon" size="22"><Filter /></el-icon>
                        </template>
                        <el-input v-model="expenseFilter.expenseName" placeholder="搜索名称" clearable size="small" @input="handleExpenseFilter" />
                      </el-popover>
                    </div>
                  </template>
                </el-table-column>
                <el-table-column prop="amount" label="金额" align="right" width="120">
                  <template #default="{ row }"><span style="color: #f56c6c; font-weight: 600;">¥{{ formatMoney(row.amount) }}</span></template>
                </el-table-column>
                <el-table-column prop="expenseDate" label="日期" width="110" align="center" />
                <el-table-column prop="invoiceType" label="发票" width="100" align="center">
                  <template #default="{ row }">
                    <el-button v-if="row.invoiceType === '有票'" type="primary" link size="small" @click="viewInvoice(row.invoiceNo)">
                      有票 <el-icon><View /></el-icon>
                    </el-button>
                    <el-tag v-else-if="row.invoiceType === '无票'" size="small" type="warning">无票</el-tag>
                    <span v-else>-</span>
                  </template>
                </el-table-column>
                <el-table-column label="操作" width="150" align="center">
                  <template #default="{ row }">
                    <el-button size="small" @click="showEditExpenseForm(row)">编辑</el-button>
                    <el-button size="small" type="danger" @click="handleDeleteExpense(row.id)">删除</el-button>
                  </template>
                </el-table-column>
              </el-table>
              <div class="pagination-wrapper" v-if="filteredExpenses.length > pageSize">
                <el-pagination background layout="total, prev, pager, next" :total="filteredExpenses.length" :page-size="pageSize" v-model:current-page="expensePage" />
              </div>
            </div>
          </el-tab-pane>

          <!-- 回款Tab -->
          <el-tab-pane label="回款" name="receipts">
            <div class="tab-header">
              <div class="tab-header-left">
                <h3 class="tab-title"><el-icon style="color: #67c23a; margin-right: 6px;"><Money /></el-icon>回款记录</h3>
                <div class="tab-subtitle">合计回款：<strong style="color: #67c23a;">¥{{ formatMoney(detailStats.totalReceipt) }}</strong></div>
              </div>
              <el-button type="primary" size="small" @click="showReceiptForm">
                <el-icon><Plus /></el-icon> 新增回款
              </el-button>
            </div>
            <div class="table-wrapper">
              <el-table :data="pagedReceipts" style="width: 100%">
                <el-table-column type="index" label="序号" width="60" align="center" />
                <el-table-column prop="customerName" label="客户" min-width="120" align="center">
                  <template #header>
                    <div class="filter-header">
                      <span>客户</span>
                      <el-popover placement="bottom" :width="200" trigger="click">
                        <template #reference>
                          <el-icon class="filter-icon" size="22"><Filter /></el-icon>
                        </template>
                        <el-input v-model="receiptFilter.customerName" placeholder="搜索客户" clearable size="small" @input="handleReceiptFilter" />
                      </el-popover>
                    </div>
                  </template>
                </el-table-column>
                <el-table-column prop="receiptName" label="回款名称" min-width="120" show-overflow-tooltip>
                  <template #header>
                    <div class="filter-header">
                      <span>回款名称</span>
                      <el-popover placement="bottom" :width="200" trigger="click">
                        <template #reference>
                          <el-icon class="filter-icon" size="22"><Filter /></el-icon>
                        </template>
                        <el-input v-model="receiptFilter.receiptName" placeholder="搜索名称" clearable size="small" @input="handleReceiptFilter" />
                      </el-popover>
                    </div>
                  </template>
                </el-table-column>
                <el-table-column prop="amount" label="金额" align="right" width="120">
                  <template #default="{ row }"><span style="color: #67c23a; font-weight: 600;">¥{{ formatMoney(row.amount) }}</span></template>
                </el-table-column>
                <el-table-column prop="receiptDate" label="日期" width="110" align="center" />
                <el-table-column prop="invoiceType" label="发票" width="100" align="center">
                  <template #default="{ row }">
                    <el-button v-if="row.invoiceType === '有票'" type="primary" link size="small" @click="viewInvoice(row.invoiceNo)">
                      有票 <el-icon><View /></el-icon>
                    </el-button>
                    <el-tag v-else-if="row.invoiceType === '无票'" size="small" type="warning">无票</el-tag>
                    <span v-else>-</span>
                  </template>
                </el-table-column>
                <el-table-column label="操作" width="150" align="center">
                  <template #default="{ row }">
                    <el-button size="small" @click="showEditReceiptForm(row)">编辑</el-button>
                    <el-button size="small" type="danger" @click="handleDeleteReceipt(row.id)">删除</el-button>
                  </template>
                </el-table-column>
              </el-table>
              <div class="pagination-wrapper" v-if="filteredReceipts.length > pageSize">
                <el-pagination background layout="total, prev, pager, next" :total="filteredReceipts.length" :page-size="pageSize" v-model:current-page="receiptPage" />
              </div>
            </div>
          </el-tab-pane>

          <!-- 资金流水Tab -->
          <el-tab-pane label="资金流水" name="flows">
            <div class="tab-header">
              <div class="tab-header-left">
                <h3 class="tab-title"><el-icon style="color: #909399; margin-right: 6px;"><DataAnalysis /></el-icon>资金流水</h3>
                <div class="tab-subtitle">记录垫资、支出、回款的所有操作</div>
              </div>
            </div>
            <div v-if="projectDetail.flows.length === 0" class="empty-state">
              <el-icon size="48" color="#c0c4cc"><Document /></el-icon>
              <div style="margin-top: 12px; color: #909399;">暂无资金流水</div>
            </div>
            <div v-else class="flow-list">
              <div v-for="(flow, index) in pagedFlows" :key="index" class="flow-item">
                <div :class="['flow-icon', getFlowTypeClass(flow.type)]">
                  <el-icon size="16">
                    <Money v-if="flow.type === 'receipt'" />
                    <Document v-else-if="flow.type === 'expense'" />
                    <DataAnalysis v-else />
                  </el-icon>
                </div>
                <div class="flow-info">
                  <div class="flow-title">
                    {{ flow.customerName }}
                    <el-tag size="small" :type="flow.type === 'receipt' ? 'success' : 'warning'" style="margin-left: 6px; font-size: 10px;">
                      {{ flow.type === 'receipt' ? '已回款' : flow.type === 'expense' ? '已付款' : '已垫资' }}
                    </el-tag>
                  </div>
                  <div class="flow-meta">
                    <el-icon size="12"><Document /></el-icon> {{ flow.flowDate }} · {{ flow.summary || '-' }} ·
                    <span :class="['invoice-tag', flow.invoiceType === '有票' ? 'has-invoice' : 'no-invoice']">
                      {{ flow.invoiceType || '无票' }}
                    </span>
                  </div>
                </div>
                <div :class="['flow-amount', getFlowTypeClass(flow.type)]">
                  {{ flow.type === 'expense' ? '-' : flow.type === 'receipt' ? '+' : '' }}¥{{ formatMoney(flow.amount) }}
                </div>
              </div>
              <div class="pagination-wrapper" v-if="(projectDetail.flows || []).length > pageSize">
                <el-pagination background layout="total, prev, pager, next" :total="(projectDetail.flows || []).length" :page-size="pageSize" v-model:current-page="flowPage" />
              </div>
            </div>
          </el-tab-pane>

          <!-- 备忘录Tab -->
          <el-tab-pane label="备忘录" name="memos">
            <div class="tab-header">
              <div class="tab-header-left">
                <h3 class="tab-title"><el-icon style="color: #e6a23c; margin-right: 6px;"><Notebook /></el-icon>备忘录</h3>
                <div class="tab-subtitle">项目相关备忘记录</div>
              </div>
              <el-button type="primary" size="small" @click="showAddMemo">
                <el-icon><Plus /></el-icon> 新增备忘
              </el-button>
            </div>
            <div v-if="!projectDetail.memos || projectDetail.memos.length === 0" class="empty-state">
              <el-icon size="48" color="#c0c4cc"><Notebook /></el-icon>
              <div style="margin-top: 12px; color: #909399;">暂无备忘录</div>
            </div>
            <div v-else class="memo-list">
              <div v-for="(memo, index) in projectDetail.memos" :key="index" class="memo-item">
                <div class="memo-header">
                  <div class="memo-title">{{ memo.title }}</div>
                  <div class="memo-actions">
                    <el-button size="small" text @click="showEditMemo(memo)">编辑</el-button>
                    <el-button size="small" text type="danger" @click="handleDeleteMemo(memo.id)">删除</el-button>
                  </div>
                </div>
                <div class="memo-content" v-html="memo.content"></div>
                <div class="memo-meta">
                  <el-icon size="12"><User /></el-icon> {{ memo.operator || '管理员' }} · {{ formatTime(memo.updateTime) }}
                </div>
              </div>
            </div>
          </el-tab-pane>

          <!-- 销项Tab -->
          <el-tab-pane label="销项" name="outputInvoices">
            <div class="tab-header">
              <div class="tab-header-left">
                <h3 class="tab-title"><el-icon style="color: #e6a23c; margin-right: 6px;"><Document /></el-icon>销项发票</h3>
                <div class="tab-subtitle">合计金额：<strong style="color: #67c23a;">¥{{ formatMoney(outputInvoiceStats?.totalAmount || 0) }}</strong>，税额：<strong style="color: #f56c6c;">¥{{ formatMoney(outputInvoiceStats?.totalTax || 0) }}</strong></div>
              </div>
              <el-button type="primary" size="small" @click="showAddOutputInvoice">
                <el-icon><Plus /></el-icon> 新增销项
              </el-button>
            </div>
            <div class="table-wrapper">
              <el-table :data="pagedOutputInvoices" style="width: 100%" border>
                <el-table-column type="index" label="序号" width="60" align="center" />
                <el-table-column prop="code" label="发票代码" min-width="120" align="center" show-overflow-tooltip />
                <el-table-column prop="number" label="发票号码" min-width="150" align="center" show-overflow-tooltip />
                <el-table-column prop="buyerName" label="购买方" min-width="150" align="center" show-overflow-tooltip />
                <el-table-column prop="date" label="开票日期" width="110" align="center" />
                <el-table-column prop="amount" label="不含税金额" align="right" width="120">
                  <template #default="{ row }">¥{{ formatMoney(row.amount) }}</template>
                </el-table-column>
                <el-table-column prop="tax" label="税额" align="right" width="100">
                  <template #default="{ row }">¥{{ formatMoney(row.tax) }}</template>
                </el-table-column>
                <el-table-column prop="total" label="价税合计" align="right" width="120">
                  <template #default="{ row }">¥{{ formatMoney(row.total) }}</template>
                </el-table-column>
                <el-table-column prop="status" label="状态" width="90" align="center">
                  <template #default="{ row }">
                    <el-tag :type="row.status === '正常' ? 'success' : row.status === '红冲' ? 'danger' : 'info'" size="small">{{ row.status }}</el-tag>
                  </template>
                </el-table-column>
              </el-table>
              <div class="pagination-wrapper" v-if="outputInvoices.length > pageSize">
                <el-pagination background layout="total, prev, pager, next" :total="outputInvoices.length" :page-size="pageSize" v-model:current-page="outputInvoicePage" />
              </div>
            </div>
          </el-tab-pane>

          <!-- 进项Tab -->
          <el-tab-pane label="进项" name="inputInvoices">
            <div class="tab-header">
              <div class="tab-header-left">
                <h3 class="tab-title"><el-icon style="color: #409eff; margin-right: 6px;"><Document /></el-icon>进项发票</h3>
                <div class="tab-subtitle">合计金额：<strong style="color: #67c23a;">¥{{ formatMoney(inputInvoiceStats?.totalAmount || 0) }}</strong>，税额：<strong style="color: #f56c6c;">¥{{ formatMoney(inputInvoiceStats?.totalTax || 0) }}</strong></div>
              </div>
              <el-button type="primary" size="small" @click="showAddInputInvoice">
                <el-icon><Plus /></el-icon> 新增进项
              </el-button>
            </div>
            <div class="table-wrapper">
              <el-table :data="pagedInputInvoices" style="width: 100%" border>
                <el-table-column type="index" label="序号" width="60" align="center" />
                <el-table-column prop="code" label="发票代码" min-width="120" align="center" show-overflow-tooltip />
                <el-table-column prop="number" label="发票号码" min-width="150" align="center" show-overflow-tooltip />
                <el-table-column prop="sellerName" label="销售方" min-width="150" align="center" show-overflow-tooltip />
                <el-table-column prop="date" label="开票日期" width="110" align="center" />
                <el-table-column prop="amount" label="不含税金额" align="right" width="120">
                  <template #default="{ row }">¥{{ formatMoney(row.amount) }}</template>
                </el-table-column>
                <el-table-column prop="tax" label="税额" align="right" width="100">
                  <template #default="{ row }">¥{{ formatMoney(row.tax) }}</template>
                </el-table-column>
                <el-table-column prop="total" label="价税合计" align="right" width="120">
                  <template #default="{ row }">¥{{ formatMoney(row.total) }}</template>
                </el-table-column>
                <el-table-column prop="status" label="状态" width="90" align="center">
                  <template #default="{ row }">
                    <el-tag :type="row.status === '已认证' ? 'success' : row.status === '不可抵扣' ? 'danger' : 'info'" size="small">{{ row.status }}</el-tag>
                  </template>
                </el-table-column>
              </el-table>
              <div class="pagination-wrapper" v-if="inputInvoices.length > pageSize">
                <el-pagination background layout="total, prev, pager, next" :total="inputInvoices.length" :page-size="pageSize" v-model:current-page="inputInvoicePage" />
              </div>
            </div>
          </el-tab-pane>
        </el-tabs>
        </div>
      </div>
    </el-dialog>

    <!-- 关联发票对话框 -->
    <el-dialog v-model="invoiceFormVisible" :title="invoiceFormType === 'output' ? '关联销项发票' : '关联进项发票'" width="500px">
      <el-form label-width="80px">
        <el-form-item label="选择发票">
          <el-select v-model="selectedInvoiceId" filterable remote :remote-method="searchInvoicesForProject" :loading="invoiceSearchLoading" placeholder="输入发票号搜索..." style="width: 100%" @change="onProjectInvoiceSelect">
            <el-option v-for="inv in projectInvoiceOptions" :key="inv.id" :label="inv.number + ' - ' + (inv.buyerName || inv.sellerName || '')" :value="inv.id">
              <div style="display: flex; justify-content: space-between; align-items: center;">
                <span>{{ inv.number }}</span>
                <span style="color: #999; font-size: 12px;">¥{{ formatMoney(inv.total) }}</span>
              </div>
            </el-option>
          </el-select>
        </el-form-item>
        <el-form-item v-if="selectedInvoice" label="发票信息">
          <div style="padding: 12px; background: #f5f7fa; border-radius: 8px; width: 100%;">
            <div style="display: flex; justify-content: space-between; margin-bottom: 8px;">
              <span><strong>发票号：</strong>{{ selectedInvoice.number }}</span>
              <span><strong>日期：</strong>{{ selectedInvoice.date }}</span>
            </div>
            <div style="display: flex; justify-content: space-between;">
              <span><strong>金额：</strong>¥{{ formatMoney(selectedInvoice.amount) }}</span>
              <span><strong>税额：</strong>¥{{ formatMoney(selectedInvoice.tax) }}</span>
              <span><strong>合计：</strong>¥{{ formatMoney(selectedInvoice.total) }}</span>
            </div>
          </div>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="invoiceFormVisible = false">取消</el-button>
        <el-button type="primary" @click="saveInvoiceForm" :disabled="!selectedInvoice">确认关联</el-button>
      </template>
    </el-dialog>

    <!-- 新增垫资/支出/回款表单 -->
    <el-dialog v-model="flowFormVisible" :title="flowFormTitle" width="600px">
      <el-form :model="flowForm" label-width="100px">
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item label="客户名称">
              <el-input v-model="flowForm.customerName" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="金额">
              <el-input-number v-model="flowForm.amount" :precision="2" :min="0" style="width: 100%" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item label="日期">
              <el-date-picker v-model="flowForm.flowDate" type="date" value-format="YYYY-MM-DD" style="width: 100%" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="发票类型">
              <el-select v-model="flowForm.invoiceType" style="width: 100%">
                <el-option label="有票" value="有票" />
                <el-option label="无票" value="无票" />
              </el-select>
            </el-form-item>
          </el-col>
        </el-row>
        <el-form-item label="关联发票" v-if="flowForm.invoiceType === '有票'">
          <el-select v-model="flowForm.invoiceId" filterable remote :remote-method="searchInvoices" :loading="invoiceLoading" placeholder="输入发票号搜索..." style="width: 100%" @change="onInvoiceSelect">
            <el-option v-for="inv in invoiceOptions" :key="inv.id" :label="inv.number + ' - ' + (inv.buyerName || inv.sellerName || '')" :value="inv.id">
              <div style="display: flex; justify-content: space-between; align-items: center;">
                <span>{{ inv.number }}</span>
                <span style="color: #999; font-size: 12px;">¥{{ formatMoney(inv.total) }}</span>
              </div>
            </el-option>
          </el-select>
        </el-form-item>
        <el-form-item label="发票号" v-if="flowForm.invoiceType === '有票' && flowForm.invoiceNo">
          <el-input v-model="flowForm.invoiceNo" disabled />
        </el-form-item>
        <el-form-item label="事由/用途">
          <el-input v-model="flowForm.summary" type="textarea" :rows="2" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="flowFormVisible = false">取消</el-button>
        <el-button type="primary" @click="saveFlowForm">保存</el-button>
      </template>
    </el-dialog>

    <!-- 备忘录表单 -->
    <el-dialog v-model="memoFormVisible" :title="isEditMemo ? '编辑备忘' : '新增备忘'" width="700px" @opened="handleMemoDialogOpened">
      <el-form :model="memoForm" label-width="80px">
        <el-form-item label="标题">
          <el-input v-model="memoForm.title" placeholder="请输入标题" />
        </el-form-item>
        <el-form-item label="内容">
          <div style="border: 1px solid #dcdfe6; border-radius: 4px;">
            <Toolbar :editor="memoEditor" :defaultConfig="toolbarConfig" style="border-bottom: 1px solid #dcdfe6;" />
            <Editor :defaultConfig="editorConfig" v-model="memoForm.content" style="height: 250px;" @onCreated="handleEditorCreated" />
          </div>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="memoFormVisible = false">取消</el-button>
        <el-button type="primary" @click="saveMemoForm">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, reactive, computed, onMounted, onBeforeUnmount, shallowRef } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Plus, ArrowDown, Document, Money, DataAnalysis, Notebook, User, Download, View, Filter } from '@element-plus/icons-vue'
import { Editor, Toolbar } from '@wangeditor/editor-for-vue'
import '@wangeditor/editor/dist/css/style.css'
import api from '../api/index'
import axios from 'axios'
import { InvoiceApi } from '../api/invoice'

const loading = ref(false)
const tableData = ref([])

const stats = reactive({
  ongoingCount: 0,
  completedCount: 0,
  totalBudget: 0,
  totalInvoiced: 0
})

const dialogVisible = ref(false)
const isEdit = ref(false)

const formData = reactive({
  id: null,
  projectNo: '',
  projectName: '',
  manager: '',
  budget: 0,
  invoicedAmount: 0,
  status: '进行中',
  remark: ''
})

const formatMoney = (num) => {
  return Number(num || 0).toLocaleString('zh-CN', { minimumFractionDigits: 2 })
}

const statusType = (status) => {
  const types = { '进行中': 'primary', '已完成': 'success', '待启动': 'info' }
  return types[status] || 'info'
}

let loadDataTimer = null
const exportData = async () => {
  try {
    const token = localStorage.getItem('token')
    const tenantId = localStorage.getItem('tenantId')
    const res = await axios.get('https://finance.52youran.top/api/projects/export', {
      responseType: 'blob',
      headers: {
        'Authorization': 'Bearer ' + token,
        'X-Tenant-Id': tenantId
      }
    })
    const blob = new Blob([res.data], { type: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet' })
    const link = document.createElement('a')
    link.href = URL.createObjectURL(blob)
    const now = new Date()
    const timestamp = now.getFullYear().toString() +
      (now.getMonth() + 1).toString().padStart(2, '0') +
      now.getDate().toString().padStart(2, '0') + '_' +
      now.getHours().toString().padStart(2, '0') +
      now.getMinutes().toString().padStart(2, '0') +
      now.getSeconds().toString().padStart(2, '0')
    link.download = '项目数据导出_' + timestamp + '.xlsx'
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
    ElMessage.success('导出成功')
  } catch (e) {
    ElMessage.error('导出失败')
    console.error(e)
  }
}

const loadData = async () => {
  if (loadDataTimer) return
  loadDataTimer = setTimeout(() => { loadDataTimer = null }, 100)

  loading.value = true
  try {
    tableData.value = await api.get('/projects')
  } catch (e) {
    console.error(e)
  }
  loading.value = false
}

const loadStats = async () => {
  try {
    const data = await api.get('/projects/stats')
    Object.assign(stats, data)
  } catch (e) {
    console.error(e)
  }
}

const showAdd = async () => {
  isEdit.value = false
  // 自动生成项目编号：P + 年月日 + 序号
  const now = new Date()
  const dateStr = now.getFullYear().toString() +
    (now.getMonth() + 1).toString().padStart(2, '0') +
    now.getDate().toString().padStart(2, '0')
  const timeStr = now.getHours().toString().padStart(2, '0') +
    now.getMinutes().toString().padStart(2, '0') +
    now.getSeconds().toString().padStart(2, '0')

  Object.assign(formData, {
    id: null,
    projectNo: 'P' + dateStr + timeStr,
    projectName: '',
    manager: '',
    budget: 0,
    invoicedAmount: 0,
    status: '进行中',
    remark: ''
  })
  dialogVisible.value = true
}

const showEdit = (row) => {
  isEdit.value = true
  Object.assign(formData, row)
  dialogVisible.value = true
}

const handleSave = async () => {
  try {
    if (isEdit.value) {
      await api.put(`/projects/${formData.id}`, formData)
      ElMessage.success('更新成功')
    } else {
      await api.post('/projects', formData)
      ElMessage.success('创建成功')
    }
    dialogVisible.value = false
    loadData()
    loadStats()
  } catch (e) {
    ElMessage.error('保存失败')
  }
}

const detailVisible = ref(false)
const currentProject = ref(null)
const projectDetail = ref({ advances: [], expenses: [], receipts: [], flows: [], memos: [], relatedInvoiceNos: [] })
const detailTab = ref('advances')

// 垫资筛选
const advanceFilter = reactive({
  customerName: '',
  purpose: ''
})

const filteredAdvances = computed(() => {
  let data = projectDetail.value.advances || []
  if (advanceFilter.customerName) {
    const keyword = advanceFilter.customerName.toLowerCase()
    data = data.filter(item => item.customerName && item.customerName.toLowerCase().includes(keyword))
  }
  if (advanceFilter.purpose) {
    const keyword = advanceFilter.purpose.toLowerCase()
    data = data.filter(item => item.purpose && item.purpose.toLowerCase().includes(keyword))
  }
  return data
})

const handleAdvanceFilter = () => {
  advancePage.value = 1
}

// 回款筛选
const receiptFilter = reactive({
  customerName: '',
  receiptName: ''
})

const filteredReceipts = computed(() => {
  let data = projectDetail.value.receipts || []
  if (receiptFilter.customerName) {
    const keyword = receiptFilter.customerName.toLowerCase()
    data = data.filter(item => item.customerName && item.customerName.toLowerCase().includes(keyword))
  }
  if (receiptFilter.receiptName) {
    const keyword = receiptFilter.receiptName.toLowerCase()
    data = data.filter(item => item.receiptName && item.receiptName.toLowerCase().includes(keyword))
  }
  return data
})

const handleReceiptFilter = () => {
  receiptPage.value = 1
}

// 支出筛选
const expenseFilter = reactive({
  customerName: '',
  expenseName: ''
})

const filteredExpenses = computed(() => {
  let data = projectDetail.value.expenses || []
  if (expenseFilter.customerName) {
    const keyword = expenseFilter.customerName.toLowerCase()
    data = data.filter(item => item.customerName && item.customerName.toLowerCase().includes(keyword))
  }
  if (expenseFilter.expenseName) {
    const keyword = expenseFilter.expenseName.toLowerCase()
    data = data.filter(item => item.expenseName && item.expenseName.toLowerCase().includes(keyword))
  }
  return data
})

const handleExpenseFilter = () => {
  expensePage.value = 1
}

const detailStats = reactive({
  totalAdvance: 0,
  totalExpense: 0,
  paidAmount: 0,
  totalReceipt: 0,
  receiptedAmount: 0,
  balance: 0
})

// 分页状态
const advancePage = ref(1)
const expensePage = ref(1)
const receiptPage = ref(1)
const flowPage = ref(1)
const outputInvoicePage = ref(1)
const inputInvoicePage = ref(1)
const pageSize = ref(10)

// 发票数据
const outputInvoices = ref([])
const inputInvoices = ref([])
const outputInvoiceStats = reactive({ totalAmount: 0, totalTax: 0 })
const inputInvoiceStats = reactive({ totalAmount: 0, totalTax: 0 })

// 分页数据
const pagedAdvances = computed(() => {
  const start = (advancePage.value - 1) * pageSize.value
  return filteredAdvances.value.slice(start, start + pageSize.value)
})
const pagedExpenses = computed(() => {
  const start = (expensePage.value - 1) * pageSize.value
  return filteredExpenses.value.slice(start, start + pageSize.value)
})
const pagedReceipts = computed(() => {
  const start = (receiptPage.value - 1) * pageSize.value
  return filteredReceipts.value.slice(start, start + pageSize.value)
})
const pagedFlows = computed(() => {
  const start = (flowPage.value - 1) * pageSize.value
  return (projectDetail.value.flows || []).slice(start, start + pageSize.value)
})
const pagedOutputInvoices = computed(() => {
  const start = (outputInvoicePage.value - 1) * pageSize.value
  return outputInvoices.value.slice(start, start + pageSize.value)
})
const pagedInputInvoices = computed(() => {
  const start = (inputInvoicePage.value - 1) * pageSize.value
  return inputInvoices.value.slice(start, start + pageSize.value)
})

const expenseProgress = computed(() => {
  if (!currentProject.value || !currentProject.value.expenseContractAmount) return 0
  return Math.min(100, Math.round((detailStats.totalExpense / currentProject.value.expenseContractAmount) * 100))
})

const receiptProgress = computed(() => {
  if (!currentProject.value || !currentProject.value.receiveContractAmount) return 0
  return Math.min(100, Math.round((detailStats.totalReceipt / currentProject.value.receiveContractAmount) * 100))
})

const flowFormVisible = ref(false)
const flowFormTitle = ref('')
const flowFormType = ref('advance')
const flowForm = reactive({
  customerName: '',
  amount: 0,
  flowDate: '',
  invoiceType: '无票',
  invoiceId: '',
  invoiceNo: '',
  summary: ''
})

// 备忘录相关
const memoFormVisible = ref(false)
const isEditMemo = ref(false)
const memoForm = reactive({
  id: null,
  title: '',
  content: ''
})

// 富文本编辑器配置
const memoEditor = shallowRef(null)
const toolbarConfig = {
  excludeKeys: ['fullScreen', 'group-video']
}
const editorConfig = {
  placeholder: '请输入备忘内容...',
  MENU_CONF: {
    uploadImage: {
      disabled: true
    }
  }
}

const handleEditorCreated = (editor) => {
  memoEditor.value = editor
}

onBeforeUnmount(() => {
  const editor = memoEditor.value
  if (editor) editor.destroy()
})

const showAddMemo = () => {
  isEditMemo.value = false
  Object.assign(memoForm, { id: null, title: '', content: '' })
  memoFormVisible.value = true
}

const showEditMemo = (memo) => {
  isEditMemo.value = true
  Object.assign(memoForm, { id: memo.id, title: memo.title, content: memo.content || '' })
  memoFormVisible.value = true
}

const handleMemoDialogOpened = () => {
  // dialog打开后设置编辑器内容
  if (memoEditor.value) {
    memoEditor.value.setHtml(memoForm.content || '')
  }
}

const saveMemoForm = async () => {
  if (!memoForm.title) {
    ElMessage.warning('请输入标题')
    return
  }
  // 从编辑器获取内容
  if (memoEditor.value) {
    memoForm.content = memoEditor.value.getHtml()
  }
  try {
    if (isEditMemo.value) {
      await api.put(`/project-details/memos/${memoForm.id}`, memoForm)
      ElMessage.success('更新成功')
    } else {
      await api.post(`/project-details/${currentProject.value.id}/memos`, memoForm)
      ElMessage.success('保存成功')
    }
    memoFormVisible.value = false
    // 刷新数据但不重置Tab
    const data = await api.get(`/project-details/${currentProject.value.id}`)
    projectDetail.value = data || { advances: [], expenses: [], receipts: [], flows: [], memos: [] }
  } catch (e) {
    ElMessage.error('保存失败')
  }
}

const handleDeleteMemo = async (id) => {
  await ElMessageBox.confirm('确认删除该备忘录？', '提示', { type: 'warning' })
  try {
    await api.delete(`/project-details/memos/${id}`)
    ElMessage.success('删除成功')
    showDetail(currentProject.value)
  } catch (e) {
    ElMessage.error('删除失败')
  }
}

const formatTime = (time) => {
  if (!time) return ''
  return time.replace('T', ' ').slice(0, 16)
}

const invoiceLoading = ref(false)
const invoiceOptions = ref([])

const searchInvoices = async (query) => {
  if (!query) {
    invoiceOptions.value = []
    return
  }
  invoiceLoading.value = true
  try {
    // 获取所有发票，过滤出包含搜索关键词的
    const allInvoices = await InvoiceApi.list()
    // 使用跨项目关联的发票号进行过滤（不能跨项目关联）
    const allRelatedNos = projectDetail.value.allRelatedInvoiceNos || []
    invoiceOptions.value = (allInvoices || []).filter(inv =>
      (inv.number && inv.number.includes(query)) ||
      (inv.buyerName && inv.buyerName.includes(query)) ||
      (inv.sellerName && inv.sellerName.includes(query))
    ).filter(inv => !allRelatedNos.includes(inv.number)).slice(0, 20)
  } catch (e) {
    console.error(e)
  }
  invoiceLoading.value = false
}

const onInvoiceSelect = (invoiceId) => {
  const inv = invoiceOptions.value.find(i => i.id === invoiceId)
  if (inv) {
    flowForm.invoiceNo = inv.number
    flowForm.amount = inv.total || 0
    flowForm.customerName = inv.buyerName || inv.sellerName || ''
  }
}

// 查看发票图片/PDF
const viewInvoice = async (invoiceNo) => {
  if (!invoiceNo) {
    ElMessage.warning('发票号为空')
    return
  }
  try {
    const invoices = await InvoiceApi.list()
    const invoice = invoices.find(i => i.number === invoiceNo)
    if (invoice && invoice.imageUrl) {
      window.open(invoice.imageUrl, '_blank')
    } else {
      ElMessage.warning('未找到发票图片')
    }
  } catch (e) {
    ElMessage.error('查询发票失败')
  }
}

const showAdvanceForm = () => {
  flowFormType.value = 'advance'
  flowFormTitle.value = '新增垫资'
  Object.assign(flowForm, { customerName: '', amount: 0, flowDate: '', invoiceType: '无票', invoiceId: '', invoiceNo: '', summary: '' })
  invoiceOptions.value = []
  flowFormVisible.value = true
}

const showExpenseForm = () => {
  flowFormType.value = 'expense'
  flowFormTitle.value = '新增支出'
  Object.assign(flowForm, { customerName: '', amount: 0, flowDate: '', invoiceType: '无票', invoiceId: '', invoiceNo: '', summary: '' })
  invoiceOptions.value = []
  flowFormVisible.value = true
}

const showReceiptForm = () => {
  flowFormType.value = 'receipt'
  flowFormTitle.value = '新增回款'
  Object.assign(flowForm, { customerName: '', amount: 0, flowDate: '', invoiceType: '无票', invoiceId: '', invoiceNo: '', summary: '' })
  invoiceOptions.value = []
  flowFormVisible.value = true
}

const saveFlowForm = async () => {
  try {
    let url = ''
    let data = {}
    const isEdit = flowForm.id

    if (flowFormType.value === 'advance') {
      url = isEdit ? `/project-details/advances/${flowForm.id}` : '/project-details/advances'
      data = {
        projectId: currentProject.value.id,
        customerName: flowForm.customerName,
        amount: flowForm.amount,
        advanceDate: flowForm.flowDate,
        purpose: flowForm.summary,
        invoiceType: flowForm.invoiceType,
        invoiceNo: flowForm.invoiceNo,
        invoiceAmount: flowForm.invoiceType === '有票' ? flowForm.amount : 0
      }
    } else if (flowFormType.value === 'expense') {
      url = isEdit ? `/project-details/expenses/${flowForm.id}` : '/project-details/expenses'
      data = {
        projectId: currentProject.value.id,
        customerName: flowForm.customerName,
        expenseName: flowForm.summary,
        amount: flowForm.amount,
        expenseDate: flowForm.flowDate,
        invoiceType: flowForm.invoiceType,
        invoiceNo: flowForm.invoiceNo,
        invoiceAmount: flowForm.invoiceType === '有票' ? flowForm.amount : 0,
        remark: ''
      }
    } else {
      url = isEdit ? `/project-details/receipts/${flowForm.id}` : '/project-details/receipts'
      data = {
        projectId: currentProject.value.id,
        customerName: flowForm.customerName,
        receiptName: flowForm.summary,
        amount: flowForm.amount,
        receiptDate: flowForm.flowDate,
        invoiceType: flowForm.invoiceType,
        invoiceNo: flowForm.invoiceNo,
        invoiceAmount: flowForm.invoiceType === '有票' ? flowForm.amount : 0,
        remark: ''
      }
    }

    if (isEdit) {
      await api.put(url, data)
      ElMessage.success('更新成功')
    } else {
      await api.post(url, data)
      ElMessage.success('保存成功')
    }
    flowFormVisible.value = false
    // 刷新数据但不重置Tab
    const data2 = await api.get(`/project-details/${currentProject.value.id}`)
    projectDetail.value = data2 || { advances: [], expenses: [], receipts: [], flows: [], memos: [] }
  } catch (e) {
    ElMessage.error('保存失败')
  }
}

const showDetail = async (row) => {
  currentProject.value = row
  detailVisible.value = true
  detailTab.value = 'advances'

  try {
    const data = await api.get(`/project-details/${row.id}`)
    projectDetail.value = data || { advances: [], expenses: [], receipts: [], flows: [], memos: [] }

    // 计算统计
    detailStats.totalAdvance = projectDetail.value.advances.reduce((sum, a) => sum + Number(a.amount || 0), 0)
    detailStats.totalExpense = projectDetail.value.expenses.reduce((sum, e) => sum + Number(e.amount || 0), 0)
    detailStats.paidAmount = detailStats.totalExpense
    detailStats.totalReceipt = projectDetail.value.receipts.reduce((sum, r) => sum + Number(r.amount || 0), 0)
    detailStats.receiptedAmount = detailStats.totalReceipt
    detailStats.balance = (currentProject.value.receiveContractAmount || 0) - detailStats.paidAmount

    // 加载关联发票
    loadProjectInvoices(row.id)
  } catch (e) {
    console.error(e)
  }
}

const loadProjectInvoices = async (projectId) => {
  try {
    const invoices = await api.get(`/project-details/${projectId}/invoices`)
    outputInvoices.value = (invoices || []).filter(inv => inv.type === 'output')
    inputInvoices.value = (invoices || []).filter(inv => inv.type === 'input')

    // 计算统计
    outputInvoiceStats.totalAmount = outputInvoices.value.reduce((sum, i) => sum + Number(i.amount || 0), 0)
    outputInvoiceStats.totalTax = outputInvoices.value.reduce((sum, i) => sum + Number(i.tax || 0), 0)
    inputInvoiceStats.totalAmount = inputInvoices.value.reduce((sum, i) => sum + Number(i.amount || 0), 0)
    inputInvoiceStats.totalTax = inputInvoices.value.reduce((sum, i) => sum + Number(i.tax || 0), 0)
  } catch (e) {
    console.error('加载发票数据失败:', e)
  }
}

// 关联发票相关
const invoiceFormVisible = ref(false)
const invoiceFormType = ref('output') // output=销项, input=进项
const selectedInvoiceId = ref(null)
const selectedInvoice = ref(null)
const projectInvoiceOptions = ref([])
const invoiceSearchLoading = ref(false)

const showAddOutputInvoice = () => {
  invoiceFormType.value = 'output'
  selectedInvoiceId.value = null
  selectedInvoice.value = null
  projectInvoiceOptions.value = []
  invoiceFormVisible.value = true
  // 自动加载发票列表
  searchInvoicesForProject('')
}

const showAddInputInvoice = () => {
  invoiceFormType.value = 'input'
  selectedInvoiceId.value = null
  selectedInvoice.value = null
  projectInvoiceOptions.value = []
  invoiceFormVisible.value = true
  // 自动加载发票列表
  searchInvoicesForProject('')
}

const searchInvoicesForProject = async (query) => {
  invoiceSearchLoading.value = true
  try {
    // 获取所有发票
    const allInvoices = query
      ? await api.get('/invoices', { params: { search: query } })
      : await api.get('/invoices')
    // 获取当前项目已关联的发票ID
    const relatedInvoices = await api.get(`/project-details/${currentProject.value.id}/invoices`)
    const relatedIds = (relatedInvoices || []).map(inv => inv.id)

    // 筛选对应类型且未关联的发票
    projectInvoiceOptions.value = (allInvoices || []).filter(inv =>
      inv.type === invoiceFormType.value && !relatedIds.includes(inv.id)
    )
  } catch (e) {
    console.error(e)
  }
  invoiceSearchLoading.value = false
}

const onProjectInvoiceSelect = (invoiceId) => {
  selectedInvoice.value = projectInvoiceOptions.value.find(inv => inv.id === invoiceId) || null
}

const saveInvoiceForm = async () => {
  if (!selectedInvoice.value) {
    ElMessage.warning('请选择发票')
    return
  }
  try {
    await api.post(`/project-details/${currentProject.value.id}/invoices`, {
      invoiceId: selectedInvoice.value.id,
      invoiceType: invoiceFormType.value
    })
    ElMessage.success('关联成功')
    invoiceFormVisible.value = false
    // 刷新数据
    loadProjectInvoices(currentProject.value.id)
  } catch (e) {
    ElMessage.error(e.response?.data?.error || '关联失败')
  }
}

const getFlowTypeClass = (type) => {
  return type === 'expense' ? 'out' : type === 'receipt' ? 'in' : 'adv'
}

const handleDeleteAdvance = async (id) => {
  await ElMessageBox.confirm('确认删除该垫资记录？', '提示', { type: 'warning' })
  try {
    await api.delete(`/project-details/advances/${id}`)
    ElMessage.success('删除成功')
    showDetail(currentProject.value)
  } catch (e) {
    ElMessage.error('删除失败')
  }
}

const handleDeleteExpense = async (id) => {
  await ElMessageBox.confirm('确认删除该支出记录？', '提示', { type: 'warning' })
  try {
    await api.delete(`/project-details/expenses/${id}`)
    ElMessage.success('删除成功')
    showDetail(currentProject.value)
  } catch (e) {
    ElMessage.error('删除失败')
  }
}

const handleDeleteReceipt = async (id) => {
  await ElMessageBox.confirm('确认删除该回款记录？', '提示', { type: 'warning' })
  try {
    await api.delete(`/project-details/receipts/${id}`)
    ElMessage.success('删除成功')
    showDetail(currentProject.value)
  } catch (e) {
    ElMessage.error('删除失败')
  }
}

const showEditAdvanceForm = (row) => {
  flowFormType.value = 'advance'
  flowFormTitle.value = '编辑垫资'
  Object.assign(flowForm, {
    id: row.id,
    customerName: row.customerName,
    amount: row.amount,
    flowDate: row.advanceDate,
    invoiceType: row.invoiceType || '无票',
    invoiceId: '',
    invoiceNo: row.invoiceNo || '',
    summary: row.purpose || ''
  })
  flowFormVisible.value = true
}

const showEditExpenseForm = (row) => {
  flowFormType.value = 'expense'
  flowFormTitle.value = '编辑支出'
  Object.assign(flowForm, {
    id: row.id,
    customerName: row.customerName,
    amount: row.amount,
    flowDate: row.expenseDate,
    invoiceType: row.invoiceType || '无票',
    invoiceId: '',
    invoiceNo: row.invoiceNo || '',
    summary: row.expenseName || ''
  })
  flowFormVisible.value = true
}

const showEditReceiptForm = (row) => {
  flowFormType.value = 'receipt'
  flowFormTitle.value = '编辑回款'
  Object.assign(flowForm, {
    id: row.id,
    customerName: row.customerName,
    amount: row.amount,
    flowDate: row.receiptDate,
    invoiceType: row.invoiceType || '无票',
    invoiceId: '',
    invoiceNo: row.invoiceNo || '',
    summary: row.receiptName || ''
  })
  flowFormVisible.value = true
}

const handleDelete = async (row) => {
  await ElMessageBox.confirm(`确认删除项目 ${row.projectName}？`, '提示', { type: 'warning' })
  try {
    await api.delete(`/projects/${row.id}`)
    ElMessage.success('删除成功')
    loadData()
    loadStats()
  } catch (e) {
    ElMessage.error('删除失败')
  }
}

onMounted(() => {
  loadData()
  loadStats()
})
</script>

<style scoped>
.stats-row { margin-bottom: 20px; }
.stat-card { text-align: center; }
.stat-label { font-size: 12px; color: #999; }
.stat-value { font-size: 18px; font-weight: 700; color: #1a1a2e; margin-top: 4px; }
.stat-value.blue { color: #409eff; }
.stat-value.green { color: #67c23a; }
.stat-value.orange { color: #e6a23c; }
.stat-value.purple { color: #909399; }
.flex-between { display: flex; justify-content: space-between; align-items: center; }

:deep(.el-table th) {
  text-align: center;
}

.pd-header {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: 10px;
  padding: 16px 20px;
  margin-bottom: 20px;
  color: #fff;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.pd-header-left {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 12px;
  flex-wrap: wrap;
}

.pd-header-left .pd-name {
  font-size: 18px;
  font-weight: 700;
  margin-right: 4px;
}

.pd-header-left .pd-meta {
  font-size: 12px;
  opacity: 0.9;
  padding: 2px 8px;
  background: rgba(255, 255, 255, 0.2);
  border-radius: 10px;
}

.pd-header-left .el-tag {
  font-size: 12px;
}

.pd-header-left .pd-received-label {
  font-size: 13px;
  opacity: 0.85;
}

.pd-header-left .pd-received-num {
  font-size: 18px;
  font-weight: 700;
}

.pd-header-right {
  display: flex;
  align-items: center;
  gap: 8px;
}

.pd-header-right .pd-received-label {
  font-size: 13px;
  opacity: 0.85;
}

.pd-header-right .pd-received-num {
  font-size: 20px;
  font-weight: 700;
}

.summary-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 12px;
  margin-bottom: 16px;
}

.summary-item {
  background: #fff;
  border-radius: 8px;
  padding: 12px 16px;
  border: 1px solid #f0f0f0;
  text-align: left;
  display: flex;
  align-items: center;
  gap: 12px;
}

.summary-item.border-blue::before {
  content: '';
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 4px;
  background: #409eff;
  border-radius: 10px 0 0 10px;
}

.summary-item.border-green::before {
  content: '';
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 4px;
  background: #67c23a;
  border-radius: 10px 0 0 10px;
}

.summary-item.border-orange::before {
  content: '';
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 4px;
  background: #e6a23c;
  border-radius: 10px 0 0 10px;
}

.summary-item.border-purple::before {
  content: '';
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 4px;
  background: #909399;
  border-radius: 10px 0 0 10px;
}

.summary-item.border-red::before {
  content: '';
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 4px;
  background: #f56c6c;
  border-radius: 10px 0 0 10px;
}

.summary-item.border-cyan::before {
  content: '';
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 4px;
  background: #00d4ff;
  border-radius: 10px 0 0 10px;
}

.summary-item.border-pink::before {
  content: '';
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 4px;
  background: #ff6b9d;
  border-radius: 10px 0 0 10px;
}

.s-icon-wrap {
  width: 36px;
  height: 36px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.s-icon-wrap.bg-blue { background: #ecf5ff; color: #409eff; }
.s-icon-wrap.bg-green { background: #f0f9eb; color: #67c23a; }
.s-icon-wrap.bg-orange { background: #fdf6ec; color: #e6a23c; }
.s-icon-wrap.bg-purple { background: #f4f4f5; color: #909399; }
.s-icon-wrap.bg-red { background: #fef0f0; color: #f56c6c; }
.s-icon-wrap.bg-cyan { background: #e0f7ff; color: #00b4d8; }
.s-icon-wrap.bg-pink { background: #ffe0eb; color: #ff6b9d; }

.s-label {
  font-size: 12px;
  color: #909399;
  margin-bottom: 4px;
}

.s-value {
  font-size: 16px;
  font-weight: 700;
}

.s-value.blue { color: #409eff; }
.s-value.green { color: #67c23a; }
.s-value.orange { color: #e6a23c; }
.s-value.purple { color: #909399; }
.s-value.red { color: #f56c6c; }
.s-value.cyan { color: #00b4d8; }
.s-value.pink { color: #ff6b9d; }

/* 区域框样式 */
.section-box {
  background: #fff;
  border: 1px solid #ebeef5;
  border-radius: 8px;
  padding: 16px;
  margin-bottom: 0;
}

.progress-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px;
  margin-top: 12px;
}

.progress-card {
  background: #fafafa;
  border-radius: 8px;
  padding: 14px 16px;
  border: 1px solid #f0f0f0;
}

.progress-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.progress-title {
  font-size: 13px;
  font-weight: 600;
  color: #303133;
}

.progress-sub {
  font-size: 11px;
  color: #909399;
}

.progress-bar {
  display: flex;
  align-items: center;
  gap: 10px;
}

.progress-track {
  flex: 1;
  height: 8px;
  background: #e4e7ed;
  border-radius: 4px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  border-radius: 4px;
  transition: width 0.6s ease;
}

.progress-fill.blue {
  background: linear-gradient(90deg, #409eff, #79bbff);
}

.progress-fill.green {
  background: linear-gradient(90deg, #67c23a, #95d475);
}

.progress-label {
  font-size: 13px;
  font-weight: 700;
  min-width: 40px;
  text-align: right;
}

.progress-label.blue {
  color: #409eff;
}

.progress-label.green {
  color: #67c23a;
}

/* Tab样式 */
.tab-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 16px;
  padding: 16px 20px;
  background: #fafafa;
  border-radius: 8px;
  border: 1px solid #f0f0f0;
}

.tab-header-left {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.tab-title {
  font-size: 15px;
  font-weight: 600;
  color: #303133;
  margin: 0;
  display: flex;
  align-items: center;
}

.tab-subtitle {
  font-size: 12px;
  color: #909399;
}

.table-wrapper {
  border: 1px solid #ebeef5;
  border-radius: 8px;
  overflow: hidden;
}

.table-wrapper :deep(.el-table th) {
  text-align: center;
  background-color: #fafafa;
}

.table-wrapper :deep(.el-table td) {
  text-align: center;
}

.empty-state {
  text-align: center;
  padding: 40px;
  background: #fafafa;
  border-radius: 8px;
}

/* 资金流水时间线 */
.flow-list {
  list-style: none;
  position: relative;
}

.flow-list::before {
  content: '';
  position: absolute;
  left: 19px;
  top: 8px;
  bottom: 8px;
  width: 2px;
  background: #e8e8e8;
}

.flow-item {
  display: flex;
  align-items: flex-start;
  padding: 14px 0;
  gap: 14px;
  position: relative;
}

.flow-item:last-child {
  padding-bottom: 0;
}

.flow-icon {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  z-index: 1;
  box-shadow: 0 2px 6px rgba(0,0,0,.1);
}

.flow-icon.in {
  background: linear-gradient(135deg, #f6ffed, #d9f7be);
  color: #67c23a;
}

.flow-icon.out {
  background: linear-gradient(135deg, #ecf5ff, #d9ecff);
  color: #409eff;
}

.flow-icon.adv {
  background: linear-gradient(135deg, #fdf6ec, #faecd8);
  color: #e6a23c;
}

.flow-info {
  flex: 1;
  min-width: 0;
  padding-top: 2px;
}

.flow-info .flow-title {
  font-size: 14px;
  font-weight: 500;
  color: #333;
}

.flow-info .flow-meta {
  font-size: 12px;
  color: #999;
  margin-top: 3px;
  display: flex;
  align-items: center;
  gap: 4px;
}

.invoice-tag {
  padding: 1px 6px;
  border-radius: 4px;
  font-size: 10px;
  font-weight: 500;
}

.invoice-tag.has-invoice {
  background: #f0f9eb;
  color: #67c23a;
  border: 1px solid #c2e7b0;
}

.invoice-tag.no-invoice {
  background: #fef0f0;
  color: #f56c6c;
  border: 1px solid #fbc4c4;
}

.flow-list :deep(.el-table th) {
  text-align: center;
}

.flow-list :deep(.el-table td) {
  text-align: center;
}

.flow-amount {
  font-size: 16px;
  font-weight: 700;
  white-space: nowrap;
  padding-top: 2px;
}

.flow-amount.in {
  color: #67c23a;
}

.flow-amount.out {
  color: #f56c6c;
}

.flow-amount.adv {
  color: #e6a23c;
}

/* 备忘录样式 */
.memo-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.memo-item {
  padding: 16px;
  background: #fafafa;
  border-radius: 8px;
  border: 1px solid #f0f0f0;
}

.memo-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.memo-title {
  font-size: 15px;
  font-weight: 600;
  color: #303133;
}

.memo-actions {
  display: flex;
  gap: 4px;
}

.memo-content {
  font-size: 14px;
  color: #606266;
  line-height: 1.6;
  white-space: pre-wrap;
  margin-bottom: 8px;
}

.memo-meta {
  font-size: 12px;
  color: #909399;
  display: flex;
  align-items: center;
  gap: 4px;
}

/* 表格列宽自适应 */
:deep(.el-table) {
  width: 100% !important;
}

:deep(.el-table__body-wrapper) {
  overflow-x: auto;
}

:deep(.el-table th .cell) {
  text-align: center;
}

:deep(.el-table td .cell) {
  white-space: nowrap;
  text-align: center;
}

.clickable-text {
  color: #409eff;
  cursor: pointer;
}

.clickable-text:hover {
  color: #66b1ff;
  text-decoration: underline;
}

.pagination-wrapper {
  display: flex;
  justify-content: flex-end;
  align-items: center;
  margin-top: 16px;
  padding: 12px 16px;
  background: #fafafa;
  border-radius: 8px;
  border: 1px solid #f0f0f0;
}

:deep(.el-pagination) {
  --el-pagination-bg-color: transparent;
  --el-pagination-text-color: #606266;
  --el-pagination-button-bg-color: #fff;
  --el-pagination-hover-color: #409eff;
}

:deep(.el-pagination .el-pager li) {
  border-radius: 8px;
  margin: 0 3px;
  min-width: 32px;
  height: 32px;
  line-height: 32px;
  font-weight: 500;
  transition: all 0.2s;
  border: 1px solid #e4e7ed;
}

:deep(.el-pagination .el-pager li:hover) {
  color: #409eff;
  border-color: #409eff;
}

:deep(.el-pagination .el-pager li.is-active) {
  background: linear-gradient(135deg, #409eff 0%, #66b1ff 100%);
  color: #fff;
  border-color: #409eff;
  box-shadow: 0 2px 8px rgba(64, 158, 255, 0.3);
}

:deep(.el-pagination .btn-prev, .el-pagination .btn-next) {
  border-radius: 8px;
  border: 1px solid #e4e7ed;
  background: #fff;
}

:deep(.el-pagination .btn-prev:hover, .el-pagination .btn-next:hover) {
  color: #409eff;
  border-color: #409eff;
}

:deep(.el-pagination__total) {
  color: #909399;
  font-size: 13px;
}

:deep(.el-pagination .btn-prev, .el-pagination .btn-next) {
  border-radius: 6px;
}

/* 列头筛选样式 */
.filter-header {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
}

.filter-icon {
  cursor: pointer;
  color: #a8abb2;
  font-size: 20px;
  padding: 4px;
  border-radius: 4px;
  transition: all 0.2s;
}

.filter-icon:hover {
  color: #409eff;
  background: rgba(64, 158, 255, 0.1);
}
</style>
