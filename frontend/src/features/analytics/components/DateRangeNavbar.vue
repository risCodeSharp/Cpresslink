<script lang="ts" setup>
import { useDashboardStore } from '@/stores/dashboardStore';
import type { DateRange } from '@/types';

const store = useDashboardStore();
function selectRange(range: DateRange) {
    store.setDateRange(range);
    console.log(store.dateRange);
}

type RangeMap = Record<string, DateRange>;

const ranges: RangeMap =
{
    '7d': '7d',
    '30d': '30d',
    '90d': '90d',
    'All  Time': 'all'
};

function isRangeSelected(range: DateRange): boolean {
    const v =  store.dateRange === range;
    console.log(range, v)
    return v;
}

</script>

<template>
    <h1 class="mb-6 text-4xl font-classic">Analytics</h1>
    <div
        class="flex justify-between items-center bg-white px-6 py-4 shadow-around rounded-lg text-slate-500 text-medium-size">
        <h3>Date range:</h3>
        
        <ul class="flex justify-evenly gap-5">
            <li v-for="(value, key) in ranges"><button @click="selectRange(value)"
                class="px-3 cursor-pointer py-1 rounded-lg  outline  transition hover:-translate-y-1 hover:bg-emerald-200/30 focus:bg-emerald-50 focus:outline-emerald-400"
                :class="isRangeSelected(value) ? 'bg-emerald-50 outline-emerald-600' : 'bg-gray-100 outline-gray-300'"
                    >{{ key }}</button>
            </li>
        </ul>
        <button
            class="flex items-center bg-gray-800 px-3 py-1 rounded-lg text-white transition hover:scale-110 cursor-pointer delay-75 ease-in-out hover:translate-y-0.5">
            <span class="mr-1">
                <svg width="13" height="13" viewBox="0 0 16 16" fill="none">
                    <path d="M8 2v8M5 7l3 3 3-3M3 12v1a1 1 0 0 0 1 1h8a1 1 0 0 0 1-1v-1" stroke="currentColor"
                        stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"></path>
                </svg>
            </span>
            <span>Export CSV</span>
        </button>
    </div>
</template>

<style lang="css" scoped>
.text-medium-size {
    font-size: 0.8rem;
}

.text-normal-size {
    font-size: 0.7rem;
}

.text-small-size {
    font-size: 0.7rem;
}
</style>