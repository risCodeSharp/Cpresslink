<script lang="ts" setup>


interface Props {
    title: string,
    description: string,
}

const props = defineProps<Props>();

</script>

<template>
    <div>
        <h1 class="font-classic text-2xl text-slate-900">{{ title }}</h1>
        <p class="text-normal-size text-gray-500
    ">{{ description }}</p>
    </div>
</template>