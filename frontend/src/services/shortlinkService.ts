import api from "./apiService";
import type { CreateShortLink, ShortLink } from "@/types/shortlink";
import type { ApiResponse } from "@/types/api";



export const shortlinkService = {
    create: async (payload: CreateShortLink) => {
        const response = await api.post<ApiResponse<ShortLink>>(
            "/api/links",
            payload
        );
        console.log(response.data.data);
        return response.data.data;
    },
    create_by_slug: async (payload: CreateShortLink) => {
        const response = await api.post<ApiResponse<ShortLink>>(
            "/api/links",
            payload
        );
        return response.data.data;
    },
};