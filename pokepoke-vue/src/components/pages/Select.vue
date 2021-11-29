@import "@/assets/sass/main.scss";

<template>
  <div class="select-page">
    <div class="select-box">
      <div class="create-area">
        <div class="title">ポケモンを作成</div>
      </div>
      <div class="choose-area">
        <div class="title">ポケモンを選択</div>
      </div>
    </div>
  </div>
</template>

<script>
// import axios from 'axios'
import app from '@/main.js'

export default {
  name: 'Select',
  data() {
    return {
    }
  },
  methods: {
    doLogin() {
    }
  },
  created: function() {
    this.$http.post('/login', req)
      .then(res => {
        console.log(res); //エラー処理、バリデーション、新規作成ログイン、未ログイン時/battleしても接続できないように
        app.config.globalProperties.$player_id = req.player_id;
        this.$router.push("/select");
      })
  }
}
</script>

<style scoped lang="scss">
  .select-page {
  }
</style>