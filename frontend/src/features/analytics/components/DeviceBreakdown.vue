<script lang="ts" setup>
import type { ChartData, ChartOptions, Plugin } from 'chart.js';
import {Chart as ChartJS, ArcElement, Tooltip, Legend } from 'chart.js';
import { dateCsCZ, NCard } from 'naive-ui';
import { computed, ref, type ComputedRef, type Ref } from 'vue';
import { Doughnut } from 'vue-chartjs';

ChartJS.register(ArcElement, Tooltip, Legend);

interface DeviceData {
    name: string,
    color: string,
    percent: number
}

const devicesUsedData: Ref<DeviceData[]> = ref([
    {name: 'Mobile', color: '#335C3E', percent: 65},
    {name: 'Desktop', color: '#7FB086', percent: 28},
    {name: 'Tablet', color: '#89C9BC', percent: 7},
]);

const highestDevicePercent: ComputedRef<number>= computed(() => Math.max(...devicesUsedData.value.map(d => d.percent)));

const donutData: ChartData<'doughnut'> = {
    labels: ['Mobile','Desktop','Tablet' ],
    datasets: [
        {
            backgroundColor: devicesUsedData.value.map(d => d.color),
            data: devicesUsedData.value.map(d => d.percent),
            borderWidth: 0,
            borderRadius: 2,
        },
    ]
}

const centerTextPlugin: Plugin<'doughnut'> = {
    id: 'centerText',
    beforeDraw: (chart: any) => {
        const {ctx, width, height} = chart;
        ctx.restore();
        const fontSize = (height / 65).toFixed(2);
        ctx.font = `${fontSize}rem Instrument Serif` ;
        ctx.textBaseline = 'middle';
        ctx.fillStyle = '#333';
        
        const text = String(highestDevicePercent.value) + '%';
        const textX = Math.round((width - ctx.measureText(text).width) / 2);
        const textY = height / 2;
        ctx.fillText(text, textX, textY);
        ctx.save();
    }
}


const donutOptions: ChartOptions<'doughnut'> = {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
        legend: {display: false},
    },
    cutout: '70%',
    
}


</script>

<template>
   <NCard class="pb-2 rounded-card shadow-around max-w-50">
    <h2 class="text-[0.9rem] font-semibold mb-6">Device Breakdown</h2>
        <Doughnut class="max-h-20 mb-6" :data="donutData" :options="donutOptions" :plugins="[centerTextPlugin]" />

        <ul v-for="device in devicesUsedData">
            <li class="text-medium-size flex  justify-between gap-3 w-full items-center">
                <div class="space-x-2 text-slate-700">
                    <span class="inline-block w-2 h-2 rounded  " :style="{ background: device.color}"></span>
                    <span class="">{{ device.name }}</span>
                </div>
                <span class="text-end text-slate-500/90 ">{{ device.percent }}%</span>
            </li>
        </ul>

    </NCard>
</template>