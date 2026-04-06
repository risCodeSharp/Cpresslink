<script setup lang="ts">
import AccountActions from '@/features/settings/profile/components/AccountActions.vue';
import DomainBranding from '@/features/settings/profile/components/DomainBranding.vue';
import PersonalInfo from '@/features/settings/profile/components/PersonalInfo.vue';
import SettingHeading from '@/components/SettingHeading.vue';

</script>

<template>
    <SettingHeading  title="My Profile"
        description="Your public identity and workspace details across all CPressLink environments." 
        class="mb-6"/>

    <PersonalInfo class="mb-6" />
    <DomainBranding class="mb-6" />
    <AccountActions class="mb-6" />


</template>
<style lang="css" scoped>
.rounded-card {
    border-radius: 12px;
}
</style>