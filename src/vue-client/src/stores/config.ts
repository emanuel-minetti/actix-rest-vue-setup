import { defineStore } from 'pinia';
import axios from 'axios';

const API_PATH = '/api/config';
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
      const url = location.protocol + '//' + location.host + API_PATH;
      const config = await axios.get(url);
      this.config = config.data;
    },
  },
});
