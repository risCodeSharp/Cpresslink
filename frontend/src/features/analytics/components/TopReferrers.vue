<script lang="ts" setup>
import LineBar from '@/components/LineBar.vue';
import { NCard } from 'naive-ui';
import {type Ref, ref, computed, type ComputedRef} from 'vue';
interface ReferData {
    label: string,
    value: number,
}

const referrers: Ref<ReferData[]> = ref([
    { label: "Twitter / X", value: 1103},
    { label: "Linkedin", value: 748},
    { label: "Direct / Email", value: 477},
    { label: "Instagram", value: 243},

]);

const maxValue: ComputedRef<number> = computed(() => 
    Math.max(...referrers.value.map(r => r.value))
) 


</script>

<template>
<NCard class="pb-2 rounded-card shadow-around">
    <h2 class="text-[0.9rem] font-semibold ">Top Referers</h2>
    <ul>
        <li  v-for="refer in referrers" class="mt-2 transition-all duration-300 ease-in delay-100"><LineBar  :value="refer.value" :max="maxValue" :label="refer.label"/></li>
    </ul>

</NCard>    

</template>

