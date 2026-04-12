<script script lang="ts" setup>
import { NButton, NCard, NDataTable, NForm, NFormItem, NInput, NModal, NSelect, type DataTableColumns } from 'naive-ui';
import { useMessage } from 'naive-ui';
import type { CreateRowProps, TableColumn } from 'naive-ui/es/data-table/src/interface';
import { h, reactive, type Reactive, ref, type HTMLAttributes } from 'vue';
import LinkTools from './LinkTools.vue';
import type { LinkMapping, LinkRelation, LinkStatus, RowLinkData } from '@/types';


const message = useMessage();

interface EditLinkForm {
    name: string,
    sourceUrl: string,
    destinationUrl: string,
    status: LinkStatus
}

const editLinkForm = reactive<EditLinkForm>({
    name: '',
    sourceUrl: '',
    destinationUrl: '',
    status: 'ACTIVE'
});

function rowProps(row: RowLinkData): HTMLAttributes {
    return {
        class: 'cursor-pointer',
        onClick: () => {
            message.info(row.name);
        }
    }
}

const linksMapping: LinkMapping = new Map([
    ['Summer Campaign 24', {
        source: 'cpress.link/summer-24',
        destination: 'marketing.enterprise.com/globalpromo'
    }],
    ['Product Launch v2', {
        source: 'cpress.link/p-launch',
        destination: 'app.v2-system.io/onboarding'
    }],

]);

const isEditModelShown = ref(false);
const showEditModel = () => {
    console.log(isEditModelShown.value);
    isEditModelShown.value = true;
    console.log(isEditModelShown.value);
}


console.log(linksMapping.get('Summer Campaign 24'))

const linksData: RowLinkData[] = [
    {
        name: 'Summer Campaign 24',
        clicks: 3241,
        ctr: 4.82,
        status: 'ACTIVE',
        created: 'Jun 1, 2024'
    },
    {
        name: 'Product Launch v2',
        clicks: 1890,
        ctr: 3.21,
        status: 'ACTIVE',
        created: "Jun 3, 2024"
    }

]

const columns: DataTableColumns<RowLinkData> = [
    {
        key: 'name',
        title() {
            return h('span', { class: "text-[0.78rem] text-gray-400" }, "LINK NAME")
        },
        render(row) {
            const relation: LinkRelation | undefined = linksMapping.get(row.name);
            return h('div', { class: 'flex flex-col text-[0.78rem]' }, [
                h('span', { class: 'text-gray-800 text-[0.9rem]' }, row.name),
                h('span', { class: 'text-gray-600 text-[0.72rem]' }, relation?.source ?? ''),
                h('span', { class: 'text-gray-400 text-[0.62rem]' }, relation?.destination ?? ''),
            ]);
        }
    },
    {
        key: 'clicks',
        title() {
            return h('span', { class: "text-[0.78rem] text-gray-400 text-end text-nowrap" }, "CLICKS")
        },
        render(row) {
            return h('div', { class: 'flex flex-col  text-nowrap' }, [
                h('span', { class: 'font-classic text-lg ' }, row.clicks),
                h('span', { class: 'text-gray-500 text-[0.68rem]' }, 'clicks')
            ])
        }
    },
    {
        key: 'ctr',
        title() {
            return h('span', { class: "text-[0.78rem] text-gray-400 text-nowrap" }, "CTR")
        },
        render(row) {
            return h('span', { class: 'text-nowrap' }, row.ctr + "%")
        }

    },
    {
        key: 'status',
        title() {
            return h('span', { class: "text-[0.78rem] text-gray-400 text-nowrap" }, "STATUS")
        },
        render(row) {
            const statusColors: Record<LinkStatus, string> = {
                ACTIVE: 'bg-emerald-100/80 text-green-800',
                INACTIVE: 'bg-red-100/80 text-red-800',
                PAUSED: 'bg-amber-100/80 text-amber-800',
            }

            return h('div', { class: ' mx-auto' }, [
                h('span', { class: `py-0.5 px-3 rounded-xl text-[0.72rem] text-nowrap ${statusColors[row.status]}` },
                    row.status
                )]
            );
        }
    },
    {
        key: 'created',
        title() {
            return h('span', { class: "text-[0.78rem] text-gray-400 text-nowrap" }, "CREATED")
        },
        render(row) {
            return h('span', { class: 'text-[0.78rem] text-gray-500' }, row.created);
        }
    },
    {
        key: '',
        title() {
            return h('span', { class: "text-[0.78rem] text-gray-400 text-nowrap" }, "ACTION")
        },
        render(row) {
            const link = linksMapping.get(row.name);
            return h(LinkTools, {
                class: 'flex',
                sourceLink: link!.source,
                destinationLink: link!.destination,
                onShowEditModel: showEditModel
            }, {});
        }
    }
];


