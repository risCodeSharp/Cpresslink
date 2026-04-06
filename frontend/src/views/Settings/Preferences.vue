<script setup lang="ts">
import Display from '@/features/settings/preferences/components/Display.vue';
import EmailAlert from '@/features/settings/preferences/components/EmailAlert.vue';
import LinkBehaviour from '@/features/settings/preferences/components/LinkBehaviour.vue';


</script>

<template>
    <h1 class="font-classic text-2xl text-slate-900">Preferences</h1>
    <p class="text-normal-size text-gray-500">
        Customise your notification settings, interface, and default link behaivour.
    </p>
    <EmailAlert class="mb-6 " />
    <div class="flex gap-4 items-center">
        <Display />
        <LinkBehaviour />
    </div>
    <!-- <DomainBranding class="mb-6" /> -->
    <!-- <AccountActions class="mb-6" /> -->


</template>
<style lang="css" scoped>
.rounded-card {
    border-radius: 12px;
}
</style>