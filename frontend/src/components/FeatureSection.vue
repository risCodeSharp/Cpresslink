<script lang="ts" setup>
import FeatureCard from '@/components/FeatureCard.vue';

interface Feature {
    icon: string,
    header: string,
    body: string,
};

const features: Feature[] = [
    {
        icon: `<svg width="18" height="18" viewBox="0 0 20 20" fill="currentColor" class="">
                        <path
                            d="M10 2C5.58 2 2 5.58 2 10s3.58 8 8 8 8-3.58 8-8-3.58-8-8-8zm.5 11.41L7.09 10 8.5 8.59l2 2 4-4L15.91 8z">
                        </path>
                </svg>`,
        header: "Instant shortening",
        body: "Generating a clean shot link in milliseconds. Custom slugs, branded domains, and auto-expiry all supported."
    },
    {
        icon: ` <svg width="18" height="18" viewBox="0 0 20 20" fill="currentColor">
                        <path d="M3 17h2v-7H3v7zm4 0h2V7H7v10zm4 0h2v-4h-2v4zm4 0h2V3h-2v14z"></path>
                </svg>`,
        header: "Deep analytics",
        body: " Track clicks over time, referrer sources, device types, and geo reach — all in real time."
    },
    {
        icon: ` <svg width="18" height="18" viewBox="0 0 20 20" fill="currentColor">
                        <path d="M10 1l2.4 6.8H19l-5.7 4.1 2.2 6.6L10 14.4l-5.5 4.1 2.2-6.6L1 7.8h6.6z"></path>
                </svg>`,
        header: "Smart routing",
        body: "Route visitors by device, location, or time of day. One link, infinite destinations."
    },
    {
        icon: `<svg width="18" height="18" viewBox="0 0 20 20" fill="none"><rect x="3" y="3" width="14" height="14" rx="3" stroke="currentColor" stroke-width="1.5"></rect><path d="M7 10h6M10 7v6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"></path></svg>`,
        header: "Link management",
        body: "Organise links into campaigns, set expiry dates, enable/disable with one click, and bulk-edit with ease."
    },
    {
        icon: `<svg width="18" height="18" viewBox="0 0 20 20" fill="currentColor"><path d="M10 2a8 8 0 1 0 0 16A8 8 0 0 0 10 2zm-1 11.41L5.59 10 7 8.59l2 2 4-4L14.41 8z"></path></svg>`,
        header: "99.98% uptime",
        body: "Globally distributed infrastructure ensures your links never go down, no matter the traffic spike."
    },
    {
        icon: `<svg width="18" height="18" viewBox="0 0 20 20" fill="none"><path d="M10 2l1.8 5.4H18l-4.6 3.3 1.8 5.4L10 13l-5.2 3.1 1.8-5.4L2 7.4h6.2z" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round"></path></svg>`,
        header: "Full API access",
        body: "REST API with webhooks. Integrate link shortening directly into your product, CMS, or workflow."
    },

];

</script>


<template>
    <div>
    <section class="mx-5 xxl:mx-90 xl:w-270 xl:mx-auto lg:mx-35 md:mx-10">
        <div class=" mb-2  text-gray-400">
            <p class="tracking-wider text-sm">WHY CPRESSLINK</p>
            <h2 class="tracking-tight my-3 text-2xl text-gray-700 font-classic">Everything a link needs to perform </h2>
        </div>
        <h3 class="text-base w-140 text-gray-400 my-2">
            Built for markerters, developers, and creators who need precision, speed,
            and insight from every URL they share.
        </h3>
        <div
            class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 md gap-5 Route visitors by device, location, or time of day. One link, infinite destinations mx-2 my-8">
            <FeatureCard v-for="feature in features">
                <template #icon>
                    <div v-html="feature.icon"></div>
                </template>
                <template #header>{{ feature.header }}</template>
                <template #body>{{ feature.body }}</template>
            </FeatureCard>
        </div>
    </section>
    </div>
</template>
