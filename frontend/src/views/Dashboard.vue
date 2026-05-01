<script lang="ts" setup>
import { ref } from 'vue';
import Card from '@/components/Card.vue';
import InputField from '@/components/InputField.vue';

import ClicksDetailsRow from '@/features/dashboard/components/ClicksDetailsRow.vue';
import AllLinks from '@/features/dashboard/components/AllLinks.vue';
import NetworkTraffic from '@/features/dashboard/components/NetworkTraffic.vue';
import TopPerformers from '@/features/dashboard/components/TopPerformers.vue';
import Header from '@/components/Header.vue';
import { NCard, NTab, NTabPane, NTabs } from 'naive-ui';

</script>
<template>
    <Header class="mb-10" />
    <main class="mx-10 lg:w-5xl lg:mx-auto">

        <h1 class="mb-6 text-3xl font-classic">Link Dashboard</h1>

        <NCard size="medium" class="m-1">
            <!-- <h2 class="text-[0.72rem] uppercase tracking-wide mb-2">
                Create new Link
            </h2> -->

            <NTabs type="line" animated >
                <NTabPane name="short-code">
                    <div class="flex gap-4 p-2">
                    <section class="col-span-3 w-full">
                        <span class="text-tiny mr-1.5">destination</span>
                        <InputField placeholder="Paste your long URL here..." class=" py-2" />
                    </section>
                    <div class="flex items-end">
                        <button class=" cursor-pointer bg-gray-800 col-span-1 max-w-30 w-24 h-10 text-white py-2 rounded-lg">Shorten →</button>
                    </div>
                    </div>
                </NTabPane>
                <NTabPane name="custom-link">
                <div class="grid grid-cols-7 gap-4 w-full p-1 text-stone-500/85">
                    <div class="col-span-3">
                        <span class="text-tiny mr-1.5">destination</span>
                        <InputField placeholder="Paste your long URL here..." class=" py-2" />
                    </div>
                    <div class="col-span-3">
                        <span class="text-tiny mr-1.5">cpress.link/</span>
                        <InputField placeholder="custom-slug (optional)" class="py-2"/>
                    </div>
                    <div class="flex items-end">
                        <button class=" bg-gray-800 col-span-1 max-w-30 w-24 h-10 text-white py-2 rounded-lg">Shorten →</button>
                    </div>
                </div>
                </NTabPane>
            </NTabs>
        </NCard>

        <ClicksDetailsRow />
        <div class="flex justify-around my-6 gap-6 h-80">
            <NetworkTraffic class="flex-6" />
            <TopPerformers class="flex-5" />
        </div>
        <AllLinks />
    </main>

</template>

<style>
.text-tiny {
    font-size: 0.72rem;
}
</style>