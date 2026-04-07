<script lang="ts" setup>
import { computed, type ComputedRef } from 'vue';
import { useAttrs } from 'vue';

defineOptions({
    inheritAttrs: false
})

interface Props {
    label: string,
    value: number,
    max: number,
}
const attrs = useAttrs();

const props = defineProps<Props>();

const percent = computed(() => {
    return props.max ? (props.value / props.max) * 100 : 0
})

</script>

<template>
    <div>
        <div class="flex justify-between text-[0.78rem] text-gray-900">
            <span>{{ label }}</span>
            <span>{{ value }}</span>
        </div>
        <div class="w-full h-2 bg-emerald-900/12 rounded-full flex items-center ">
            <div class="h-1 rounded-full transition-all  bg-linear-to-r from-emerald-700 to-emerald-500 
          duration-700 ease-out w-(--w)"
                :class="attrs.class" :style="{ '--w': percent + '%' }"></div>
        </div>
    </div>

</template>