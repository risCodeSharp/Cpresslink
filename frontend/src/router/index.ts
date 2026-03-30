import Analytics from '@/views/Analytics.vue'
import Dashboard from '@/views/Dashboard.vue'
import Home from '@/views/Home.vue'
import Settings from '@/views/Settings.vue'
import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      name: "home",
      path: "/",
      component: Home,
    },
     {
        name: "Dashboard",
        path: '/dashboard',
        component: Dashboard

    },
    {
        name: "Analytics",
        path: '/analytics',
        component: Analytics
    },
    {
        name: "Settings",
        path: '/settings',
        component: Settings
    },
  ],
})

export default router
