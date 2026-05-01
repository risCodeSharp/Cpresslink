import {defineStore} from "pinia";
import authService from "@/services/authService";
import type { LoginPayload, RegisterPayload, AuthResponse } from "@/types/auth";

interface AuthState {
    user: AuthResponse["user"] | null,
    token: string | null;
    loading: boolean;
}

export const useAuthStore = defineStore("auth", {
    state: (): AuthState => ({
        user: JSON.parse(localStorage.getItem('user') || 'null'),
        token: localStorage.getItem('accessToken'),
        loading: false,
    }),
    getters: {
        isAuthenticated: (state) => !!state.token,
    },

    actions: {
        async login(payload: LoginPayload): Promise<boolean> {
            let success = false;
            this.loading = true;
            try {
                const data = await authService.login(payload);
                this.setAuth(data);
                success = true;
            } catch (err: any){
                console.log(err);
            } finally {
                this.loading = false;
            }
            return success;

        },

        async register(payload: RegisterPayload): Promise<boolean> {
            this.loading = true;
            let success = false;
            try {
                const data = await authService.register(payload);
                this.setAuth(data);
                success = true;
            } catch (err: any){
                console.log(err);
            } finally {
                this.loading = false;
            }
            return success;
        },

        setAuth(data: AuthResponse) {
            this.user = data.user;
            this.token = data.accessToken;

            localStorage.setItem("accessToken", data.accessToken);
            localStorage.setItem("user", JSON.stringify(data.user));
        },

        logout() {
            authService.logout();
            this.user = null;
            this.token = null;
        },

        handleOAuth(token: string) {
            this.token = token;
            localStorage.setItem("accessToken", token);
        }
    }
})