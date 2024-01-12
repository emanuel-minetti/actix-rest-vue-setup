import i18n from '@/i18n/index';
import type { WritableComputedRef } from 'vue';

type Language = WritableComputedRef<'en' | 'de'>;

const Translations = {
  set currentLanguage(newLocale: Language) {
    i18n.global.locale = newLocale;
  },
  get supportedLocales() {
    return ['en', 'de'];
  },
  async switchLanguage(newLocale: Language) {
    Translations.currentLanguage = newLocale;
    document.querySelector('html')?.setAttribute('lang', newLocale.value);
  },
};

export default Translations;
