<script lang="ts" setup>
import { NCard, NSpace} from 'naive-ui';
import type { DataTableColumns } from 'naive-ui'
import { NButton, NTag, useMessage } from 'naive-ui'
import { reactive, type Reactive } from 'vue';

type Status = 'Paid' | 'Pending';

interface UsageRowData {
    key: number,
    date: string,
    amount: string,
    status: Status,
    invoiceType: string,
}



</script>

<template>
    <NCard class="rounded-card shadow-around my-6">
        <template #header>
            <!-- Refactor remove flex later -->
            <div class="flex text-normal-size justify-between items-center">
                <div>
                    <div class="uppercase tracking-widest text-normal-size text-slate-400 mb-0.5 ">History</div>
                    <h2 class=" text-[18px] font-semibold text-slate-900">Invoices</h2>
                </div>
                <button class="px-5 py-1.5 text-gray-600 outline outline-gray-300 hover:text-gray-800 hover:outline-gray-500 hover:bg-gray-50 cursor-pointer rounded-lg">Download All</button>
            </div>
        </template>

        
    </NCard>
    <!-- <NSpace vertical :size="12">
        <n-data-table />
    </NSpace> -->

</template>
<!-- 
later <script lang="ts" setup>
import { NCard, NSpace, NDataTable, NTag, NButton } from 'naive-ui'
import type { DataTableColumns } from 'naive-ui'
import { h, reactive } from 'vue'

type Status = 'Paid' | 'Pending'

interface UsageRowData {
  key: number
  period: string
  amount: string
  status: Status
  invoiceType: string
}

const rowData = reactive<UsageRowData[]>([
  {
    key: 1,
    period: 'Jun 2024',
    amount: '$29.00',
    status: 'Paid',
    invoiceType: 'PDF'
  },
  {
    key: 2,
    period: 'May 2024',
    amount: '$29.00',
    status: 'Paid',
    invoiceType: 'PDF'
  },
  {
    key: 3,
    period: 'Apr 2024',
    amount: '$29.00',
    status: 'Pending',
    invoiceType: 'PDF'
  }
])

const columns: DataTableColumns<UsageRowData> = [
  {
    title: 'Billing Period',
    key: 'period'
  },
  {
    title: 'Amount',
    key: 'amount'
  },
  {
    title: 'Status',
    key: 'status',
    render(row) {
      return h(
        NTag,
        {
          type: row.status === 'Paid' ? 'success' : 'warning',
          round: true
        },
        { default: () => row.status }
      )
    }
  },
  {
    title: 'Invoice',
    key: 'invoiceType',
    render() {
      return h(
        NButton,
        { size: 'small', text: true },
        { default: () => 'PDF' }
      )
    }
  }
]
</script>

<template>
  <NCard class="rounded-card shadow-around my-6">
    <template #header>
      <div class="flex justify-between items-center">
        <div>
          <div class="uppercase tracking-widest text-slate-400 mb-0.5">
            History
          </div>
          <h2 class="text-[18px] font-semibold text-slate-900">
            Invoices
          </h2>
        </div>
        <button
          class="px-5 py-1.5 text-gray-600 outline outline-gray-300 hover:text-gray-800 hover:outline-gray-500 hover:bg-gray-50 cursor-pointer rounded-lg"
        >
          Download All
        </button>
      </div>
    </template>

    <NSpace vertical :size="12">
      <NDataTable
        :columns="columns"
        :data="rowData"
        :bordered="false"
      />
    </NSpace>
  </NCard>
</template>

<style scoped>
.rounded-card {
  border-radius: 12px;
}
</style>
-->