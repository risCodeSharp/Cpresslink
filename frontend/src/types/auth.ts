export interface BackendAuthResponse {
    status: number;
    code: number;
    data: {
        user: {
            id: number;
            username: string;
            email: string;
            created_at: string;
        };
        access_token: string;
    }
}

export interface RegisterPayload {
    username: string;
    email: string;
    password: string;
}

export interface LoginPayload {
    email: string;
    password: string;
}


export interface AuthResponse {
    accessToken: string;
    user: {
        id: number;
        username: string;
        email: string;
        createdAt: Date,
    }
}