<script lang="ts" setup>
import Switch from '@/components/Switch.vue';
import { NCard, NSwitch } from 'naive-ui';
import { reactive } from 'vue';
import type { CSSProperties } from 'vue'

function railStyle({
  focused,
  checked
}: {
  focused: boolean
  checked: boolean
}): CSSProperties {
  const style: CSSProperties = {
    transition: 'all 0.2s ease-in-out', 
    backgroundColor: checked ? '#00674F' : '#e5e7eb',
  }

 if (focused) {
  // This adds a subtle dark ring inside the rail
  style.boxShadow = 'inset 0 0 0 2px rgba(0, 0, 0, 0.1)';
  
  // Optional: add a very slight outer glow that has 0 spread 
  // but high blur to avoid the "border" look
  style.boxShadow += checked 
    ? ', 0 0 8px rgba(0, 103, 79, 0.3)' 
    : ', 0 0 8px rgba(156, 163, 175, 0.3)';
}

  return style
}


const notificationSwitches = reactive([
    { title: 'Dark Mode', description: 'High-contrast interface', enabled: false },
    { title: 'Compact View', description: 'Denser link tables', enabled: false },
    { title: 'Show Favicons', description: 'Load destination icons', enabled: true },
])

</script>

<template>
    <NCard class="rounded-card shadow-around ">
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