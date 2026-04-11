<script lang="ts" setup>
import CancelSubscription from '@/features/settings/billingPlan/components/CancelSubscription.vue';
import CurrentPlan from '@/features/settings/billingPlan/components/CurrentPlan.vue';
import Invoices from '@/features/settings/billingPlan/components/Invoices.vue';
import Subscriptions from '@/features/settings/billingPlan/components/Subscriptions.vue';
import Usage from '@/features/settings/billingPlan/components/Usage.vue';
import SettingHeading from '@/features/settings/components/SettingHeading.vue';
</script>
<template>
    <SettingHeading title="Billing & Plan"
        description="Manage your subscription view usage limits, and download invoices" class="mb-6" />
    <CurrentPlan class="mb-4"/>
    <Subscriptions class="mb-4"/>
    <Usage class="mb-4"/>
    <Invoices class="mb-4"/>
    <CancelSubscription class="mb-4"/>
</template>
