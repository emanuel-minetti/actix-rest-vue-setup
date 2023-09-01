<template>
  <div class="text-primary h1 text-center data-arvs-top-title">{{ msg }}</div>
</template>

<script setup lang="ts">
defineProps<{
  msg: string
}>()
</script>
