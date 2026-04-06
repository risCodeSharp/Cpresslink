<script lang="ts" setup>
import Card from '@/components/Card.vue';
import ClickOverTimeCard from '@/components/ClickOverTimeCard.vue';
import DateRangeNavbar from '@/components/DateRangeNavbar.vue';
import LinkDetails from '@/components/LinkDetails.vue';
import LinksSidebar from '@/components/LinksSidebar.vue';
import { useDashboardStore } from '@/stores/dashboardStore';
import { NCard } from 'naive-ui';
const store = useDashboardStore();
</script>
<!-- later https://brand.com/collections/summer-2024-->
<!-- later \max-width:200px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;-->

<template>
    <DateRangeNavbar />
    <div class="flex my-6 gap-6">
        <LinksSidebar class="w-60 " />
        <div class="w-full">
            <p class="my-2 uppercase font-extralight text-normal-size tracking-widest"><span
                    class="inline-block w-2 h-2 rounded-full bg-green-500 mr-2 "></span>Live Data</p>
            <h2 class="font-classic text-2xl">{{ store.selectedLink?.name }}</h2>

            <LinkDetails />
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
            <ClickOverTimeCard />
            <div class="flex">
                <Card>
                    Top Reffers
                </Card>
                <Card>
                    Device Breakdown
                </Card>
                <Card>
                    Geographical Reach
                </Card>
            </div>
        </div>
    </div>
</template>
