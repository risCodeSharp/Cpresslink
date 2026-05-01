import { useRouter } from 'vue-router';
import api from './apiService';
import type { RegisterPayload, LoginPayload, AuthResponse, BackendAuthResponse} from '@/types/auth';


class AuthService {
    async register(payload: RegisterPayload): Promise<AuthResponse> {
        const res = await api.post<BackendAuthResponse>('api/auth/register', payload);

        return {
            user: {
                ...res.data.data.user,
                createdAt: new Date(res.data.data.user.created_at)
            },
            accessToken: res.data.data.access_token,
        }
    }
    
    async login(payload: LoginPayload): Promise<AuthResponse> {
        const res = await api.post<BackendAuthResponse>('api/auth/login', payload);
        const user = res.data.data.user;
        return {
            user: {
                ...res.data.data.user,
                createdAt: new Date(res.data.data.user.created_at)
            },
            accessToken: res.data.data.access_token,
        }
    }

    logout() {
        localStorage.removeItem("accessToken");
        localStorage.removeItem("user");
    }

      // OAuth → redirect (NOT axios)
//   loginWithGoogle() {
//     window.location.href = `${import.meta.env.VITE_API_BASE_URL}/auth/google`;
//   }

//   loginWithGithub() {
//     window.location.href = `${import.meta.env.VITE_API_BASE_URL}/auth/github`;
//   }

}


export default new AuthService();


