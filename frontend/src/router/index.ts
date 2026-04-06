import Analytics from '@/views/Analytics.vue';
import Dashboard from '@/views/Dashboard.vue';
import Home from '@/views/Home.vue';
import APIIntegrations from '@/views/Settings/API&Integrations.vue';
import BillingPlans from '@/views/Settings/BillingPlans.vue';
import Preferences from '@/views/Settings/Preferences.vue';
import Profile from '@/views/Settings/Profile.vue';
import Security from '@/views/Settings/Security.vue';
import SettingsLayout from '@/views/SettingsLayout.vue';
import { createRouter, createWebHistory } from 'vue-router';

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
      name: 'SettingsParent', // different from child names
      path: '/settings',
      component: SettingsLayout,
      children: [
        { path: '', name: 'invaild',redirect:  { name: 'Profile' } }, // default child
        { path: 'profile', name: 'Profile', component: Profile },
        { path: 'security', name: 'Security', component: Security },
        { path: 'billing', name: 'Billing', component: BillingPlans },
        { path: 'preferences', name: 'Preferences', component: Preferences },
        { path: 'api_integrations', name: 'API_Integrations', component: APIIntegrations }
      ]
    },
  ],
})

export default router
