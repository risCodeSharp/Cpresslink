<script lang="ts" setup>
import SettingSidebarTab from '@/features/settings/components/SettingSidebarTab.vue';



const sidebarOptions = [
    { icon: '<svg width="14" height="14" viewBox="0 0 16 16" fill="none"><circle cx="8" cy="5" r="3" stroke="currentColor" stroke-width="1.4"></circle><path d="M2 14c0-3.314 2.686-5 6-5s6 1.686 6 5" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"></path></svg>', text: "Profile", to: "profile" },
    { icon: '<svg width="14" height="14" viewBox="0 0 16 16" fill="none"><path d="M8 2L3 4.5V9c0 2.8 2.1 5.4 5 6 2.9-.6 5-3.2 5-6V4.5L8 2z" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round"></path></svg>', text: "Security", to: "security" },
    { icon: '<svg width="14" height="14" viewBox="0 0 16 16" fill="none"><path d="M2 4h12M2 8h8M2 12h10" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"></path></svg>', text: "Preferences", to: "preferences" },
    { icon: '<svg width="14" height="14" viewBox="0 0 16 16" fill="none"><rect x="2" y="3" width="12" height="10" rx="2" stroke="currentColor" stroke-width="1.3"></rect><path d="M2 7h12" stroke="currentColor" stroke-width="1.3"></path></svg>', text: "Billing", to: "billing" },
    { icon: '<svg width="14" height="14" viewBox="0 0 16 16" fill="none"><path d="M4 6l-3 2 3 2M12 6l3 2-3 2M9 4l-2 8" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round"></path></svg>', text: "API & Integrations", to: "api_integrations" },
]

</script>
<template>
    
    <div class="flex w-full justify-around">
        <div class="w-80 ">
            <h2 class="py-2  text-slate-500 text-[0.67rem] uppercase tracking-widest ">Account Workspace</h2>
            <ul class="text-medium-size text-slate-500">
                <li v-for="option in sidebarOptions">
                    <SettingSidebarTab :icon="option.icon" :text="option.text" :to="option.to" />
                </li>
            </ul>
        </div>
        <main class="py-2  mx-2 w-full">
            <router-view />
        </main>
    </div>
</template>