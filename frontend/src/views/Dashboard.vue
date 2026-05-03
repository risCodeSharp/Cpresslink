<script lang="ts" setup>
import { ref } from 'vue';

import ClicksDetailsRow from '@/features/dashboard/components/ClicksDetailsRow.vue';
import AllLinks from '@/features/dashboard/components/AllLinks.vue';
import NetworkTraffic from '@/features/dashboard/components/NetworkTraffic.vue';
import TopPerformers from '@/features/dashboard/components/TopPerformers.vue';
import Header from '@/components/Header.vue';

import CreateLink from '@/features/dashboard/components/CreateLink.vue';
import { shortlinkService } from '@/services/shortlinkService';

const trying = async () => {
    let links = await shortlinkService.list();
    console.log(links);
}

</script>
<template>
    <Header class="mb-10" />
    <main class="mx-10 lg:w-5xl lg:mx-auto">

        <h1 class="mb-6 text-3xl font-classic">Link Dashboard</h1>
        <button @click="trying">try</button>
        <CreateLink />
        
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