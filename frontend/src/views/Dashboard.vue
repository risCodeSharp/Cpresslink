<script lang="ts" setup>
import { ref } from 'vue';
import Card from '@/components/Card.vue';
import InputField from '@/components/InputField.vue';

import ClicksDetailsRow from '@/features/dashboard/components/ClicksDetailsRow.vue';
import AllLinks from '@/features/dashboard/components/AllLinks.vue';
import NetworkTraffic from '@/features/dashboard/components/NetworkTraffic.vue';
import TopPerformers from '@/features/dashboard/components/TopPerformers.vue';

</script>
<template>

    <h1 class="mb-6 text-3xl font-classic">Link Dashboard</h1>

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

    <ClicksDetailsRow />
    <div class="flex justify-around my-6 gap-6 h-80">
        <NetworkTraffic  class="flex-6"/>
        <TopPerformers class="flex-5"/>
    </div>
    <AllLinks />

</template>

<style>
.text-tiny {
    font-size: 0.72rem;
}
</style>