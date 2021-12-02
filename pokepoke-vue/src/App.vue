<template>
  <!-- <Battle msg="Welcome to Your Vue.js App"/> -->
  <div class="app-back-ground">
    <router-view />
  </div>
</template>

<script>
// import Battle from './components/pages/Battle.vue'
// import app from '@/main.js'
// import axios from 'axios'

export default {
  name: 'App',
  components: {
  },
  data() {
    return {
    }
  },
}
</script>

<style lang="scss">
.app-back-ground {
  background-color: green(base);
  display: flex;
  align-items: center;
  justify-content: center;
}

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  // margin-top: 60px;
}
</style>
