<script lang="ts" setup>
import FeatureSection from "@/components/FeatureSection.vue";
import WorkingStepSection from '@/components/WorkingStepSection.vue';
</script>


<template>
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

    <FeatureSection />
    <WorkingStepSection />
    <section class="mx-10 xxl:mx-90 xl:w-270 xl:mx-auto lg:mx-35 md:mx-10">
      <div class="w-full mx-auto m-10 px-7 py-10 rounded-xl bg-emerald-800 flex justify-between items-center gap-7 ">
        <div class="w-full">
          <h3 class="font-classic text-3xl text-white mb-2">Ready to take control of your links?</h3>
          <p class="text-sm text-green-100"> Join thousands of teams using CPressLink to track, optimise, and scale
            their URL strategy.</p>
        </div>
        <div class="flex justify-end gap-2 text-white font-lg text-sm w-120 mx-auto">
          <button class="rounded-md bg-green-600/70 px-4 py-2 ">Go to Dashboard → </button>
          <button class="rounded-md outline-1 outline-gray-600  bg-lime-600/40 px-4 py-2"> View Analytics</button>
        </div>
      </div>
    </section>
</template>
