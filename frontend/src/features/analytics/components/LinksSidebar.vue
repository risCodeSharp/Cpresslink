<script lang="ts" setup>
import { ref } from 'vue';
import Card from '@//components/Card.vue';
import { useDashboardStore } from '@/stores/dashboardStore';

const value = ref("");

const store = useDashboardStore();
async function selectLink(id: string) {
    store.setSelectedLink(id);
}

</script>

<template>
    <Card class="rounded-xl h-min">
        <div class="border-b pb-4 border-b-gray-300/70 py-3 flex justify-between items-center px-4">
            <h4 class="font-bold text-gray-600 tracking-tight text-medium-size">Your Links</h4>
            <p class="text-small-size">{{ store.links.length }} links</p>
        </div>
        <div class="border-b border-b-gray-300/70 py-3 px-4">
            <input class="text-gray-800 bg-slate-200/50 focus:outline-green-700 w-full py-1 px-1.5 outline outline-slate-300 rounded-md" placeholder="Search links..." />
        </div>
        <div v-for="link in store.links" :key="link.id" class="border-b border-b-gray-300 last:pb-4 "> 
            <button @click="selectLink(link.id)" 
                class="cursor-pointer border-l-olive-600 w-full h-15 flex px-5 items-center justify-between text-normal-size gap-3 pb-4 pt-5" 
                :class="{'border-l-3 bg-[#e8f0ea]' : link.id === store.selectedLink?.id}"
            >

                <div class=" rounded-lg w-7 h-7 text-emerald-800 bg-emerald-800/15 tracking-wide flex justify-center items-center">
                    <h5 class="font-semibold">{{ link.shortName }}</h5>
                </div>
                <div class="text-start">
                    <p class="truncate  w-30 text-nowrap text-emerald-950 font-medium text-medium-size">{{ link.name }}</p>
                    <p class="text-small-size">{{ link.link }}</p>
                </div>
                <p class="font-medium text-emerald-900 font-small-size">{{ link.clicks }}</p>
            </button>
        </div>
    </Card>
</template>
