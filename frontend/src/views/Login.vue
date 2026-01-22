<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const isLogin = ref(true)

const handleAuth = () => {
  localStorage.setItem('user', 'Student');
  router.push('/');
}
</script>

<template>
  <div class="auth-container">
    <div class="auth-card glass-panel">
      <h1>{{ isLogin ? 'Welcome Back' : 'Create Account' }}</h1>
      <p>Monitor your heart rate in real-time.</p>
      
      <form @submit.prevent="handleAuth" class="auth-form">
        <input type="email" placeholder="Email" class="input-field" required />
        <input type="password" placeholder="Password" class="input-field" required />
        
        <button type="submit" class="btn btn-primary full-width">
          {{ isLogin ? 'Login' : 'Register' }}
        </button>
      </form>
      
      <div class="toggle-mode" @click="isLogin = !isLogin">
        {{ isLogin ? 'Need an account? Register' : 'Have an account? Login' }}
      </div>
    </div>
  </div>
</template>

<style scoped>
.auth-container {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 60vh;
}

.auth-card {
  padding: 3rem;
  width: 100%;
  max-width: 400px;
  text-align: center;
}

h1 {
  margin-bottom: 0.5rem;
}

p {
  color: var(--text-secondary);
  margin-bottom: 2rem;
}

.full-width {
  width: 100%;
  margin-top: 1rem;
}

.toggle-mode {
  margin-top: 1.5rem;
  font-size: 0.9rem;
  color: var(--primary-color);
  cursor: pointer;
}
</style>
