<script script lang="ts" setup>
import { NCard, NDataTable, type DataTableColumns } from 'naive-ui';
import { useMessage } from 'naive-ui';
import type { CreateRowProps, TableColumn } from 'naive-ui/es/data-table/src/interface';
import { h, type HTMLAttributes } from 'vue';
import LinkTools from './LinkTools.vue';

type LinkStatus = 'ACTIVE' | 'INACTIVE' | 'PAUSED';

interface RowLinkData {
    // key: number,
    name: string,
    clicks: number,
    ctr: string,
    status: LinkStatus,
    created: string,
}

type LinkRelation = {
    source: string,
    destination: string,
};

type LinkMapping = Map<string, LinkRelation>;

const message = useMessage();

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

console.log(linksMapping.get('Summer Campaign 24'))

const linksData: RowLinkData[] = [
    {
        name: 'Summer Campaign 24',
        clicks: 3241,
        ctr: '4.82%',
        status: 'ACTIVE',
        created: 'Jun 1, 2024'
    },
    {
        name: 'Product Launch v2',
        clicks: 1890,
        ctr: '3.21%',
        status: 'ACTIVE',
        created: "Jun 3, 2024"
    }

]

const columns: DataTableColumns<RowLinkData> = [
    {
        title: 'LINK NAME',
        key: 'name',
        render(row) {
            const relation: LinkRelation | undefined = linksMapping.get(row.name);
            return h('div', { class: 'flex flex-col text-[0.78rem]'}, [
                h('span', {class: 'text-gray-800 text-[0.9rem]'}, row.name),
                h('span', {class: 'text-gray-600 text-[0.72rem]'}, relation?.source ?? ''),
                h('span', {class: 'text-gray-400 text-[0.62rem]'}, relation?.destination ?? ''),
            ]);
        }
    },
    {
        title: 'CLICKS',
        key: 'clicks',
        render(row) {
            return h('div', {class: 'flex flex-col'}, [
                h('span', {class: 'font-classic text-lg'}, row.clicks),
                h('span', {class: 'text-gray-500 text-[0.68rem]'}, 'clicks')
            ])
        }
    },
    {
        title: 'CTR',
        key: 'ctr'
    },
    {
        title: 'STATUS',
        key: 'status',
        render(row) {
            const statusColors: Record<LinkStatus, string> = {
                ACTIVE: 'bg-emerald-100/80 text-green-800',
                INACTIVE: 'bg-red-100/80 text-red-800',
                PAUSED: 'bg-amber-100/80 text-amber-800',
            }
            
            return h('div', {class: ' mx-auto'}, [
                h('span',{class: `py-0.5 px-2 rounded-lg text-[0.72rem] ${statusColors[row.status]}`}, 
                row.status
            )]
            );
        }
    },
    {
        title: 'CREATED',
        key: 'created',
        render(row) {
        return h('span', {class: 'text-[0.78rem] text-gray-500'},row.created);
        }
    },
    {
        title: 'Action',
        key: '',
        render(row) {
        return h(LinkTools, {class: 'flex'}, {});
        }
    }
];


</script>
<template>
    <NCard>
        <template #header>
            <h2 class="text-[0.78rem]">All Links</h2>
        </template>
        <!-- <div class="w-full max-w-2xl mx-auto">
            <table class="w-full border-collapse text-[0.68rem]">
               
            </table>
        </div> -->
        <NDataTable 
            :row-props="rowProps" 
            :columns="columns" 
            :data="linksData" 
            :style="{
                'font-size': 0.68, 
                'padding': '0px',
            }" />

    </NCard>
</template>
