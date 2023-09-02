import './scss/main.scss';
// noinspection ES6UnusedImports
//import { Button } from 'bootstrap'

import { createApp } from 'vue';
import { createPinia } from 'pinia';

import App from './App.vue';
import router from './router';

const app = createApp(App);

app.use(createPinia());
app.use(router);

app.mount('#app');
