@import "@/assets/sass/main.scss";

<template>
  <div class="login-page">
    <button @click="loginBattle(1)">player1-battle</button>
    <button @click="loginBattle(2)">player2-battle</button>
    <button @click="print(1)">print1</button>
    <button @click="select(1)">select1</button>
  </div>
</template>

<script>
import app from '@/main.js'

export default {
  name: 'DebugLogin',
  data() {
    return {
    }
  },
  methods: {
    loginBattle(id) {
      app.config.globalProperties.$player_id = id;
      this.$router.push("/battle");
    },
    print(id) {
      this.$http.get(`/print/${id}`)
        .then(res => {
          console.log(res)
        })
    },
    select(id) {
      app.config.globalProperties.$player_id = id;
      this.$router.push("/select");
    },
  },
}
</script>

<style scoped lang="scss">
  .login-page {
    padding: 100px;

    .login-box {
      width: 300px;
      height: 200px;
      background-color: white;
      border-radius: 1rem;
      padding: 50px 20px;
      display: flex;
      flex-direction: column;
      justify-content: space-between;
      
      .login-area {
        display: flex;
        flex-direction: column;
        align-items: center;

        button {
          font-size: 1rem;
          background-color: green(light);
          width: 14rem;
          padding: 0.5rem 1rem;
          border-radius: 2rem;
          box-shadow: 1px 4px 3px -2px #979d97;
          &:hover {
            background-color: green(light-hover);
          }
          &:first-child {
            margin-bottom: 20px;
          }

        }
      }
    }
  }
</style>