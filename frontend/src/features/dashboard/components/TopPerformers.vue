<script lang="ts" setup>
import { NCard, NScrollbar, NSelect, NTable, type DataTableColumn } from 'naive-ui';
import type { SelectMixedOption, SelectOption } from 'naive-ui/es/select/src/interface';
import { ref, type Ref } from 'vue';
import { useMessage } from 'naive-ui';
type FilterOption = 'week' | 'month' | 'threeMonth'

const filterValue: Ref<FilterOption> = ref('week');
const filterOptions: SelectOption[] = [
    {
        label: 'This week',
        value: 'week',
    },
    {
        label: 'This month',
        value: 'month',
    },
    {
        label: 'Last Three Months',
        value: 'threeMonth',
    },
];

interface LinkPerformance {
    name: string,
    url: string,
    clicks: number,
    ctr: number,
}

const message = useMessage();

const links: Ref<LinkPerformance[]> = ref(
    [
        { name: "Summer Campagin 24", url: "cpress.link/summer-24", clicks: 3241, ctr: 4.82 },
        { name: "Product Launch v2", url: "cpress.link/p-launch", clicks: 1890, ctr: 3.21 },
        { name: "Black Friday Sale", url: "cpress.link/bf-24", clicks: 6120, ctr: 6.44 },
    ]
);

async function copyText(text: string) {
    try {
        await navigator.clipboard.writeText(text);
        message.success('Copied text: ' + text);
    } catch (err: unknown) {
        message.error("Failed to copy text!");
    }
}


</script>
<template>
    <NCard class="rounded-card shadow-around">
        
        <template #header>
            <div class="flex justify-between ">
                
                <h2 class="text-[0.88rem] ">Top Performers</h2>
                <NSelect v-model="filterValue" :options="filterOptions" class=" rounded-card select-style"
                    size="small" />
            </div>
        </template>
        <NScrollbar x-scrollable style="max-height: 200px; ">
        <div class="w-full max-w-2xl mx-auto ">
            <table class="w-full border-collapse text-[0.68rem]">
                <thead>
                    <tr class="text-gray-600/75 tracking-wider">
                        <th class=" font-normal  text-left pb-2 px-2">LINK</th>
                        <th class=" font-normal  text-right pb-2 px-2">CLICKS</th>
                        <th class=" font-normal  text-right pb-2 px-2">CTR</th>
                        <th class="pb-2"></th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="link in links"
                        class="border-y-2  border-gray-200 px-3 last:border-0 font-medium hover:bg-emerald-900/6">
                        <td class="py-2 ">
                            <div>
                                <p class="text-base text-gray-800 text-[0.78rem]">{{ link.name }}</p>
                                <p class="text-slate-600">{{ link.url }}</p>
                            </div>
                        </td>
                        <td class="py-2 px-2 text-lg text-end font-classic">{{ link.clicks }}</td>
                        <td class="py-2 px-2 text-right text-sm">{{ link.ctr }}%</td>
                        <td class="py-2 pl-1">
                            <button @click="copyText(link.url)" class=" transition-colors duration-200 ease-in-out cursor-pointer text-green-900/70 bg-emerald-600/10 hover:bg-emerald-800 hover:text-white border border-emerald-800/15 p-2 rounded-lg">
                                
                                <svg class=" flex justify-center items-center w-3 h-3"  width="13" height="13" viewBox="0 0 16 16"
                                    fill="none">
                                    <rect x="5" y="5" width="9" height="9" rx="2" stroke="currentColor"
                                        stroke-width="1.4">
                                    </rect>
                                    <path d="M3 11H2a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1h8a1 1 0 0 1 1 1v1"
                                        stroke="currentColor" stroke-width="1.4" stroke-linecap="round"></path>
                                </svg>
                            </button>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
    </NScrollbar>
    </NCard>
</template>

<style scoped>
.select-style {
    width: 160px;
    font-size: 0.88rem;
    border-radius: 50px;
}
</style>
