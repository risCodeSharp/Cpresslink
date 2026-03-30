<script lang="ts" setup>
import Card from '@/components/Card.vue';
import InputField from '@/components/InputField.vue';

</script>
<template>
    <div class="m-6 lg:w-7xl lg:mx-auto">
        <h1 class="my-6 text-4xl font-classic">Link Dashboard</h1>

        <Card class="my-4">
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

        <div class="grid grid-cols-4 gap-4">
            <Card>
                <h3 class="text-tiny uppercase tracking-wider">Total Clicks</h3>
                <p class="my-2 text-gray-700 text-3xl font-classic">12.4k</p>
            </Card>
            <Card>
                <h3 class="text-tiny uppercase tracking-wider">Avg. Ctr</h3>
                <p class="my-2 text-gray-700 text-3xl font-classic">4.82%k</p>
            </Card>
            <Card>
                <h3 class="text-tiny uppercase tracking-wider">Active Links</h3>
                <p class="my-2 text-gray-700 text-3xl font-classic">42</p>
            </Card>
            <Card>
                <h3 class="text-tiny uppercase tracking-wider">Node uptime</h3>
                <p class="my-2 text-gray-700 text-3xl font-classic">99.98%</p>
            </Card>
        </div>

        <div>newtraffic barchart</div>
        <div>
            top performers links lists
        </div>

        <div></div>
    </div>
</template>

<style>
.text-tiny {
    font-size: 0.72rem;
}
</style>