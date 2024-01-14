<template>
  <span :class="`fi fi-${flagAbbrev}`" @click="showSelect = !showSelect"></span>
  <select
    v-if="showSelect"
    class="form-select"
    @mouseup="selectMouseupHandler"
    @blur="showSelect = false"
  >
    <option
      v-for="sLocale in supportedLocales"
      :key="`locale-${sLocale}`"
      :value="sLocale"
      :selected="locale === sLocale"
      @mouseup="optionMouseupHandler"
    >
      {{ t(`locale.${sLocale}`) }}
    </option>
  </select>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';

import Translations from '@/i18n/Translations';
import type { LocaleAbbrev } from '@/i18n/Translations';

const { t, locale } = useI18n();
const supportedLocales = Translations.supportedLocales;
const showSelect = ref(false);
const selectMouseupHandler = async (event: Event) => {
  const target: HTMLSelectElement = <HTMLSelectElement>event.target!;
  let flag = target.dataset.flag;
  if (flag) {
    await Translations.switchLanguage(<LocaleAbbrev>target.value);
    showSelect.value = false;
  }
  target.dataset.flag = String(!flag);
};
const optionMouseupHandler = async (event: Event) => {
  const target: HTMLOptionElement = <HTMLOptionElement>event.target!;
  await Translations.switchLanguage(<LocaleAbbrev>target.value);
  showSelect.value = false;
};
const flagAbbrev = computed(() => {
  return locale.value === 'en' ? 'gb' : locale.value;
});
</script>

<style scoped lang="scss"></style>
