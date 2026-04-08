<script lang="ts" setup id="networkTrafficScript">
import { Bar } from 'vue-chartjs';
import { Chart as ChartJS, Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale } from 'chart.js';
import { ref, computed, type ComputedRef } from 'vue';
import type { ChartData, ChartOptions, ScriptableContext } from 'chart.js';

const mostBarColor = '#2e8b57';
const normalBarColor = '#C7E3CC';

const totalclickPast5Months: number[] = [100, 200, 500, 300, 40];
const mostClicked: ComputedRef<number> = computed(() => Math.max(...totalclickPast5Months));
  const barBackgroundColor: ComputedRef<string[]> = computed(() => totalclickPast5Months.map((click: number) => click === mostClicked.value ? mostBarColor : normalBarColor));
  

ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale)



const chartData = ref<ChartData<'bar', number[], string>>({
  labels: ['', '', '', '', ''],
  datasets: [{
    data: totalclickPast5Months,
    barThickness: 'flex',
    backgroundColor: barBackgroundColor.value,
    borderRadius: 2.5,
    barPercentage: 1.15,
    borderColor: '#9CCAA7',
    borderWidth: 1.1,
    borderSkipped: false,
  }],
});


const percentageChange: ComputedRef<number> = computed(() => {
  const currentMonth: number = totalclickPast5Months[totalclickPast5Months.length - 1]!;
  const pastMonth: number = totalclickPast5Months[totalclickPast5Months.length - 2]!;
  if (pastMonth === 0) {
    return 0;
  }

  const change = (((currentMonth - pastMonth) / pastMonth) * 100);
  console.log(change)
  return parseFloat(change.toPrecision(4));
});



const chartOptions = ref<ChartOptions<'bar'>>({
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: { display: false },
    tooltip: { enabled: false },
  },
  scales: {
    x: { display: false, grid: { display: false } },
    y: { display: false, grid: { display: false } },
  },
});

</script>

<template id="networkTrafficTemplate">

  <p class="my-3  text-emerald-900/80">{{ (percentageChange > 0) ? "↑ +" : "↓ " }} {{ percentageChange }} this month</p>
  <div class="h-8">
    <Bar :data="chartData" :options="chartOptions" />
  </div>
</template>

<style scoped>
p {
  font-size: 0.75rem;
}
</style>