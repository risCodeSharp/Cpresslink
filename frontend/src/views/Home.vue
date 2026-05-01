<script lang="ts" setup>
import FeatureSection from "@/components/FeatureSection.vue";
import Footer from "@/components/Footer.vue";
import WorkingStepSection from '@/components/WorkingStepSection.vue';
import Header from "@/components/Header.vue";
</script>


<template>
  
      <Header class="mb-10"/>
    <main class="mx-10 lg:w-5xl lg:mx-auto">
      <section class="flex flex-col items-center justify-center h-220 text-center border-b border-b-gray-300 ">
      <p class="text-gray-500 font-light py-3 tracking-widest text-[0.8rem]">CPRESSLINK</p>
      <h1 class=" font-classic text-6xl xl:text-7xl py-3  mx-auto"><span>Shorten Links.</span> <span
          class="italic text-green-900"><br /> Amplify Every Click.</span></h1>
      <p class="py-3 w-100 mx-auto text-emerald-900">Transform long URLs into clean, trackable short links. Built for
        creators, and developers who card about precision.</p>
      <div class="py-8 flex gap-4">
        <button class="bg-emerald-950 text-white px-6 py-3 rounded-md">Start Shortening Free</button>
        <button class="border border-gray-300 px-6 py-3 rounded">View Analytics Demo →</button>
      </div>

      <div
        class="my-6 shadow-gray-200 shadow-[0_10px_32px_rgba(0,0,0,0.15)] flex gap-3 p-2 rounded-xl bg-white w-full max-w-2xl">
        <input class="w-full py-2 px-3 focus:outline-none" placeholder="Paste your long URL here..." />
        <button class="sm:w-40 w-full text-center text-white bg-emerald-800 px-6 py-2 rounded-lg">
          Shorten →
        </button>
      </div>

      <!-- Stats -->
      <div class="mt-12 grid grid-cols-2 md:grid-cols-4 gap-6 w-full max-w-5xl text-center">
        <div class="border-r-2 border-r-gray-300">
          <div class="font-classic text-3xl mb-2">2.4B+</div>
          <div class="text-gray-500 text-sm">LINK SHORTENED</div>
        </div>

        <div class="border-r-2 border-r-gray-300">
          <div class="font-classic text-3xl mb-2">99.98%</div>
          <div class="text-gray-500 text-sm">UPTIME</div>
        </div>

        <div class="border-r-2 border-r-gray-300">
          <div class="font-classic text-3xl mb-2">18ms</div>
          <div class="text-gray-500 text-sm">AVG. REDIRECT</div>
        </div>

        <div>
          <div class="font-classic text-3xl mb-2">180+</div>
          <div class="text-gray-500 text-sm">COUNTRIES</div>
        </div>
      </div>
    </section>

    <FeatureSection class="mt-10 mb-3 flex items-center "/>
    <WorkingStepSection class="my flex items-center "/>
    <section class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 border-t border-gray-100">
  <!-- Maintaining the lighter emerald gradient -->
  <div class="my-10 overflow-hidden rounded-2xl bg-linear-to-br from-emerald-800 via-emerald-700 to-emerald-600 p-6 md:px-10 md:py-8 shadow-lg">
    <div class="flex flex-col lg:flex-row items-center justify-between gap-8">
      
      <!-- Text Content: Refined smaller sizing -->
      <div class="max-w-md text-center lg:text-left">
        <h3 class="font-classic text-lg md:text-xl text-white mb-2 leading-snug tracking-wide drop-shadow-sm">
          Ready to take control of your links?
        </h3>
        <p class="text-emerald-50/85 text-xs md:text-[0.85rem] font-light leading-relaxed">
          Join thousands of teams using <span class="font-medium text-white">CPressLink</span> to track, optimise, and scale their URL strategy.
        </p>
      </div>

      <!-- Action Buttons: Slightly more compact to match text -->
      <div class="flex flex-col sm:flex-row items-center gap-3 shrink-0">
        <RouterLink 
          to="/dashboard" 
          class="group flex items-center justify-center rounded-xl bg-emerald-400 px-5 py-2.5 text-[0.75rem] font-bold uppercase tracking-widest text-emerald-950 transition-all hover:bg-white hover:scale-105 shadow-md"
        >
          Go to Dashboard
          <span class="ml-1.5 transition-transform group-hover:translate-x-1">→</span>
        </RouterLink>
        
        <RouterLink 
          to="/analytics" 
          class="rounded-xl border border-white/30 bg-white/10 px-5 py-2.5 text-[0.75rem] font-semibold text-white backdrop-blur-sm transition-all hover:bg-white/20"
        >
          View Analytics
        </RouterLink>
      </div>
      
    </div>
  </div>
</section>
    </main>
  <Footer class=" w-full "/>

</template>
