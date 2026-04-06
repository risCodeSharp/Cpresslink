<script setup lang="ts">
import AccountActions from '@/features/settings/profile/components/AccountActions.vue';
import DomainBranding from '@/features/settings/profile/components/DomainBranding.vue';
import PersonalInfo from '@/features/settings/profile/components/PersonalInfo.vue';


</script>

<template>
    <h1 class="font-classic text-2xl text-slate-900">My Profile</h1>
    <p class="text-normal-size text-gray-500
    ">Your public identity and workspace details across all CPressLink environments.</p>
    <PersonalInfo class="mb-6" />
    <DomainBranding class="mb-6" />
    <AccountActions class="mb-6" />


</template>
<style lang="css" scoped>
.rounded-card {
    border-radius: 12px;
}
</style>