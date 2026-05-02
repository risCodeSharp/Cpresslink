<script lang="ts" setup>
import Card from '@/components/Card.vue';
import InputField from '@/components/InputField.vue';
import { copyText } from '@/utils';
import { NCard, useMessage } from 'naive-ui';
import { Copy } from 'lucide-vue-next';
import { ref, watchEffect, type Ref } from 'vue';
import { shortlinkService } from '@/services/shortlinkService';
import type { ShortLink } from '@/types/shortlink';
import { useAuthStore } from '@/stores/auth';

const showLink = ref(false);
const toggleLink = () => (showLink.value = !showLink.value);
const user = useAuthStore().$state;
const name: Ref<string> = ref("");
const destination: Ref<string> = ref("");
const slug: Ref<string | undefined> = ref(undefined);

const get_custom_link = () => 
    `http://localhost:8030/u/${user.user?.username}/${link.value?.slug ?? "unknow" }`;

const get_short_code_link = () => 
    `http://localhost:8030/r/${link.value?.short_code ?? "unknow}" }`;

const message = useMessage();

const copyShortCodeLink = () =>  {
    const link = get_short_code_link();
    copyText(link);
    message.success(`Copied text: ${link}`);
};
const copyCustomLink = () => {
    const link = get_custom_link();
    copyText(link);
    message.success(`Copied text: ${link}`);
};

const link: Ref<ShortLink | undefined> = ref(undefined);

const submitLink = async () => {
        try {
            const new_link = await shortlinkService.create({ name: name.value, url: destination.value, slug: slug.value });
            link.value = {
                ...new_link,
                short_code: new_link.short_code
            };
            showLink.value = true;
            console.log(link.value);
        } catch (err: any) {
            console.log(err);
        }
};
</script>

<template>
    <NCard size="small" class="m-1">

        <div v-if="!showLink" class="flex flex-col gap-4">

            <div>
                <h3 class="text-sm font-semibold text-gray-700">
                    Create Links
                </h3>
            </div>

            <div class="flex flex-col md:flex-row gap-4 md:items-end">
                <section class="w-full md:flex-1">
                    <span class="text-tiny mr-1.5">name</span>
                    <InputField v-model.trim="name" placeholder="Enter name..." class="py-2 w-full" />

                </section>

                <section class="w-full md:flex-1">
                    <span class="text-tiny mr-1.5">destination</span>
                    <InputField v-model.trim="destination" placeholder="Paste your long URL here..."
                        class="py-2 w-full" />
                </section>

                <section class="w-full md:flex-[0.7]">
                    <span class="text-tiny mr-1.5">cpress.link/</span>
                    <InputField v-model.trim="slug" placeholder="custom-slug (optional)" class="py-2 w-full" />
                </section>

                <button @click="submitLink"
                    class="w-full md:w-28 h-10 cursor-pointer bg-linear-to-bl from-emerald-900 to-emerald-500 text-white rounded-lg hover:opacity-90 transition">
                    Shorten →
                </button>
            </div>
        </div>

        <div v-else class="p-2">

            <div class="bg-white/70 rounded-xl shadow-sm space-y-5 p-4">

                <div class="flex items-center gap-3">
                    <button @click="toggleLink"
                        class="flex items-center justify-center w-9 h-9 rounded-full bg-emerald-600 text-white hover:bg-emerald-700 transition">
                        ←
                    </button>

                    <h3 class="text-sm font-semibold text-gray-700">
                        Generated Links
                    </h3>
                </div>

                <div>
                    <h4 class="text-xs font-medium text-emerald-700 mb-2">
                        Custom Link
                    </h4>

                    <div
                        class="flex items-center justify-between gap-3 px-3 py-2 rounded-lg bg-white border border-emerald-100 shadow-sm hover:shadow transition overflow-hidden">
                        <span class="font-mono text-sm text-gray-700 truncate">
                           {{ get_custom_link() }}
                        </span>

                        <button @click="copyCustomLink"
                            class="flex cursor-pointer items-center gap-1 text-emerald-600 hover:text-emerald-800 transition whitespace-nowrap">
                            <Copy class="w-4 h-4" />
                            <span class="text-sm">Copy</span>
                        </button>
                    </div>
                </div>

                <div>
                    <h4 class="text-xs font-medium text-emerald-700 mb-2">
                        Shortcode Link
                    </h4>

                    <div
                        class="flex items-center justify-between gap-3 px-3 py-2 rounded-lg bg-white border border-emerald-100 shadow-sm hover:shadow transition overflow-hidden">
                        <span class="font-mono text-sm text-gray-700 truncate">
                           {{ get_short_code_link() }}
                        </span>

                        <button @click="copyShortCodeLink"
                            class="flex cursor-pointer items-center gap-1 text-emerald-600 hover:text-emerald-800 transition whitespace-nowrap">
                            <Copy class="w-4 h-4" />
                            <span class="text-sm">Copy</span>
                        </button>
                    </div>
                </div>

            </div>
        </div>
    </NCard>
</template>