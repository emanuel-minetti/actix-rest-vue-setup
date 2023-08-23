<template>
  <div class="text-primary h1 text-center">{{ msg }}</div>
</template>

<script setup lang="ts">
defineProps<{
  msg: string
}>()
</script>
