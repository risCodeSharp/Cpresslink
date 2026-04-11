<script lang="ts" setup>

import SettingHeading from '@/features/settings/components/SettingHeading.vue';
import Devices from '@/features/settings/security/components/Devices.vue';
import Password from '@/features/settings/security/components/Password.vue';
import TwofactorAuth from '@/features/settings/security/components/TwofactorAuth.vue';
</script>
<template>
    <SettingHeading title="Security"
        description="Manage your login credentials two-factor authentication, and active sessions" 
        class="mb-6" />
    <Password class="mb-6 shadow-around rounded-card"/>
    <div class="flex gap-4 w-full"> 
        <TwofactorAuth class=" rounded-card shadow-around"/>
        <Devices class=" rounded-card shadow-around"/>
    </div>
</template>
