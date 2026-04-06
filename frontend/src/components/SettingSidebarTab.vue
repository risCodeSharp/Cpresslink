<script lang='ts' setup>
import { RouterLink } from 'vue-router';

interface Props {
     to: string,
     icon: string,
     text: string,
}
const props = defineProps<Props>();

</script>

<template>
     <RouterLink :to="to" class="flex items-center py-1 px-2 hover:bg-emerald-50 rounded-lg">
          <span v-html="icon"</span>
          <span class="ml-2">{{ text }}</span>
     </RouterLink>
</template>
