<script lang="ts" setup>
import { NCard, NInput } from 'naive-ui';
import { reactive, ref, type Reactive } from 'vue';

type FormLabel =
    | 'firstName'
    | 'lastName'
    | 'emailAddress'
    | 'username'
    | 'location'
    | 'website';

type FormValue = [string, string];

const form: Reactive<Record<FormLabel, FormValue>> = reactive({
    firstName: ['', 'e.g. Rahul'],
    lastName: ['', 'e.g. Sharma'],
    emailAddress: ['', 'e.g. rahul.sharma@gmail.com'],
    username: ['', 'e.g. rahul_sharma99'],
    location: ['', 'e.g. Mumbai, Maharshtra, India'],
    website: ['', 'e.g. https://amazon.in/product/xyz123'],
});

const professionalBio = ref("");

//  convert camelCase to "Title Case" with spaces
const formatLabel = (key: string) => {
    return key.replace(/([A-Z])/g, ' $1').replace(/^./, str => str.toUpperCase());
};


</script>

<template>

    <NCard class="rounded-card shadow-around ">
        <template #header>
            <div>
                <div class="uppercase tracking-wider text-[0.58rem] text-slate-400 mb-0.5 ">Identity</div>
                <div class="flex justify-between items-center">
                    <h2 class=" text-[1rem] text-slate-900 font-semibold">Personal Information</h2>
                    <button class=" transition-colors duration-200 ease-linear cursor-pointer bg-stone-300/70 hover:bg-stone-800 hover:text-white  text-[0.68rem]  py-1 px-3 rounded-full">Save Changes</button>
                </div>
            </div>
        </template>

        <div class="grid grid-cols-2 gap-4 ">
            <div class="flex flex-col " v-for="(_, key) in form" :key="key">
                <label class="leading-4 ml-2 text-[0.58rem] text-slate-500 uppercase font-light tracking-wider">{{
                    formatLabel(key)
                }}</label>
                <n-input round v-model="form[key][0]" size="small" :placeholder='form[key][1]'  style="background: #f4f4f0; font-size: 0.75rem;" />
            </div>
        </div>

        <div class="flex flex-col gap-0.5 py-4">
            <label class="text-[0.61rem] ml-1 text-slate-500 uppercase font-light tracking-widest">Professional
                Bio</label>
            <n-input v-model="professionalBio" type="textarea" size="small"
                placeholder="Write a short bio aroud your experience, skills, or background..." round
                style="background: #f4f4f0;" />
        </div>

    </NCard>
</template>


<style lang="css" scoped>
.rounded-card {
    border-radius: 12px;
}
</style>