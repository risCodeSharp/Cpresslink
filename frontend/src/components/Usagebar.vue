<script lang="ts" setup>
import { computed, type ComputedRef } from 'vue';
interface Props {
    label: string,
    value: number,
    max: number,
}

const props = defineProps<Props>();

const percent = computed(() => (props.value / props.max) * 100);
const color: ComputedRef<string> = computed(() => {
    const value = percent.value;
    if (value >= 90) return "bg-red-600";
    if (value >= 75) return "bg-amber-500";
    if (value >= 50) return "bg-yellow-400";
    if (value >= 25) return "bg-emerald-500";
    return "bg-green-600";
});

</script>

<template>
    <div>
        <div class="flex justify-between text-medium-size text-gray-600">
            <span>{{ label }}</span>
            <div class="space-x-1">
                <span>{{ value }} / {{ max }}</span>
                <span class="text-gray-400">used</span>
            </div>
        </div>
        <div class="w-full h-2 bg-emerald-700/10 rounded-full flex items-center ">
            <div class="h-1   rounded-full transition-all ease-in duration-300" :class="color"
                :style="{ width: percent + '%' }"></div>
        </div>
    </div>

</template>