<script lang="ts" setup>
import { NBadge, NCard, NSwitch } from 'naive-ui';
import { X, Check, Minus } from 'lucide-vue-next';

</script>

<template>
    <NCard class="shadow-around rounded-card">
        <template #header>
            <!-- Refactor remove flex later -->
            <div class="flex text-normal-size justify-between items-center mb-1.5">
                <div class="w-full">
                    <div class="uppercase tracking-widest text-[0.68rem] text-slate-400 mb-0.5 ">SUBSCRIPTIONS</div>
                    <div class="flex justify-between w-full">
                        <h2 class=" text-[1rem] font-semibold text-slate-900">Scale Your Experience</h2>
                        <div class="flex items-center space-x-2 ">
                            <p>Monthly</p>
                            <NSwitch />
                            <div
                                class="bg-green-100 px-2 py-0.5 rounded-full outline outline-green-300 font-semibold">Save
                                25%</div>
                        </div>
                    </div>
                </div>

            </div>
        </template>

        <div class="flex text-[0.72rem] w-full gap-3 ">
            <button v-for="i in 3" class="cursor-pointer bg-stone-50 text-left border p-3 w-full rounded-lg border-stone-200">
                <h3 class="  text-[0.9rem] font-semibold">Starter</h3>
                <p><span class=" leading-tight text-[1.5rem] font-bold">$0</span><span
                        class="text-stone-500/83 font-medium">/month</span></p>
                <p class=" leading-3  text-[0.75rem] font-medium mb-2.5 ">Free forever</p>
                <ul>
                    <li class="text-stone-700">
                        <Check class="w-4 h-4 inline-block text-emerald-700 mr-0.5" /> 50 short links
                    </li>
                    <li class="text-stone-700">
                        <Check class="w-4 h-4 inline-block text-emerald-700 mr-0.5" /> Basic analytics
                    </li>
                    <li class="text-stone-700">
                        <Check class="w-4 h-4 inline-block text-emerald-700 mr-0.5" /> 1,000 API calls/month
                    </li>
                    <li class="text-stone-400"><span>
                            <Minus class="w-4 h-4 inline-block text-stone-400 mr-0.5" />
                        </span> No custom domains</li>
                    <li class="text-stone-400"><span>
                            <Minus class="w-4 h-4 inline-block text-stone-400 mr-0.5" />
                        </span> No geo analytics</li>
                </ul>
            </button>

        </div>
    </NCard>
</template>