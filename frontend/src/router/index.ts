import { useAuthStore } from '@/stores/auth';
import Analytics from '@/views/Analytics.vue';
import Dashboard from '@/views/Dashboard.vue';
import Home from '@/views/Home.vue';
import Login from '@/views/Login.vue';
import OAuthCallback from '@/views/OAuthCallback.vue';
import APIIntegrations from '@/views/Settings/API&Integrations.vue';
import BillingPlans from '@/views/Settings/BillingPlans.vue';
import Preferences from '@/views/Settings/Preferences.vue';
import Profile from '@/views/Settings/Profile.vue';
import Security from '@/views/Settings/Security.vue';
import SettingsLayout from '@/views/SettingsLayout.vue';
import Signup from '@/views/Signup.vue';
import { createRouter, createWebHistory, type Router } from 'vue-router';

const router: Router = createRouter({
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
      component: Dashboard,
      meta: { requiresAuth: true },

    },
    {
      name: "Analytics",
      path: '/analytics',
      component: Analytics,
      meta: { requiresAuth: true },
    },
    {
      name: "Login",
      path:  '/login',
      component: Login,
    },
    {
      name: "Signup",
      path:  '/signup',
      component: Signup,
    },
    {
      name: "oauth",
      path: "/oauth",
      component: OAuthCallback,
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
      ],
      meta: { requiresAuth: true },
    },
  ],
});

router.beforeEach((to, _, next) => {
  const auth = useAuthStore();

  if (to.meta.requiresAuth && !auth.isAuthenticated) {
    next("/login");
  } else {
    next();
  }
});

export default router;
