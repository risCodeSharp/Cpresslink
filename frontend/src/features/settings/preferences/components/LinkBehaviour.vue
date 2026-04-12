<script lang="ts" setup>
import Switch from '@/components/Switch.vue';
import { NCard } from 'naive-ui';
import { reactive } from 'vue';



const notificationSwitches = reactive([
    { title: 'Auto-expire After 1 year', description: 'New links expire in 365 days by default', enabled: true },
    { title: 'Track UTM Params', description: 'Automaticaly append campaign tracking', enabled: false },
    { title: '301 Redirect', description: 'Use permanent redirects (SEO-friendly)', enabled: true },
])

</script>

<template>
    <NCard class="rounded-card shadow-around my-6">
        <template #header>
            <!-- Refactor remove flex later -->
            <div>
                <div class="uppercase tracking-wider text-[0.58rem] text-stone-400 mb-0.5 ">Defaults</div>
                <h2 class=" text-[1rem] font-semibold text-stone-900">Link Behaviour</h2>
            </div>
        </template>

        <ul class="">
            <li class="flex justify-between items-center border-b border-b-slate-200 pb-2 mb-2 last:border-b-0 last:pb-0"
                v-for="item in notificationSwitches">
                <div>
                    <h3 class=" font-medium text-[0.78rem] text-gray-700">{{ item.title }}</h3>
                    <p class="text-[0.68rem] text-gray-500  ">{{ item.description }}</p>
                </div>
                <Switch v-model="item.enabled" />
            </li>
        </ul>
    </NCard>


</template>


<style lang="css" scoped>
.rounded-card {
    border-radius: 12px;
}
</style>