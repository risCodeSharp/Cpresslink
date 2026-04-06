<script lang="ts" setup>
import CurrentPlan from '@/features/settings/billingPlan/components/CurrentPlan.vue';
import Invoices from '@/features/settings/billingPlan/components/Invoices.vue';
import Usage from '@/features/settings/billingPlan/components/Usage.vue';
import SettingHeading from '@/features/settings/components/SettingHeading.vue';
</script>
<template>
    <SettingHeading title="Billing & Plan"
        description="Manage your subscription view usage limits, and download invoices" class="mb-6" />
    <CurrentPlan />
    <Usage />
    <Invoices />
</template>
