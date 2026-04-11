<script lang="ts" setup>
import { type Ref, ref } from "vue";
import { Eye, EyeOff } from 'lucide-vue-next';

interface Props {
    placeholder: string,
    label: string,
}

const props = defineProps<Props>();
const model = defineModel();

const showPassword: Ref<boolean> = ref(false);

function togglePassword() {
    showPassword.value = !showPassword.value;
}
</script>

<template>
    <label class="text-gray-700/90 w-full">
        <div class="ml-0.5 text-[0.62rem] mb-0.5 tracking-wide text-gray-400 font-light">{{ label }}</div>
        <div class="relative group flex justify-around items-center rounded-lg bg-olive-200/47 outline outline-gray-400/40 ">
        <input :type="showPassword ? 'password' : 'text' "
            class=" placeholder:text-[0.7rem] w-full placeholder:text-slate-500/80 px-3 py-1 pr-8 bg-transparent focus:outline-zinc-400/50"
            :placeholder="placeholder" />
        <button @click="togglePassword" class="absolute right-2 top-1/2 -translate-y-1/2 text-gray-500 opacity-0 group-focus-within:opacity-100 transition">
            <Eye v-if="!showPassword" class="w-5 h-5"/>
            <EyeOff v-else class="w-5 h-5"/>
        </button>
        </div>
    </label>
</template>
