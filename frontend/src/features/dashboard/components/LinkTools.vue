<script lang="ts" setup>
import type { RowLinkData } from '@/types';
import { useMessage } from 'naive-ui';
import { NModal } from 'naive-ui';
interface Props {
    sourceLink: string,
    destinationLink: string,
}
const props = defineProps<Props>();
const emit = defineEmits<{
    (event: 'showEditModel'): void
}>()
const message = useMessage();
async function copyText(text: string) {
    try {
        await navigator.clipboard.writeText(text);
        message.success('Copied text: ' + text);
    } catch (err: unknown) {
        message.error("Failed to copy text!");
    }
}
function showModel() { emit('showEditModel');}


</script>

<template>
    <div class="flex items-center gap-1 max-w-40">

        <!-- Copy -->
        <button @click="copyText(destinationLink)" class="group p-[3.5px] rounded-lg border border-emerald-300
           bg-emerald-50 text-emerald-600
           hover:bg-emerald-600 hover:text-white
           active:scale-95
           transition-all duration-200 ease-out
           shadow-sm" title="Copy Link">
            <svg class="w-4 h-4 transition-transform duration-200 group-hover:scale-110 group-hover:-rotate-3"
                fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round"
                    d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
            </svg>
        </button>

        <!-- Open Link -->
        <button class="group p-[3.5px] rounded-lg border border-emerald-300
           bg-emerald-50 text-emerald-600
           hover:bg-emerald-600 hover:text-white
           active:scale-95
           transition-all duration-200 ease-out
           shadow-sm" title="Open Link">
            <svg class="w-4 h-4 transition-transform duration-200 group-hover:translate-x-0.5 group-hover:-translate-y-0.5"
                fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round"
                    d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
            </svg>
        </button>

        <!-- Edit -->
        <button @click="showModel" class="group p-[3.5px] rounded-lg border border-emerald-300
           bg-emerald-50 text-emerald-600
           hover:bg-emerald-600 hover:text-white
           active:scale-95
           transition-all duration-200 ease-out
           shadow-sm" title="Edit"
           >
            <svg class="w-4 h-4 transition-transform duration-200 group-hover:rotate-12" fill="none"
                stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round"
                    d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
            </svg>
        </button>

        <!-- Divider -->
        <div class="w-[1.4px] rounded h-6 bg-gray-300 mx-1"></div>

        <!-- Delete -->
        <button class="group p-[3.5px] rounded-lg border border-rose-300
           bg-rose-50 text-rose-600
           hover:bg-rose-600/78 hover:text-white
           active:scale-95
           transition-all duration-200 ease-out
           shadow-sm" title="Delete">
            <svg class="w-4 h-4 transition-transform duration-200 group-hover:rotate-6" fill="none"
                stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round"
                    d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
            </svg>
        </button>

    </div>
</template>