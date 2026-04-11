<script lang="ts" setup>
import { NCard} from 'naive-ui';


</script>

<template>
    <NCard class="rounded-card shadow-around my-6 outline outline-rose-100">
        <template #header>
            <div class="flex text-normal-size justify-between items-center">
                <div>
                    <div class="uppercase tracking-wider text-rose-600 mb-0.5 font-light">Cancel</div>
                    <h2 class=" text-[17px] text-slate-900 font-semibold">Cancel Subscription</h2>
                </div>
            </div>
            <div class="py-2">
                <p class="text-[0.8rem] text-olive-500 font-light">Cancelling will downgrade your account to Free tier at the end of your billing period. Your link link will continue to redirect but analytics will be limited.</p>
                <div class="flex gap-5 mt-3 max-sm:flex-wrap">
                <button class="bg-olive-200/47 hover:bg-rose-100/45 text-sm px-4 py-2 rounded-lg text-rose-600 font-light w-70 outline outline-red-200 hover:outline-rose-600 cursor-pointer transition  ease-in-out duration-200 ">Cancel Pro Subscription</button>
                </div>
            </div>
        </template>
    </NCard>
</template>


<style lang="css" scoped>
.rounded-card {
    border-radius: 12px;
}
</style>