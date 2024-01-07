import { defineStore } from 'pinia';
import axios from 'axios';

export const useConfigStore = defineStore('config', {
  state: () => ({
    config: {
      copyright: String,
      version: String,
      global_message: String,
    },
  }),
  getters: {},
  actions: {
    fetchConfig: async function () {
      try {
        const url = location.protocol + '//' + location.host + '/api/config';
        const config = await axios.get(url);
        this.config = config.data;
      } catch (e) {
        // TODO add error handling
        alert(e);
        console.log(e);
      }
    },
  },
});
