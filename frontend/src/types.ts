export type DateRange = '7d' | '30d' | '90d' | 'all';

export interface LinkData {
    id: string,
    name: string,
    link: string,
    shortName: string,
    clicks: string,
};

export type LinkStatus = 'ACTIVE' | 'INACTIVE' | 'PAUSED';

export interface RowLinkData {
    // key: number,
    name: string,
    clicks: number,
    ctr: number,
    status: LinkStatus,
    created: string,
}

export type LinkRelation = {
    source: string,
    destination: string,
};

export type LinkMapping = Map<string, LinkRelation>;


export interface DeviceInformation {
    browser: string;
    device: string;
    deviceName: string;
    os: string;
    userAgent: string;
}

export interface Location {
    longitude: number,
    latitude: number,
    stateName: string,

}
