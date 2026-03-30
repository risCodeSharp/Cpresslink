<script lang="ts" setup>
import { } from 'vue';
interface Props {
    step: number,
    title: string,
    description: string,
};

const props: Props = defineProps<Props>();

</script>

<template>
    <div class=" text-center mx-4">
        <div class="mb-5 mx-auto rounded-full flex justify-center items-center bg-white w-13 h-13 shadow-md font-classic ">{{ step }}
        </div>
        <h4 class="mb-1 font-semibold text-mist-700 title">{{ title }}</h4>
        <p class="text-sm text-mist-500 desc">{{ description }}</p>
    </div>
</template>
<style lang="css" scoped>
.title {
    font-size: 14.5px;
}
</style>
