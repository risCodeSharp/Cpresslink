<script lang="ts" setup>
import { NCard, NDataTable } from 'naive-ui';
import type { DataTableColumns, } from 'naive-ui'
import { NButton, NTag, useMessage } from 'naive-ui'
import { h, reactive, type Reactive } from 'vue';

type Status = 'PAID' | 'PENDING';

interface InvoiceData {
  date: string,
  subscription: string,
  amount: string,
  status: Status,
  download_url: string,
}
const invoiceData: Reactive<InvoiceData[]> = reactive([
  {
    date: "Jun 1, 2024",
    subscription: "Start Plan",
    amount: "Free",
    status: "PAID",
    download_url: "",
  },
  {
    date: "May 1, 2024",
    subscription: "Pro Plan (Monthly)",
    amount: "$29.00",
    status: 'PAID',
    download_url: "",
  },
  {
    date: "Apr 1, 2024",
    subscription: "Pro Plan (Monthly)}",
    amount: "$29.00",
    status: 'PENDING',
    download_url: "",
  },
  {
    date: "Mar 1, 2024",
    subscription: "Start Plan -> Pro Plan(Monthly) Upgraded",
    amount: "$19.35",
    status: "PAID",
    download_url: "",
  },
]);
/**
 *     date: string,
    subscription: string,
    amount: string,
    status: Status,
    download_url: string,
 */
const invoiceColumns: DataTableColumns<InvoiceData> = [
  {
    title: 'Date',
    key: 'date',
    render(row: InvoiceData) {
      return h('div', { class: "text-nowrap" }, row.date)
    }
  },
  {
    title: 'Choosen Subscription',
    key: 'subscription'
  },
  {
    title() {
      return h('h4', { class: 'text-center' }, 'Amount')
    },
    key: 'amount',
    render(row: InvoiceData) {
      return h('div', { class: "text-nowrap text-center" }, row.amount)
    }

  },
  {
    title() {
      return h('h4', { class: 'text-center' }, 'Status')
    },
    key: 'status',
    render(row: InvoiceData) {
      const color = row.status === 'PAID' ? 'bg-green-600/12 text-green-700' : 'bg-rose-200/67 text-rose-600'
      return h('div', { class: `text-nowrap text-center px-2 w-min mx-auto ${color} rounded-full  font-semibold` }, row.status)
    }
  },
  {
    title() {
      return h('h4', { class: 'text-end' }, 'Download')
    },
    key: 'download',
    render(row: InvoiceData) {
      return h('div', { class: "w-full flex justify-end pr-1" }, [
        h('a', { class: "text-nowrap cursor-pointer text-stone-500/80 hover:text-stone-800", title: 'PDF Download', href: row.download_url }, "↓ PDF")
      ]);
    }
  },
];

</script>

<template>
  <NCard class="rounded-card shadow-around">
    <template #header>
      <!-- Refactor remove flex later -->
      <div class="flex text-normal-size justify-between items-center">
        <div>
          <div class="uppercase tracking-wider text-[0.58rem] text-stone-400 mb-0.5 ">History</div>
          <h2 class=" text-[1rem] font-semibold text-slate-900">Invoices</h2>
        </div>
        <button
          class=" transition-colors duration-200 ease-linear cursor-pointer bg-stone-300/70 hover:bg-stone-800 hover:text-white  text-[0.68rem]  py-1 px-3 rounded-full">
          Download All
        </button>

      </div>
    </template>
    <NDataTable :columns="invoiceColumns" style="font-size: 0.7rem;;" :data="invoiceData" />


  </NCard>
  <!-- <NSpace vertical :size="12">
        <n-data-table />
    </NSpace> -->

</template>
