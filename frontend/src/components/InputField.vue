<script lang="ts" setup>

interface Props {
    placeholder: string
}

const props = defineProps<Props>();
</script>

<template>
    <input class="w-full bg-olive-100 p-3 placeholder:font-medium font-arial focus:outline-olive-400 focus:outline-1 rounded-lg border border-olive-200" :placeholder="placeholder" />
</template>