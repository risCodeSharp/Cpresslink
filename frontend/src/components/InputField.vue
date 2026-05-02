<script lang="ts" setup>

interface Props {
    placeholder: string
}

const model = defineModel<string | null>();
const props = defineProps<Props>();
</script>

<template>
    <input v-model="model" class="w-full text-[0.68rem] bg-stone-100 border-emerald-600 px-3 py-1 placeholder:text-gray-500/90  placeholder:font-medium font-arial focus:outline-stone-400 focus:outline-1 rounded-lg border " :placeholder="placeholder" />
</template>