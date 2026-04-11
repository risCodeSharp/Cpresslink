<script lang="ts" setup>
import { NCard } from 'naive-ui';


</script>

<template>
    <NCard class="shadow-around card-rounded">
        <template #header>
            <!-- Refactor remove flex later -->
            <div class="flex text-normal-size justify-between items-center">
                <div>
                    <div class="uppercase tracking-widest text-normal-size text-slate-400 mb-0.5 ">SUBSCRIPTIONS</div>
                    <h2 class=" text-[18px] font-semibold text-slate-900">Scale Your Experience</h2>
                </div>
                <button class="px-5 py-1.5 text-gray-600 outline outline-gray-300 hover:text-gray-800 hover:outline-gray-500 hover:bg-gray-50 cursor-pointer rounded-lg">Download All</button>
            </div>
        </template>
    </NCard>
</template>