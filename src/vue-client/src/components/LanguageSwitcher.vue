<template>
  <span :class="`fi fi-${flagAbbrev}`" @click="showSelect = !showSelect"></span>
  <select v-if="showSelect" class="form-select" @change="switchLanguage" @blur="showSelect = false">
    <option
      v-for="sLocale in supportedLocales"
      :key="`locale-${sLocale}`"
      :value="sLocale"
      :selected="locale === sLocale"
    >
      {{ t(`locale.${sLocale}`) }}
    </option>
  </select>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import Translations from '@/i18n/Translations';
import { computed, ref } from 'vue';

const { t, locale } = useI18n();
const supportedLocales = Translations.supportedLocales;

const showSelect = ref(false);
const switchLanguage = async (event: any) => {
  const newLocale = event.target.value;
  await Translations.switchLanguage(newLocale);
  showSelect.value = false;
};
const flagAbbrev = computed(() => {
  return locale.value === 'en' ? 'gb' : locale.value;
});
</script>

<style scoped lang="scss"></style>
