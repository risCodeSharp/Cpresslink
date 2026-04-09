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

        <Header />

        <div class="flex flex-col justify-between min-h-screen">
          <main class="m-6 lg:w-5xl lg:mx-auto">
            <router-view />
          </main>
          <Footer />
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
