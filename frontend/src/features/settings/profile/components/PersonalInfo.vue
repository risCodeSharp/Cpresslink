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

    <NCard class="rounded-card shadow-around my-6">
        <template #header>
            <div class="flex text-normal-size justify-between items-center">
                <div>
                    <div class="uppercase tracking-wider text-slate-400 mb-0.5 ">Identity</div>
                    <h2 class=" text-[17px] text-slate-900 font-semibold">Personal Information</h2>
                </div>
                <button class="bg-black text-white  py-1.5 px-4 rounded-lg">Save Changes</button>
            </div>
        </template>

        <div class="grid grid-cols-2 gap-4">
            <div class="flex flex-col gap-0.5" v-for="(_, key) in form" :key="key">
                <label class="ml-2 text-[0.61rem] text-slate-500 uppercase font-light tracking-widest">{{
                    formatLabel(key)
                }}</label>
                <n-input round v-model="form[key][0]" :placeholder='form[key][1]' style="background: #f4f4f0;" />
            </div>
        </div>

        <div class="flex flex-col gap-0.5 py-4">
            <label class="text-[0.61rem] ml-1 text-slate-500 uppercase font-light tracking-widest">Professional
                Bio</label>
            <n-input v-model="professionalBio" type="textarea"
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