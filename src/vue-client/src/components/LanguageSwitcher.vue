<template>
  <div
    tabindex="0"
    :class="`fi fi-${flagAbbrev}`"
    @click="flagClickedHandler"
    @keyup="listenToEnterHandler"
  ></div>
  <select
    id="ls-select"
    v-if="showSelect"
    class="form-select"
    @mouseup="selectMouseupHandler"
    @change="optionMouseupHandler"
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
import { computed, nextTick, ref } from 'vue';
import { useI18n } from 'vue-i18n';

import Translations from '@/i18n/Translations';
import type { LocaleAbbrev } from '@/i18n/Translations';

const { t, locale } = useI18n();
const supportedLocales = Translations.supportedLocales;
const showSelect = ref(false);

const flagClickedHandler = async () => {
  showSelect.value = !showSelect.value;
  await nextTick();
  const selectElement = document.getElementById('ls-select');
  selectElement!.focus();
};
const listenToEnterHandler = (event: KeyboardEvent) => {
  const target = <HTMLElement>event.target;
  if (event.key == 'Enter') {
    target.click();
  }
};
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
