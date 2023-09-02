<template>
  <Title msg="About" />
  <p>This is an about page</p>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Title from '@/components/Title.vue';

export default defineComponent({
  components: { Title },
});
</script>
