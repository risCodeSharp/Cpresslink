<script lang="ts" setup>
import Card from '@/components/Card.vue';
import { Chart as ChartJS, Title, Tooltip, LineElement, CategoryScale, LinearScale, PointElement, Filler, type ChartOptions, type ChartData } from "chart.js";
import { onMounted, ref, type Ref } from 'vue';
import { Line } from 'vue-chartjs';

// we are discard the fill value but it is required so we have mentioned it 
const labels: Array<string> = Array.from({ length: 30 }, (_, i) =>
    i === 29
        ? "Today"
        : `May ${i + 1}`);
console.log(labels)

// const data: number[] = Array.from({ length: 30} , () => 
//     Math.floor(Math.random() * (300 - 200 + 1)) + 100 
// );
const data: number[] = [
  526, 542, 513, 488, 565, 612, 690, 733, 769, 812, 
  850, 787, 721, 680, 595, 529, 442, 415, 437, 503, 
  580, 643, 744, 821, 899, 927, 991, 930, 808, 717
];

ChartJS.register(Title, Tooltip, LineElement, CategoryScale, LinearScale, PointElement, Filler);

const chartData = ref<ChartData<'line'>>({
  labels: [],
  datasets: []
});
const chartOptions: ChartOptions<'line'> = {
    responsive: true,
    plugins: { legend: { display: false }, tooltip: { enabled: true } },
    scales: {
        x: {
            grid: { display: false },
            ticks: {
                autoSkip: false,
                callback: (_, index) => {
                    if (index === 0 || index === 9 || index === 19 || index === 29) {
                        return labels[index];
                    }
                    return null;
                }
            }
        },
        y: {
            display: true,
            grid: {
                color: "rgba(0, 0, 0, 0.05)", // Light grid lines for readability
                
            }
        }
    }

}

const pointRadius = data.map((v, i) => v === 1 || v === 10 || v === 20 || v === 30 ? 3: 0);
onMounted(() => {
    const canvas = document.querySelector("canvas") as HTMLCanvasElement;
    const ctx = canvas.getContext("2d");

    if (!ctx) return;

    // Creating a vetical gradient from top to bottom
 const gradient = ctx.createLinearGradient(0, 0, 0, 200);
  gradient.addColorStop(0, "rgba(16, 185, 129, 0.2)");   
  gradient.addColorStop(0.4, "rgba(50, 185, 180, 0.1)"); 
  gradient.addColorStop(1, "rgba(255, 255, 255, 0)");    
    chartData.value = {
        labels, 
        datasets: [
            {
            label: "Clicks",
            data,
            borderColor: "#024b30",
            backgroundColor: gradient,
            fill: true,
            tension: 0.2,
            pointRadius,
            // pointBackgroundColor: 
            }
        ]
    }
})

</script>
<template>
    <Card>
        <div>
            <div class="mb-6 flex justify-between rounded-lg p-5">
                <div>
                    <p class="text-[0.88rem] text-gray-800 font-semibold">Clicks Over Time</p>
                    <p class="text-[0.78rem]">Performance metrics for the last 30 days</p>
                </div>
                <div class="text-end">
                    <p class="font-classic text-[1.7rem] text-gray-600">3,241</p>
                    <p class="text-normal-size text-emerald-500">+14.2% growth</p>
                </div>
            </div>
            <div class="mx-3">
                <Line class="max-h-50 w-5/6" :data="chartData" :options="chartOptions" />
            </div>
        </div>
    </Card>
</template>
