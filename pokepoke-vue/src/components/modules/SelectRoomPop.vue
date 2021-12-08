@import "@/assets/sass/main.scss";

<template>
  <div class="pop-overwrap" @click.self="close()">
    <div class="select-room-box">
      <div class="tab-area" @click="toggle()">
        <div :class="{active: toggleMakeIn}">作成</div>
        <div :class="{active: !toggleMakeIn}">入室</div>
      </div>
      <div class="input-area">
        <div class="input-box" v-if="!toggleMakeIn">
          <div>部屋ID</div>
          <input type="text" v-model.trim="roomId"/>
        </div>
        <div class="input-box">
          <div>パスワード</div>
          <input type="text" v-model.trim="roomPass"/>
        </div>
        <button @click="doRoom()">OK</button>
      </div>
    </div>
  </div>
</template>

<script>
import app from '@/main.js'

export default {
  name: 'SelectRoomPop',
  props: {
    // checkedPokemons: []
  },
  data() {
    return {
      roomId: '',
      roomPass: '',
      toggleMakeIn: true, 
    }
  },
  methods: {
    close() {
      console.log("close");
      this.$emit('closeRoomPop');
    },
    toggle() {
      this.toggleMakeIn = !this.toggleMakeIn;
    },
    doRoom() {
      let req = {
        room_id: this.roomId,
        password: this.roomPass,
        player1_id: this.$player_id,
        player2_id: 0,
        id: 0,
        is_double_battle: 0,
      }
      this.$http.post((this.toggleMakeIn)? '/make-room': '/make-room', req)
        .then(res => {
          app.config.globalProperties.$room_id = res.data.id //部屋id
          this.$emit('goToRoom',res.data.id); 
        })
    }
  }
}
</script>

<style scoped lang="scss">
  .pop-overwrap {

    .select-room-box {
      width: 300px;
      height: 300px;
      border-radius: 1rem;
      background-color: white;
      display: flex;
      // justify-content: center;
      align-items: center;
      flex-direction: column;

      .tab-area {
        display: flex;
        justify-content: space-around;
        width: 100%;
        div {
          width: 50%;
          padding: 1rem 0;
          background-color: #d9d9d9;
          &.active {
            background-color: white;
          }
          &:first-child {
            border-radius: 1rem 0 0 0 / 1rem 0 0 0;
          }
          &:last-child {
            border-radius: 0 1rem 0 0 / 0 1rem 0 0;
          }
        }
      }

      .input-area {
        display: flex;
        padding: 18% 0 15%;
        justify-content: space-between;
        height: 100%;
        .input-box {
          margin-bottom: 0;
        }
        button {
          font-size: 1.1rem;
        }
      }
    }
  }
</style>