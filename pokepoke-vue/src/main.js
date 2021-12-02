import { createApp } from 'vue'
import App from './App.vue'
import router from './router'

const app = createApp(App)
app.use(router)
app.mount('#app')

import axios from 'axios'
app.config.globalProperties.$http = axios;
app.config.globalProperties.$player_id = 1;

export default app