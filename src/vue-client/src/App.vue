<template>
  <header>
    <nav class="navbar navbar-expand">
      <div class="container-fluid">
        <RouterLink class="nav-link" to="/">
          <img
            alt="Vue logo"
            src="@/assets/logo.svg"
            width="100"
            height="100"
            class="d-inline-block"
          />
          <span class="h1 text-primary"> Vite App </span>
        </RouterLink>
        <ul class="navbar-nav">
          <li class="nav-item">
            <RouterLink class="nav-link" to="/">Home</RouterLink>
          </li>
          <li class="nav-item">
            <RouterLink class="nav-link" to="/about">About</RouterLink>
          </li>
        </ul>
      </div>
    </nav>
  </header>
  <RouterView />
</template>

<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
</script>

<style lang="scss" scoped>
@import 'scss/main';

.nav-link {
  @extend .fs-3;
  @extend .text-primary;
}

.router-link-active {
  @extend .fw-bold;
}
</style>
