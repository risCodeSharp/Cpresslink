<script setup lang="ts">
import AccountActions from '@/features/settings/profile/components/AccountActions.vue';
import DomainBranding from '@/features/settings/profile/components/DomainBranding.vue';
import PersonalInfo from '@/features/settings/profile/components/PersonalInfo.vue';
import SettingHeading from '@/features/settings/components/SettingHeading.vue';
import AvatarCard from '@/features/settings/profile/components/AvatarCard.vue';

</script>

<template>
    <SettingHeading title="My Profile"
        description="Your public identity and workspace details across all CPressLink environments." class="mb-6" />
    <AvatarCard />

    <PersonalInfo class="mb-6" />
    <DomainBranding class="mb-6" />
    <AccountActions class="mb-6" />


</template>
<style lang="css" scoped>
.rounded-card {
    border-radius: 12px;
}

.dark-gradient {
    background: linear-gradient(135deg, #1a2e23 0%, #2a4a35 100%);
}
</style>