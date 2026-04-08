<script lang="ts" setup>
import type { ChartData, ChartOptions } from 'chart.js';
import { NCard } from 'naive-ui';
import {
    Chart as ChartJS,
    Title,
    Tooltip,
    LineElement,
    Filler,
    PointElement,
    CategoryScale,
    LinearScale
} from 'chart.js';
import { Line } from 'vue-chartjs';
import { nextTick, onMounted, onUnmounted, ref, type Ref } from 'vue';

ChartJS.register(Title, Filler, Tooltip, LineElement, PointElement, CategoryScale, LinearScale);

const days = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
const today: number = new Date().getDay() // start from 0 -> sun
const dayData = [856, 834, 700, 454, 654, 777, 990];

const data: Array<number | null> = dayData.map((val, i) =>
    (i <= today) ? val : null
);

const labels = days.map((val, i) =>
    (i === today) ? 'Today' : val
);
const chartData: Ref<ChartData<'line'>> = ref({

    labels: labels,
    datasets: [
        {
            label: 'traffic',
            data,
            borderColor: "#10b981",
            backgroundColor: 'rgba(16, 185, 129, 0.15)',
            borderWidth: 2.5,
            tension: 0.4,
            fill: true,
            pointRadius: 3,
            pointBackgroundColor: "#ffffff",
            pointBorderColor: "#10b981",
            pointBorderWidth: 2,
            pointHoverRadius: 7,
            pointHoverBackgroundColor: "#50C878",
            // ----------------------------
        }
    ]
});

const chartOptions: ChartOptions<'line'> = {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
        legend: { display: false }
    },
    scales: {
        x: { grid: { color: 'rgba(0, 0, 0, 0.05)' } },
        y: {
            grid: { color: 'rgba(0, 0, 0, 0.05)' },
            border: { display: false }
        }
    }
};

const lineChartRef = ref<any>(null);

const handleResize = () => {
    nextTick(() => {
        if (lineChartRef.value && lineChartRef.value.chart) {
            lineChartRef.value.chart.resize();
        }
    });
};

onMounted(() => {
    window.addEventListener('resize', handleResize);
});

onUnmounted(() => {
    window.removeEventListener('resize', handleResize);
});

</script>
<template>
    <NCard class="rounded-card shadow-around  max-w-300">
        <template #header>
            <div class="flex justify-between">
                <div>
                    <h2 class="text-[0.88rem]">Network Traffic</h2>
                    <p class="text-[0.78rem]">Real-time engagemtn acros active links</p>
                </div>
                <button class="text-[0.78rem] cursor-pointer">Full Report →</button>
            </div>
        </template>
        <div class="w-full">
            <Line ref="lineChartRef" class="w-full transition-all h-55" :data="chartData" :options="chartOptions" />
        </div>
    </NCard>
</template>