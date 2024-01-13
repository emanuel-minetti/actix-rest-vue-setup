<template>
  <span :class="`fi fi-${flag_abbrev}`"></span>
  <select class="form-select" @change="switchLanguage">
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
import { computed } from 'vue';

const { t, locale } = useI18n();
const supportedLocales = Translations.supportedLocales;
const switchLanguage = async (event: any) => {
  const newLocale = event.target.value;
  await Translations.switchLanguage(newLocale);
};
const flag_abbrev = computed(() => {
  return locale.value === 'en' ? 'gb' : locale.value;
});
</script>

<style scoped lang="scss"></style>
