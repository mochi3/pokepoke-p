@import "@/assets/sass/main.scss";

<template>
  <div class="login-page">
    <div class="login-box">
      <div class="input-area">
        <div class="input-box id">
          <div>ID</div>
          <input type="text" v-model.trim="id"/>
        </div>
        <div class="input-box password">
          <div>パスワード </div>
          <input type="text" v-model.trim="password"/>
        </div>
      </div>
      <div class="login-area">
        <button class="login-button" @click="doLogin()">ログイン</button>
        <button class="new-login-button">新規作成してログイン</button>
      </div>
    </div>
  </div>
</template>

<script>
import axios from 'axios'
import app from '@/main.js'

export default {
  name: 'Login',
  data() {
    return {
        id: "",
        password: ""
    }
  },
  methods: {
    doLogin() {
      app.config.globalProperties.$http = axios; //最初の処理内に移動する
      let req = {
        id: 0,
        player_id: this.id,
        password: this.password,
        atodekesu: 0
      };
      this.$http.post('/login', req)
        .then(res => {
          console.log(res); //エラー処理、バリデーション、新規作成ログイン、未ログイン時/battleしても接続できないように
          app.config.globalProperties.$player_id = req.player_id;
          this.$router.push("/select");
        })
    }
  },
}
</script>

<style scoped lang="scss">
  .login-page {
    display: flex;
    justify-content: center;
    background-color: green(base);
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
      .input-area {
        display: flex;
        flex-direction: column;

        .input-box {
          display: flex;
          justify-content: center;
          width: 100%;     
          height: 24px;     
          &:first-child {
            margin-bottom: 20px;
          }

          div {
            display: flex;
            width: 30%;
            justify-content: flex-end;
            padding-right: 5px;
          }
        }
      }
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