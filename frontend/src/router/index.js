// src/router/index.js
import { createRouter, createWebHistory } from 'vue-router';

const routes = [
  {
    path: '/',
    name: 'Landing',
    component: () => import('../pages/Landing.vue'),
    meta: { title: 'MicroRoulette - Provably Fair On-Chain Roulette' }
  },
  {
    path: '/play',
    name: 'Game',
    component: () => import('../pages/Game.vue'),
    meta: { title: 'Play - MicroRoulette' }
  },
  {
    path: '/history',
    name: 'History',
    component: () => import('../pages/History.vue'),
    meta: { title: 'History - MicroRoulette' }
  },
  {
    path: '/leaderboard',
    name: 'Leaderboard',
    component: () => import('../pages/Leaderboard.vue'),
    meta: { title: 'Leaderboard - MicroRoulette' }
  },
  {
    path: '/rules',
    name: 'Rules',
    component: () => import('../pages/Rules.vue'),
    meta: { title: 'How to Play - MicroRoulette' }
  },
  {
    path: '/:pathMatch(.*)*',
    redirect: '/'
  }
];

const router = createRouter({
  history: createWebHistory(),
  routes,
  scrollBehavior(to, from, savedPosition) {
    if (savedPosition) {
      return savedPosition;
    }
    return { top: 0 };
  }
});

// Update document title on navigation
router.beforeEach((to, from, next) => {
  document.title = to.meta.title || 'MicroRoulette';
  next();
});

export default router;
