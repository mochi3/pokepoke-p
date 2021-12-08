@import "@/assets/sass/main.scss";

<template>
  <div class="pop-overwrap" @click.self="close()">
    <div class="battle-pokemon-change-box">
      <div class="pokemon-box" v-for="pokemon in madePokemons" :key="pokemon.id">
        <img :src="require('@/assets/images/' + pokemon.base_pokemon_id + '.png')">
        <div class="pokemon-name">{{pokemon.nickname}}</div>
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
      this.$emit('closeChangePop');
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

    .battle-pokemon-change-box {
      width: 300px;
      height: 500px;
      border-radius: 1rem;
      background-color: white;
      display: flex;
      // justify-content: center;
      align-items: center;
      flex-direction: column;
    }
  }
</style>