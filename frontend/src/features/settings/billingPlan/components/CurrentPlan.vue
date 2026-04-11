<script lang="ts" setup>
import { NCard } from 'naive-ui';
</script>

<template>
    <NCard class="rounded-card shadow-around dark-green-card"
        >
        <!-- Refactor remove flex later -->
        <div class="uppercase tracking-wider text-[0.58rem]  text-white/50 ">Current Plan</div>
        <div class="text-white/80 flex text-[0.1rem] justify-between items-center">
            <div>
                <h2 class="text-white/90 font-classic text-[1.45rem]">Pro Plan ⚡</h2>
                <div class="font-light text-small-size text-white/50 ">Up to 500 links - Custom domains -
                    Full analytics - API access</div>
            </div>
            <div class=" space-y-1 text-end text-olive-300/70 text-small-size text-small-size font-light">
                <p class="font-classic text-3xl text-white/95">$29</p>
                <p>per month - billed annually</p>
                <p>Next bill: Jul 1, 2026</p>
            </div>
        </div>
    </NCard>
</template>