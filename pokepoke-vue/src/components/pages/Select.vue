@import "@/assets/sass/main.scss";

<template>
  <SelectRoomPop v-if="isShowRoomPop" @closeRoomPop="isShowRoomPop = false" @goToRoom="goToRoom"></SelectRoomPop>
  <MakePokemonPop v-if="isShowMakePokemonPop" @closeMakePokemonPop="closeMakePokemonPop"></MakePokemonPop>
  <div class="select-page">
    <div class="select-box">
      <div class="create-area">
        <button class="submit-button green" @click="isShowMakePokemonPop = true">ポケモンを作成</button>
      </div>
      <div class="choose-area">
        <div class="title">ポケモンを選択</div>
        <div class="select-list" v-if="showSelectList">
          <div class="pokemon-box" v-for="pokemon in madePokemons" :key="pokemon.id">
            <input class="chosen-box" type="checkbox" v-model="checkedPokemons" :value="pokemon.id"
                    :disabled="checkedPokemons.filter(v => v==pokemon.id).length == 0 && checkedPokemons.length >= 6">
            <img :src="require('@/assets/images/' + pokemon.base_pokemon_id + '.png')">
            <div class="pokemon-name">{{pokemon.nickname}}</div>
          </div>
        </div>
      </div>
    </div>
    <button class="submit-button white" @click="isShowRoomPop = true" :disabled="checkedPokemons.length == 0">対戦する</button>
  </div>
</template>

<script>
// import axios from 'axios'
import SelectRoomPop from '@/components/modules/SelectRoomPop'
import MakePokemonPop from '@/components/modules/MakePokemonPop'
// import app from '@/main.js'

export default {
  name: 'Select',
  components: {
    SelectRoomPop,
    MakePokemonPop
  },
  props: {
    // checkedPokemons: []
  },
  data() {
    return {
      playerId: '',
      madePokemons: [],
      showSelectList: false,
      checkedPokemons: [],
      isShowRoomPop: false,
      isShowMakePokemonPop: false,
    }
  },
  methods: {
    getMadePokemon() {
      this.$http.get(`/made-pokemon/${this.playerId}`)
        .then(res => {
          console.log(res);
          this.madePokemons = res.data;
          this.showSelectList = true;
        })
    },
    goToRoom(roomId) {
      // app.config.globalProperties.$selected_pokemons = this.madePokemons.filter(v => this.checkedPokemons.some(x => x == v.id));
      // console.log(this.$selected_pokemons);
      let req = {
        id: 0,
        room_id: roomId,
        player_id: this.$player_id
      }
      this.checkedPokemons.map((v, i) => req["pokemon" + (i+1) + "_id"] = v);
      console.log(req);
      this.$http.post("/selected-pokemon", req)
        .then(res => {
          console.log("show" + res);
          this.$router.push("/show-each");
        })
    },
    closeMakePokemonPop(reloadFlg) {
      this.isShowMakePokemonPop = false;
      if (reloadFlg) {
        this.showSelectList = false;
        this.getMadePokemon();
      }
    }
  },
  created: function() {
    console.log("created");
    this.playerId = this.$player_id;
    this.getMadePokemon();
  }
}
</script>

<style scoped lang="scss">
  .select-page {
    padding: 50px;
    .select-box {
      background-color: white;
      border-radius: 2rem;
      width: 500px;
      padding: 10px;
      margin-bottom: 20px;
      
      .title {
        display: flex;
        align-items: center;
        justify-content: center;
        height: 80px;
      }
      .create-area {
      }
      .choose-area {
        height: 50vh;
        .select-list {
          display: flex;
          flex-direction: column;
          align-items: center;
          overflow: scroll;
          height: calc(100% - 80px);

          .pokemon-box {
            display: flex;
            width: 80%;
            img {
              width: 80px;
            }
            .pokemon-name {

            }
          }
        }
      }
    }
    .room-area {
      .input {
        display: flex;
        .input-box {
          
        }
      }
      .button {
        display: flex;
      }
    }
  }
</style>