<script lang="ts" setup>
import LineBar from '@/components/LineBar.vue';
import { NCard } from 'naive-ui';
import {type Ref, ref, computed, type ComputedRef} from 'vue';
interface CountryData {
  label: string
  value: number
};

const countries: Ref<CountryData[]> = ref([
  { label: "United States", value: 1321 },
  { label: "United Kingdom", value: 324 },
  { label: "Germany", value: 259 },
  { label: "Canada", value: 210 },
  { label: "India", value: 198 },
  { label: "Australia", value: 176 },
  { label: "France", value: 150 },
  { label: "Netherlands", value: 132 },
]);

const maxValue: ComputedRef<number> = computed(() =>
  Math.max(...countries.value.map(c => c.value))
);


</script>

<template>
<NCard class="pb-2 rounded-card shadow-around">
    <h2 class="text-[0.9rem] font-semibold ">Geographical Reach</h2>
    <ul>
        <li 
            v-for="country in countries"
             class="mt-2 transition-all duration-300 ease-in delay-100"
            ><LineBar :value="country.value" :max="maxValue" :label="country.label"/></li>
    </ul>

</NCard>    

</template>

