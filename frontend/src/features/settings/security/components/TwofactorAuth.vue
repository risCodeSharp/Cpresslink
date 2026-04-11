<script lang="ts" setup>
import { NCard } from 'naive-ui';
import { NQrCode } from 'naive-ui';
import { type Ref, ref } from 'vue';

const qrLink = "https://riscodesharp.github.io/Cpresslink/";

</script>

<template>
    <NCard>
        <template #header>
            <div class="flex w-full justify-between items-center text-normal-size  border-b border-b-gray-200 pb-3">
                <div>
                    <div class="uppercase tracking-widest text-[0.58rem] text-slate-400 mb-0.5 ">Authenticator App</div>
                    <h2 class=" text-[0.95rem] font-semibold text-slate-700">Two-Factor Authentication</h2>
                </div>
                <button
                    class="text-green-950/80 font-semibold px-3 py-1.5 border border-gray-300 rounded-lg bg-emerald-800/8">Enable</button>
            </div>
        </template>
        <div class="flex justify-between items-center">
            <p class=" text-gray-500 font-normal text-[0.65rem] w-42 mb-4 ">
                Add an extra layer if security to your account.
                Use an app like Google Autheticator or Authy to gernerate codes.
            </p>

            <div class=" p-2.5 py-3 h-min  flex bg-white border-4 border-emerald-900 rounded-lg ">
                <NQrCode :value="qrLink" color="oklch(37.8% 0.077 168.94)" :size="90"
                    style="background-color: transparent; translate: -13%  -13%;" />
            </div>
        </div>
    </NCard>
</template>

<style scoped></style>