<script lang="ts" setup>
import Card from '@/components/Card.vue';

</script>

<template>
    <div class=" w-full flex gap-4 py-6">
        <Card class="w-full py-4 px-6 flex flex-col justify-around gap-0.5 max-h-full">
            <h4 class="text-slate-400 uppercase text-small-size tracking-wider">Total Clicks</h4>
            <p class="text-gray-800 font-classic text-[1.7rem] ">3,241</p>
            <p class="text-green-600 text-normal-size font-light">+14.2% growth</p>
        </Card>
        <Card class="w-full py-4 px-6 flex flex-col justify-around gap-0.5 max-h-full">

            <h4 class="text-slate-400 uppercase text-small-size tracking-wider">AVG. CTR</h4>
            <p class="text-gray-700 font-classic text-[1.7rem]">4.82%</p>
            <p class="text-green-600 text-normal-size font-light">+0.4% from yesterday</p>
        </Card>
        <Card class="w-full py-4 px-6 flex flex-col justify-around gap-0.5 max-h-full">
            <h4 class="text-slate-400 uppercase text-small-size tracking-wider">TOP SOURCE</h4>
            <p class="text-gray-700 font-classic text-xl ">Twitter / X</p>
            <p class="text-green-600 text-normal-size font-light">34% of total traffic</p>
        </Card>
    </div>
</template>