import i18n from '@/i18n/index';

const Translations = {
  set currentLanguage(newLocale: 'en' | 'de') {
    i18n.global.locale.value = newLocale;
  },
  get supportedLocales() {
    return ['en', 'de'];
  },
  async switchLanguage(newLocale: 'en' | 'de') {
    Translations.currentLanguage = newLocale;
    document.querySelector('html')?.setAttribute('lang', newLocale);
  },
};

export default Translations;
