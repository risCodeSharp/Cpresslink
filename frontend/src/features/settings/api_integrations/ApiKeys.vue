<script lang="ts" setup>
import { X } from 'lucide-vue-next';
import { NCard } from 'naive-ui';


</script>

<template>
     <NCard class="shadow-around rounded-card">
        <template #header>
            <div class="flex w-full justify-between items-center text-normal-size ">
                <div>
                    <div class="uppercase tracking-widest text-[0.58rem] text-slate-400 mb-0.5 ">Access</div>
                    <h2 class=" text-[0.95rem] font-semibold text-slate-700">API Keys</h2>
                </div>
                <button class=" transition-colors duration-200 ease-linear cursor-pointer bg-stone-300/70 hover:bg-stone-800 hover:text-white  text-[0.68rem]  py-1 px-3 rounded-full">+ New Key</button>
                
            </div>
        </template>
        <ul class="text-[0.67rem] text-gray-500/90">
            <li v-for="i in 3" class="flex justify-between w-full items-center border-b border-b-stone-200/88 py-2 last:border-b-0">
                <div>
                    <h3 class="text-[0.78rem] font-semibold text-gray-800">Production Key</h3>
                    <span class="font-mono text-[0.68rem] py-1 px-2 rounded-lg bg-zinc-200/70">cpk_live_••••••••••••••••3f9a</span>
                    <div class="leading-6.5 text-[0.65rem]"><span>Created Jan 12, 2024 ·</span> ·  <span>Last used 2 hours ago</span></div>
                </div>
                <div class="space-x-2.5">
                    <span class="px-3 py-1 bg-emerald-500/12 text-emerald-800 rounded-full font-semibold uppercase">Active</span>
                    <span class="p-1.5 border border-stone-200 inset-10 rounded-lg text-sm">⧉</span>
                    <span class="p-1.5 border border-stone-200 inset-10 rounded-lg text-sm"><X  class="inline w-4 h-4"/></span>
                </div>
            </li>
        </ul>

    </NCard>
</template>