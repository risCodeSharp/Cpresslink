<script lang="ts" setup>
import Usagebar from './Usagebar.vue';
import { NCard} from 'naive-ui';
import { reactive, type Reactive } from 'vue';

interface UsageData {
    name: string,
    value: number,
    max: number,
}

const usageData: Reactive<UsageData[]> = reactive([
    {name: 'Short Links', value: 42, max: 500},
    {name: 'Total Clicks', value: 12412, max: 100000},
    {name: 'Custom Domains', value: 1, max: 5},
    {name: 'API Calls', value: 8210, max: 10000},
]);

</script>

<template>
    <NCard class="rounded-card shadow-around">
        <template #header>
            <!-- Refactor remove flex later -->
            <div class="flex text-normal-size justify-between items-center">
                <div>
                    <div class="uppercase tracking-widest text-normal-size text-slate-400 mb-0.5 ">Usage</div>
                    <h2 class=" text-[18px] font-semibold text-slate-900">Resource Usage This Month</h2>
                </div>
            </div>
        </template>

        <Usagebar 
            v-for="{name, value, max} in usageData"
            :label="name" 
            :value="value" 
            :max="max"
        class="mb-2"/>
    </NCard>


</template>


<style lang="css" scoped>
.rounded-card {
    border-radius: 12px;
}
</style>