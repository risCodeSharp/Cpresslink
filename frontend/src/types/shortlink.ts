export type CreateShortLink = {
    name: string;
    url: string;
    slug?: string;
};


export type ShortLink = {
    id: string;
    name: string;
    slug: string;
    short_code: string;
};
