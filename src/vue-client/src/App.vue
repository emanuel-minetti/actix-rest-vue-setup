<template>
  <header class="outside-main">
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
  <main>
    <div
      id="app-error-message"
      v-if="errorMessage.length > 0"
      class="alert alert-danger text-center"
      role="alert"
    >
      {{ errorMessage }}
    </div>
    <div
      id="app-global-message"
      v-if="config.global_message.length > 0 && errorMessage.length == 0"
      class="alert alert-warning text-center"
      role="alert"
    >
      {{ config.global_message }}
    </div>
    <div class="container">
      <RouterView />
    </div>
  </main>
  <footer class="outside-main mt-3">
    <div class="container-fluid">
      <div class="row align-items-center">
        <div class="col">
          <ul class="list-unstyled">
            <li>
              <RouterLink class="nav-link" to="/imprint">Imprint</RouterLink>
            </li>
            <li>
              <RouterLink class="nav-link" to="/data-privacy">Data Privacy Declaration</RouterLink>
            </li>
            <li>
              <a class="nav-link" href="https://example.com" target="_blank">Company home page</a>
            </li>
          </ul>
        </div>
        <div class="col-4">
          <ul class="list-unstyled">
            <li>
              {{ config.copyright }}
            </li>
            <li>Version: {{ config.version }}</li>
          </ul>
        </div>
      </div>
    </div>
  </footer>
</template>

<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router';
import { useConfigStore } from '@/stores/config';
import { computed, onMounted, ref } from 'vue';

const errorMessage = ref('');
const configStore = useConfigStore();
const config = computed(() => {
  return configStore.config;
});

onMounted(() => {
  configStore
    .fetchConfig()
    .catch((e) => (errorMessage.value = 'Failed to read configuration: ' + e));
});
</script>

<style lang="scss" scoped>
@import 'scss/main';

.navbar .nav-link {
  @extend .fs-3;
  @extend .text-primary;
}

.outside-main {
  background-color: $gray-200;
}

.router-link-active {
  @extend .fw-bold;
}

footer {
  font-size: $font-size-sm;
}

footer .nav-link {
  text-decoration: underline;
}
</style>
