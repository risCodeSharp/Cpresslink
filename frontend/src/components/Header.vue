<script lang="ts" setup>
import { onMounted, ref, type Ref } from 'vue';
import { RouterLink, useRoute } from 'vue-router';

type PageName = "Home" | "Dashboard" | "Analytics" | "Settings";

interface Page {
    name: PageName,
    path: string
}

let activePage: Ref<PageName> = ref("Home");

const pages: Page[] = [
    {
        name: "Home",
        path: '/'
    },
    {
        name: "Dashboard",
        path: '/dashboard'
    },
    {
        name: "Analytics",
        path: '/analytics'
    },
    {
        name: "Settings",
        path: '/settings'
    },
]
function updateActivePage(page: PageName) {
    activePage.value = page;
}

onMounted(() => {
    const route = useRoute().path;
    pages.forEach((page: Page) => {
        if (page.path === route) {
            activePage.value = page.name;
        }
    });
})

</script>
<template>
    <header class="sticky top-0 z-50 bg-white text-gray-400 font-arial shadow-sm">
        <nav class="flex w-full justify-between h-15 px-10 items-center">

            <!-- <ul class="flex gap-7 text-sm items-center font-arial">
                <ul class="font-classic text-xl text-green-900">CPress<em class="italic text-lime-700">Link</em></ul>
                <li class=" text-black border-b-2 border-gray-900  flex items-center justify-center h-15 my-2">Home</li>
                <li>Dashboard</li>
                <li>Analytics</li>
                <li>Settings</li>
            </ul> -->
            <div class="flex gap-7 text-sm items-center font-arial">
                <ul class="font-classic text-xl text-green-900">CPress<em class="italic text-lime-700">Link</em></ul>
                <li v-for="page in pages" class="flex items-center justify-center h-15 my-2"
                    :class="{ 'text-black border-b-2 border-gray-900' : page.name === activePage}"">
                    <RouterLink :to="page.path" @click="updateActivePage(page.name)">{{ page.name }}</RouterLink></li>
            </div>
            <div class="flex gap-2 items-center text-sm">
                <div class="flex rounded-lg border border-slate-200 justify-between items-center px-2 py-1.5 bg-gray-100">
                    <svg class="mx-2 text-gray-600" width="12" height="12" viewBox="0 0 16 16" fill="none">
                        <circle cx="7" cy="7" r="5" stroke="currentColor" stroke-width="1.5"></circle>
                        <path d="m11 11 3 3" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"></path>
                    </svg>
                    <input class="focus:outline-none text-gray-700" placeholder="Search links..." />
                </div>
                <button class="bg-gray-800 text-white px-5 py-1.5 rounded-lg ">+ New</button>
                <!-- notification later -->
                 <button class="p-2 border border-gray-200 rounded-md">
                    <svg width="14" height="14" viewBox="0 0 16 16" fill="none"><path d="M8 1a5 5 0 0 1 5 5v3l1.5 2.5H1.5L3 9V6a5 5 0 0 1 5-5zm0 14a2 2 0 0 1-2-2h4a2 2 0 0 1-2 2z" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round" fill="none"></path></svg>
                    
                 </button>
                 <button>
                    <span class="p-2 rounded-full bg-lime-100">AT</span>
                 </button>
            </div>
        </nav>
    </header>
</template>
<style>
input {
    font-size: 0.8rem;
    font-family: Arial, Helvetica, sans-serif;
}
</style>