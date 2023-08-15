<template>
  <header>
    <nav class="navbar">
      <div class="container-fluid">
        <a class="navbar-brand" href="#">
          <img
            alt="Vue logo"
            src="@/assets/logo.svg"
            width="125"
            height="125"
            class="d-inline-block align-text-top"
          />
          Vite App
        </a>
      </div>
    </nav>
  </header>
  <div class="wrapper">
    <nav>
      <RouterLink to="/">Home</RouterLink>
      <RouterLink to="/about">About</RouterLink>
    </nav>
  </div>
  <button class="btn btn-primary">Test</button>
  <HelloWorld msg="You did it!" />
  <RouterView />
</template>

<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import HelloWorld from './components/HelloWorld.vue'
</script>

<style lang="scss" scoped>
@import 'scss/main';

.navbar-brand {
  --bs-navbar-brand-color: #{$primary};
}
</style>
