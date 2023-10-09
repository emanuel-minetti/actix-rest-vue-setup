import { describe, it, expect } from 'vitest';
import { mount } from '@vue/test-utils';

import Title from '../Title.vue';

describe('Title', () => {
  it('renders properly', () => {
    const wrapper = mount(Title, { props: { msg: 'Hello Vitest' } });
    expect(wrapper.text()).toContain('Hello Vitest');
  });
});
