<script lang="ts" setup>
import { NCard } from 'naive-ui';
import InputPassword from './InputPassword.vue';
</script>

<template>
    <NCard >
        <template #header>
            <!-- Refactor remove flex later -->
            <div class="flex text-normal-size justify-between items-center">
                <div>
                    <div class="uppercase tracking-widest text-[0.58rem] text-slate-400 mb-0.5 ">CREDENTIALS</div>
                    <h2 class=" text-[1rem] font-semibold text-slate-700">Password</h2>
                </div>
            </div>
            <p class=" text-gray-400/80 font-normal text-[0.72rem]">We recommend a strong unique password. Rotate every 90 days for optimal security.</p>
        </template>

        <div class="mb-4">
            <InputPassword label="CURRENT PASSWORD" placeholder="Enter current password" />
        </div>
        <div class="flex justify-between gap-3 mb-4">
            <InputPassword label="NEW PASSWORD" placeholder="Min. 12 characters" />
            <InputPassword label="CONFIRM NEW PASSWORD" placeholder="Repeat new password" />
        </div>
        <div class="py-2 mb-4">
            <buttton class="px-4 cursor-pointer py-1.5 bg-slate-700 text-white rounded-lg mb-10">Update Password</buttton>
        </div>
    </NCard>
</template>

<style scoped>

</style>