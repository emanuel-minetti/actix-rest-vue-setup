import i18n from '@/i18n/index';

export type LocaleAbbrev = 'de' | 'en';

const Translations = {
  set currentLanguage(newLocale: LocaleAbbrev) {
    i18n.global.locale.value = newLocale;
  },
  get supportedLocales() {
    return ['en', 'de'];
  },
  async switchLanguage(newLocale: LocaleAbbrev) {
    Translations.currentLanguage = newLocale;
    document.querySelector('html')?.setAttribute('lang', newLocale);
  },
};

export default Translations;
