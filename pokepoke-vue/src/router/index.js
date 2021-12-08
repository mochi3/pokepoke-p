import { createRouter, createWebHistory } from 'vue-router'
import Login from '@/components/pages/Login'
import Battle from '@/components/pages/Battle'
import Select from '@/components/pages/Select'
import ShowEach from '@/components/pages/ShowEach'
import DebugLogin from '@/components/pages/DebugLogin'
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
  {
    path: '/show-each',
    name: 'ShowEach',
    component: ShowEach
  },
  {
    path: '/debug-login',
    name: 'DebugLogin',
    component: DebugLogin
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
