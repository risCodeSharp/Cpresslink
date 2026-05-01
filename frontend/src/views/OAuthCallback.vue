<script lang="ts" setup>
import {onMounted} from 'vue';
import {useRoute, useRouter} from 'vue-router';
import {useAuthStore} from '@/stores/auth';

const route = useRoute();
const router = useRouter();
const auth = useAuthStore();

onMounted(() => {
    const token = route.query.token as string;

    if (token) {
        auth.handleOAuth(token);
        router.push("/dashboard")
    } else {
        router.push("/login");
    }
})

</script>

<template>
    <h1>Logging in... </h1>
</template>