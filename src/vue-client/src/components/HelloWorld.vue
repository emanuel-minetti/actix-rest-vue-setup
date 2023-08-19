<script setup lang="ts">
defineProps<{
  msg: string
}>()
</script>

<template>
  <div class="text-primary h1">{{ msg }}</div>
</template>
