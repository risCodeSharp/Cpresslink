<script lang="ts" setup>
import {ref} from 'vue';
import Card from '@/components/Card.vue';
import InputField from '@/components/InputField.vue';
import WeekTrafficBarChart from '@/components/WeekTrafficBarChart.vue';

</script>
<template>
    
        <h1 class="mb-6 text-4xl font-classic">Link Dashboard</h1>

        <Card class="my-4 px-4 py-5">
            <h2 class="uppercase tracking-widest text-tiny ">
                short a new link
            </h2>

            <div class="grid grid-cols-6 gap-4 w-full">

                <InputField placeholder="Paste your long URL here..." class="col-span-3" />
                <p class="flex justify-center items-center col-span-2">
                    <span class="text-tiny mr-1.5">cpress.link/</span>
                    <InputField placeholder="custom-slug (optional)" />
                </p>
                <button class="bg-black col-span-1 text-white py-2 rounded-lg">Shorten →</button>
            </div>
        </Card>

        <div class="grid grid-cols-4 gap-4 my-6">
            <Card class="p-6">
                <h3 class="text-tiny uppercase tracking-wider">Total Clicks</h3>
                <p class="my-2 text-gray-700 text-4xl font-classic">12.4k</p>
            <WeekTrafficBarChart class="h-10" />

            </Card>
            <Card class="p-6">
                <h3 class="text-tiny uppercase tracking-wider">Avg. Ctr</h3>
                <p class="my-2 text-gray-700 text-4xl font-classic">4.82%</p>
            </Card>
            <Card class="p-6">
                <h3 class="text-tiny uppercase tracking-wider">Active Links</h3>
                <p class="my-2 text-gray-700 text-4xl font-classic">42</p>
            </Card>
            <Card class="p-6">
                <h3 class="text-tiny uppercase tracking-wider">Node uptime</h3>
                <p class="my-2 text-gray-700 text-4xl font-classic">99.98%</p>
            </Card>
        </div>
        <div class="flex justify-around my-6 gap-6">
        <Card class="shadow-around flex-4">newtraffic barchart</Card>
        <Card class="shadow-around flex-3">
            top performers links lists
        </Card>
    </div>
    
    <Card>
        <h2 class="font-classic text-xl">All Links</h2>
    </Card>
        
    
</template>

<style>
.text-tiny {
    font-size: 0.72rem;
}
</style>