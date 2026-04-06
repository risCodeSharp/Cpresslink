<script lang="ts" setup>
import { NCard, NInput } from 'naive-ui';


</script>

<template>
    <NCard class="rounded-card shadow-around my-6">
        <template #header>
            <div class="flex text-normal-size justify-between items-center">
                <div>
                    <div class="uppercase tracking-wider text-slate-400 mb-0.5 ">Workspace</div>
                    <h2 class=" text-[17px] font-semibold text-slate-900">Domain & Branding</h2>
                </div>
                <button class="bg-black text-white py-1.5 px-4 rounded-lg">Save Changes</button>
            </div>
        </template>
        
        <div class="flex gap-4 mb-3">
            <div class="flex flex-col gap-0.5 w-full" >
                <label class="ml-2 text-[0.61rem] text-slate-500 uppercase font-light tracking-widest">Default Domain</label>
                <n-input round style="background: #f4f4f0;" placeholder="cpress.link"/>
            </div>
            <div class="flex flex-col gap-0.5 w-full" >
                <label class="ml-2 text-[0.61rem] text-slate-500 uppercase font-light tracking-widest">Custom Domain</label>
                <n-input disabled round style="background: #f4f4f0;" />
            </div>
        </div>
        <div class="flex flex-col gap-0.5 w-full">
                <label class="ml-2 text-[0.61rem] text-slate-500 uppercase font-light tracking-widest">Brand Name (Shown on link preview)</label>
                <n-input round style="background: #f4f4f0;" placeholder="brand name"/>
        </div>

    </NCard>


</template>


<style lang="css" scoped>
.rounded-card {
    border-radius: 12px;
}
</style>