<script lang="ts" setup>
import Switch from '@/components/Switch.vue';
import { NCard} from 'naive-ui';
import { reactive } from 'vue';

const notificationSwitches = reactive([
    { title: 'Email Notifications', description: 'Critical system alerts and project updates', enabled: true },
    { title: 'Daily Analytics Reports', description: 'Summary of clicks, CTR, and top performers delivered each morning', enabled: false },
    { title: 'Weekly Digest', description: 'A full breakdown of your link performance each Monday', enabled: true },
    { title: 'Link Expiry Warnings', description: 'Get notified 48 hours before a link express', enabled: true },
    { title: 'Click Milestone Alerts', description: 'Celebrate when your link hit 1k, 10k, 100k clicks', enabled: false },
])

</script>

<template>
    <NCard class="rounded-card shadow-around my-6">
        <template #header>
            <!-- Refactor remove flex later -->
            <div class="flex text-normal-size justify-between items-center">
                <div>
                    <div class="uppercase tracking-widest text-normal-size text-slate-400 mb-0.5 ">Notifications</div>
                    <h2 class=" text-[18px] font-semibold text-slate-900">Email & Alerts</h2>
                </div>
            </div>
        </template>

        <ul class="">
            <li class="flex justify-between items-center border-b border-b-slate-200 pb-2 mb-2 last:border-b-0 last:pb-0"
                v-for="item in notificationSwitches">
                <div>
                    <h3 class=" font-medium text-[0.825rem] text-gray-700">{{ item.title }}</h3>
                    <p class="text-normal-size text-gray-500  ">{{ item.description }}</p>
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