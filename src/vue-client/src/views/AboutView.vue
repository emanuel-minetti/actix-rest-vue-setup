<template>
  <Title msg="About" />
  <h1>This is an about page</h1>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import Title from '@/components/Title.vue'

export default defineComponent({
  components: { Title }
})
</script>
