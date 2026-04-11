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
                <div>
                    <div class="uppercase tracking-wider text-[0.58rem] text-slate-400 mb-0.5 ">Usage</div>
                    <h2 class=" text-[1rem] font-semibold text-slate-900">Resource Usage This Month</h2>
                </div>
        </template>

        <Usagebar 
            v-for="{name, value, max} in usageData"
            :label="name" 
            :value="value" 
            :max="max"
        class="mb-1.5"/>
    </NCard>


</template>


<style lang="css" scoped>
.rounded-card {
    border-radius: 12px;
}
</style>