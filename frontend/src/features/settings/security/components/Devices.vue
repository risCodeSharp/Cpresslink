<script lang="ts" setup>
import { NCard } from 'naive-ui';
import { computed, type ComputedRef, onMounted, reactive, type Reactive, type Ref, ref } from 'vue';
import DeviceInfoService from '@/services/DeviceInfoService';
import LocationService from '@/services/LocationService';
import type { DeviceInformation, Location } from '@/types';

interface DeviceSession {
    name: string,
    lastSession: string,
    location: string,
    isCurrentDevice: boolean,
    type: string,
    browser: string,

}

const currentDevice: DeviceInformation = DeviceInfoService.getBrowserAndDeviceInfo();


function getDeviceSymbol(type: string) {
    console.log(type)
    if (type.toLowerCase() === 'mobile') return '📱';
    if (type.toLowerCase() === 'laptop') return '💻';
    return '🖥️'
}

const devices: Reactive<DeviceSession[]> = reactive([
    {
        name: currentDevice.deviceName,
        lastSession: "Current session",
        location: 'Loading...',
        isCurrentDevice: true,
        type: currentDevice.device,
        browser: currentDevice.browser,
    },
    {
        name: "iPhone 15",
        lastSession: "2 days ago",
        location: "San francisco",
        isCurrentDevice: false,
        type: 'mobile',
        browser: "Safari",
    },
    {
        name: "Windows PC",
        lastSession: "1 week age",
        location: "New York",
        isCurrentDevice: false,
        type: "desktop",
        browser: "firefox",
    },
]);

onMounted(async () => {
    const stateName = await LocationService.getLocation();
    if (devices[0]) {
        devices[0].location = stateName.stateName ?? 'Loading...';
    }
});

</script>

<template>
    <NCard>
        <template #header>
            <div class="uppercase tracking-widest text-[0.58rem] text-slate-400 ">ACTIVE SESSIONS</div>
            <h2 class=" text-[0.95rem] font-semibold text-slate-700">Devices</h2>
        </template>
        <ul class="mb-2">
            <li v-for="device in devices" class="py-2  border-b border-b-slate-200 last:border-b-0">
                <div class="flex justify-between">
                    <div class="flex gap-1">
                    <div class="p-0.5 bg-green-100 outline outline-gray-300 rounded-md mr-2">
                        {{ getDeviceSymbol(device.type) }}
                    </div>
                    <div class="text-[0.68rem] leading-3.5">
                        <h3 class="font-semibold ">{{ device.name }} - {{ device.browser }}</h3>
                        <span>{{ device.lastSession }} - {{ device.location }}</span>
                    </div>
                    </div>
                    <button v-if="!device.isCurrentDevice" class="cursor-pointer text-gray-400/85 h-6 border border-gray-300 rounded-lg font-medium px-1.5 py-0.5 text-[0.68rem] text-nowrap">Revoke</button>
                </div>
            </li>
        </ul>
        <button class="text-[0.78rem] w-full bg-red-100/40 py-1 outline outline-red-200 text-red-600/80 rounded-lg">Sign Out All Devices</button>
        
    </NCard>
</template>

<style scoped></style>