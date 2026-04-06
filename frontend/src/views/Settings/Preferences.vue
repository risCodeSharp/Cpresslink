<script setup lang="ts">
import SettingHeading from '@/components/SettingHeading.vue';
import Display from '@/features/settings/preferences/components/Display.vue';
import EmailAlert from '@/features/settings/preferences/components/EmailAlert.vue';
import LinkBehaviour from '@/features/settings/preferences/components/LinkBehaviour.vue';


</script>

<template>
    <SettingHeading class="mb-6" 
        title="Preferences"
        description="Customise your notification settings, interface, and default link behaivour." />
        
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