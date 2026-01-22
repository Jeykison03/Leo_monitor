<script setup>
import { RouterView, useRouter, useRoute } from 'vue-router'
import { ref, watch } from 'vue'

const router = useRouter()
const route = useRoute()
const isLoggedIn = ref(false)

watch(route, () => {
  isLoggedIn.value = !!localStorage.getItem('user')
})

const handleLogout = () => {
  localStorage.removeItem('user')
  isLoggedIn.value = false
  router.push('/login')
}
</script>

<template>
  <div class="app-container">
    <header class="navbar glass-panel">
      <div class="logo">VitalSense</div>
      <nav v-if="isLoggedIn" class="nav-links">
        <router-link to="/" class="nav-item">Dashboard</router-link>
        <button @click="handleLogout" class="nav-item btn-logout">Logout</button>
      </nav>
    </header>

    <main>
      <RouterView />
    </main>
  </div>
</template>

<style scoped>
.app-container {
  min-height: 100vh;
  padding: 2rem;
  max-width: 1400px;
  margin: 0 auto;
}

.navbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 2rem;
  margin-bottom: 2rem;
}

.logo {
  font-weight: 700;
  font-size: 1.5rem;
  background: linear-gradient(90deg, #58a6ff, #2f81f7);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.nav-links {
  display: flex;
  gap: 1.5rem;
  align-items: center;
}

.nav-item {
  color: var(--text-secondary);
  text-decoration: none;
  font-weight: 500;
  transition: color 0.2s;
  cursor: pointer;
}

.nav-item:hover, .nav-item.router-link-active {
  color: var(--text-primary);
}

.btn-logout {
  background: none;
  border: 1px solid var(--border-color);
  padding: 0.5rem 1rem;
  border-radius: 6px;
  color: var(--danger-color);
}

.btn-logout:hover {
  background: rgba(248, 81, 73, 0.1);
  border-color: var(--danger-color);
}
</style>
