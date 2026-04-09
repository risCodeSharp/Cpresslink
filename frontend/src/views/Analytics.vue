<script lang="ts" setup>
import Card from '@/components/Card.vue';
import ClickOverTimeCard from '@/features/analytics/components/ClickOverTimeCard.vue';
import DateRangeNavbar from '@/features/analytics/components/DateRangeNavbar.vue';
import DeviceBreakdown from '@/features/analytics/components/DeviceBreakdown.vue';
import GeographicalReach from '@/features/analytics/components/GeographicalReach.vue';
import LinkDetails from '@/features/analytics/components/LinkDetails.vue';
import LinksAnalyticsRow from '@/features/analytics/components/LinksAnalyticsRow.vue';
import LinksSidebar from '@/features/analytics/components/LinksSidebar.vue';
import TopReferrers from '@/features/analytics/components/TopReferrers.vue';
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
            <div class="flex justify-between items-center">
                <h2 class="font-classic text-2xl py-1">{{ store.selectedLink?.name }}</h2>
                <div class="py-1 space-x-2">
                <button class="px-6  bg-white outline outline-emerald-500 rounded-lg">Share</button>
                <button class="px-6  bg-emerald-500 text-white outline outline-white rounded-lg">Configure</button>
                </div>
            </div>

            <LinkDetails class="mt-4" />
            <LinksAnalyticsRow class="mt-6" />
            <ClickOverTimeCard class="mt-6" />
            <div class="flex gap-4 mt-6">
                <TopReferrers />
                <GeographicalReach />
                <DeviceBreakdown />
            </div>
        </div>
    </div>
</template>
