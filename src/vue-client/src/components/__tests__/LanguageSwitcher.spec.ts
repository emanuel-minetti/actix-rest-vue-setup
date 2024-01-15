import { describe, it, expect } from 'vitest';
import { mount } from '@vue/test-utils';

import LanguageSwitcher from '../LanguageSwitcher.vue';
import i18n from '../../i18n';
describe('LanguageSwitcher', () => {
  it('renders properly', () => {
    const wrapper = mount(LanguageSwitcher, {
      global: {
        plugins: [i18n],
      },
    });
    expect(wrapper.findAll('select').length.valueOf() === 0);
  });
});
