<template>
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
          <RouterLink class="nav-link" to="/">{{ $t('nav.home') }}</RouterLink>
        </li>
        <li class="nav-item">
          <RouterLink class="nav-link" to="/about">{{ $t('nav.about') }}</RouterLink>
        </li>
      </ul>
    </div>
  </nav>
</template>

<script setup lang="ts">
import { RouterLink } from 'vue-router';
</script>

<style scoped lang="scss">
@import '../scss/main';

.navbar .nav-link {
  @extend .fs-3;
  @extend .text-primary;
}
</style>
