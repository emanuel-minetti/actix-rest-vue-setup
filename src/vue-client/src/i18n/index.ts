import { createI18n } from 'vue-i18n';

const messages = {
  en: {
    nav: {
      home: 'Home',
      about: 'About',
    },
  },
  de: {
    nav: {
      home: 'Startseite',
      about: 'Über uns',
    },
  },
};

export default createI18n({
  legacy: false,
  locale: 'en',
  fallbackLocale: 'de',
  messages,
});