</script>
<template>
    <NCard>
        <template #header>
            <div class="flex justify-between w-full">
                <h2 class="text-[1.3rem] font-classic">All Links</h2>
                <div class="flex justify-end text-[0.78rem] items-center gap-4">
                    <button class="px-4 cursor-pointer py-1 border border-stone-300 text-gray-600/75 rounded-lg">All</button>
                    <button class="px-4 cursor-pointer py-1 border border-stone-300 text-gray-600/75 rounded-lg">Active</button>
                    <button class="px-4 cursor-pointer py-1 border border-stone-300 text-gray-600/75 rounded-lg border-r-2">InActive</button>
                    <div class="h-6 border-r border-stone-300"></div>
                    <button class="px-4 cursor-pointer py-1 border border-stone-300 text-gray-600/75 rounded-lg">Sort: Click ↓</button>
                    <button class="px-4 cursor-pointer py-1 border border-stone-300 text-gray-600/75 rounded-lg">Filter</button>
                </div>
            </div>
        </template>

        <NDataTable :row-props="rowProps" :columns="columns" :data="linksData" :style="{
            'font-size': 0.68,
            'padding': '0px',
        }" />
        < </NCard>
            <NModal v-model:show="isEditModelShown" preset="card" title="Edit Link" contentScrollable :bordered="false"
                class="rounded-2xl" :style="{
                    width: 'clamp(340px, 92vw, 520px)',
                    height: 'clamp(420px, 85vh, 560px)'
                }" :segmented="{ content: true, footer: true }">
                <!-- Form Content -->
                <NForm :model="editLinkForm" labelPlacement="top" size="medium" class="px-1">
                    <NSpace vertical size="large">
                        <!-- Link Name Field -->
                        <NFormItem label="Link Name" path="name">
                            <NInput v-model:value="editLinkForm.name" placeholder="Enter link name" clearable />
                        </NFormItem>

                        <!-- Source URL Field -->
                        <NFormItem label="Source URL" path="source">
                            <NInput v-model:value="editLinkForm.sourceUrl" placeholder="https://cpress.link/..."
                                clearable />
                        </NFormItem>

                        <!-- Destination URL Field -->
                        <NFormItem label="Custom URL" path="destination">
                            <NInput v-model:value="editLinkForm.destinationUrl"
                                placeholder="https://marketing.enterprise.com/..." clearable />
                        </NFormItem>

                        <!-- Status Selector -->
                        <NFormItem label="Status" path="status">
                            <NSelect v-model:value="editLinkForm.status" :options="[
                                { label: 'Active', value: 'ACTIVE' },
                                { label: 'Paused', value: 'PAUSED' },
                                { label: 'Inactive', value: 'INACTIVE' }
                            ]" />
                        </NFormItem>
                    </NSpace>
                </NForm>

                <!-- Footer Buttons -->
                <template #footer>
                    <NSpace justify="end">
                        <NButton @click="isEditModelShown = false">Cancel</NButton>
                        <NButton type="primary" @click="isEditModelShown = false">Save</NButton>
                    </NSpace>
                </template>

            </NModal>
</template>