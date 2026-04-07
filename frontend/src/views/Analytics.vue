<script lang="ts" setup>
import Card from '@/components/Card.vue';
import ClickOverTimeCard from '@/features/analytics/components/ClickOverTimeCard.vue';
import DateRangeNavbar from '@/features/analytics/components/DateRangeNavbar.vue';
import LinkDetails from '@/features/analytics/components/LinkDetails.vue';
import LinksAnalyticsRow from '@/features/analytics/components/LinksAnalyticsRow.vue';
import LinksSidebar from '@/features/analytics/components/LinksSidebar.vue';
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

            <LinkDetails class="mb-t"/>
            <LinksAnalyticsRow class="mt-6"/>
            <ClickOverTimeCard class="mt-6"/>
            <div class="flex gap-4 mt-6">
                <Card class="w-full">
                    Top Reffers
                </Card>
                <Card class="w-full">
                    Device Breakdown
                </Card>
                <Card class="w-full">
                    Geographical Reach
                </Card>
            </div>
        </div>
    </div>
</template>
