import{defineStore} from 'pinia';
import {type Ref, ref, computed} from 'vue';
import type { DateRange, LinkData } from '@/types';
export const useDashboardStore = defineStore('dashboard', () => {
    const dateRange: Ref<DateRange> = ref('7d');
    const links: Ref<LinkData[]> = ref([
        {
            id: '1',
            name: 'Summer Campaign 24',
            link: 'cpress.link/summer-24',
            clicks: "3.2k",
            shortName: "SC"
        },
        {
            id: '2',
            name: 'Product Launch v2',
            link: 'cpress.link/p-launch',
            clicks: "1.9k",
            shortName: "PL"
        },
        {
            id: '3',
            name: 'Newsletter May',
            link: 'cpress.link/news-05',
            clicks: "0.9k",
            shortName: "NM"
        },
        {
            id: '4',
            name: 'API Docs',
            link: 'cpress.link/api-docs',
            clicks: "0.5k",
            shortName: "AP"
        },
        {
            id: '5',
            name: 'Black Friday Sale',
            link: 'cpress.link/bk-24',
            clicks: "6.1k",
            shortName: "BF"
        },
        {
            id: '6',
            name: 'Webinar Registration',
            link: 'cpress.link/webinar-q2',
            clicks: "2.2k",
            shortName: "WR"
        },
        {
            id: '7',
            name: 'Pricing Page Test',
            link: 'cpress.link/pricing-v3',
            clicks: "0.9k",
            shortName: "PP"
        },
    ]);
    const selectedLinkId: Ref<string | null> = ref( links.value[0]?.id ?? null);

    const selectedLink = computed( () => links.value.find(link => link.id === selectedLinkId.value) || null);

    const setDateRange = (range: DateRange) => {
        dateRange.value = range;
    }

    const setSelectedLink = (id: string) => {
        selectedLinkId.value = id;
    }

    
    return {dateRange,links, selectedLink, setSelectedLink, setDateRange};
})