<script lang="ts" setup>
import WeekTrafficBarChart from '@/components/WeekTrafficBarChart.vue';
import { NCard } from 'naive-ui';
</script>

<template>
    <div class="grid grid-cols-4 gap-4 my-6">
        <NCard class="shadow-around rounded-card">
            <h3 class="text-[0.7rem] uppercase tracking-wider">Total Clicks</h3>
            <p class="my-2 text-gray-700 text-3xl font-classic">12.4k</p>
            <WeekTrafficBarChart class="h-10" />
        </NCard>

        <NCard class="shadow-around rounded-card dark-green-card" style="color: white">
            <h3 class="text-[0.7rem] text-white/70 uppercase tracking-wider">Avg. Ctr</h3>
            <p class="my-2 text-3xl font-classic">4.82%</p>
            <p class="text-normal-size tracking-wider text-green-300/80">+0.4 from yesterday</p>
        </NCard>
        
        <NCard class="shadow-around rounded-card">
            <h3 class="text-[0.7rem] uppercase tracking-wider">Active Links</h3>
            <p class="my-2 text-gray-700 text-3xl font-classic">42</p>
            <p class="text-normal-size text-gray-500/89">3 added today</p>
        </NCard>
        
        <NCard class="shadow-around rounded-card">
            <h3 class="text-[0.7rem] uppercase tracking-wider">Node uptime</h3>
            <p class="my-2 text-gray-700 text-3xl font-classic">99.98%</p>
            <p class="text-normal-size text-green-600/88"><span class="w-1.5 h-1.5  mr-1 bg-green-600/88 rounded-full inline-block"></span>Operational</p>
        </NCard>
    </div>
    
</template>