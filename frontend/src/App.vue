<script setup lang="ts">

import { RouterView } from 'vue-router';
import Footer from '@/components/Footer.vue';
import Header from '@/components/Header.vue';
import { NMessageProvider, NModalProvider } from 'naive-ui';

</script>

<template>
  <NConfigProvider>
    <NModalProvider>
      <NMessageProvider>


        <div class="flex flex-col justify-between ">
            <router-view />
        </div>

      </NMessageProvider>
    </NModalProvider>
  </NConfigProvider>
</template>
<style scoped>
.font-arial {
    font-family: Arial, Helvetica, sans-serif;
}
</style>
