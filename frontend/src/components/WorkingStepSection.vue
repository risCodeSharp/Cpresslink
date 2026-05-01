<script lang="ts" setup>
import FeatureCard from '@/components/FeatureCard.vue';
import StepItem from './StepItem.vue';

interface Step {
    step: number,
    title: string,
    description: string,
};

const steps: Step[] = [
    {
        step: 1,
        title: "Paste your URL",
        description: "Drop any long link into the input bar a product page, blog page, campaign URL, or anything else."
    },
    {
        step: 2, 
        title: "Get a short link",
        description: "CPressLink generates a clean 'cpress.link/slug <- replace this later' instantly. Customise the slug or keep the auto one."
    },
    {
        step: 3, 
        title: "Share & track",
        description: "Copy and share anywhere. Your dashboard updates in real time with clicks, sources, devices, and geo data."
    },
]


</script>


<template>
    <div>
    <section class="mx-5 xxl:mx-90 xl:w-270 xl:mx-auto lg:mx-35 md:mx-10 my-20">
        <div class=" text-gray-400">
            <p class="tracking-wider text-sm">HOW IT WORKS</p>
            <h2 class="tracking-tight my-3 text-2xl text-gray-700 font-classic">Three steps to a smarter link</h2>
        </div>
        <h3 class="text-base w-140 text-gray-400 my-2">
            No setup required. Paste, shorten, share - and watch the data roll in.
        </h3>
        <!-- main content -->
         <div class="grid grid-cols-3 gap-3 mt-15">
             <StepItem v-for="step in steps" :step="step.step" :title="step.title" :description="step.description"/>
         </div>
    </section>
    </div>
</template>
