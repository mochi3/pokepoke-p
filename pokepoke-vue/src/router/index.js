import { createRouter, createWebHistory } from 'vue-router'
import Login from '@/components/pages/Login'
import Battle from '@/components/pages/Battle'
import Select from '@/components/pages/Select'
// import { createApp } from 'vue'

// Vue.use(VueRouter)



const routes = [
  {
    path: '/',
    name: 'Login',
    component: Login
  },
  {
    path: '/battle',
    name: 'Battle',
    component: Battle
  },
  {
    path: '/select',
    name: 'Select',
    component: Select
  },
//   {//vue-router4（vue3）では使えない表記
//     path: '*',
//     redirect: '/'
//     },
]

const router = createRouter({
    history: createWebHistory(process.env.BASE_URL),
    routes
})

export default router
