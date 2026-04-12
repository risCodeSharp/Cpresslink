<script lang="ts" setup>
import { NCard } from 'naive-ui';


</script>

<template>
    <NCard class="shadow-around rounded-card">
        <template #header>
            <div class="flex w-full justify-between items-center text-normal-size ">
                <div>
                    <div class="uppercase tracking-widest text-[0.58rem] text-slate-400 mb-0.5 ">Connected Apps</div>
                    <h2 class=" text-[0.95rem] font-semibold text-slate-700">Integrations</h2>
                </div>
            </div>
        </template>
        <ul>
            <li v-for="i in 2" class="flex gap-2 py-4 border-b border-b-stone-200/87 items-center text-[0.69rem] w-full justify-between ">
                <section class="flex gap-2">
                    <div class="p-1.25  bg-blue-100 rounded-lg h-min text-[1rem]">📊</div>
                    <div>
                        <h3 class="text-[0.82rem] font-medium">Google Analytics 4</h3>
                        <p class=" leading-2.5 text-stone-500/80">Push click events to GA4 in real time</p>
                    </div>
                </section>
                <div class="flex text-[0.68rem] space-x-3">
                    <p class="px-2 py-1 bg-emerald-500/12 text-emerald-800 rounded-full font-semibold">Connected</p>
                    <button class="px-3 border border-stone-400/60 cursor-pointer text-stone-500/95 rounded-lg">Disconnect</button>
                </div>
            </li>
            <li v-for="i in 2" class="flex gap-2 py-4 border-b border-b-stone-200/87 items-center text-[0.69rem] w-full justify-between last:border-0">
                <section class="flex gap-2">
                    <div class="p-1.25  bg-blue-100 rounded-lg h-min text-[1rem]">💬</div>
                    <div>
                        <h3 class="text-[0.82rem] font-medium">Slack</h3>
                        <p class=" leading-2.5 text-stone-500/80">Get alerts and reports in your Slack workspace</p>
                    </div>
                </section>
                    <button class="px-8 border border-stone-300 bg-gray-700/6 text-stone-700  text-[0.8rem] py-0.75 rounded-lg ">Connect</button>
            </li>
        </ul>
    </NCard>
</template>