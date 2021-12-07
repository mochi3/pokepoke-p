<template>
  <div class="show-each-page">
    <div class="name-area">
      <div>YOU</div>
      <div>.</div>
    </div>
    <div class="select-area">
      <div class="selct-pokemon-area my">
        <div class="pokemon-box" v-for="pokemon in myPokemons" :key="pokemon.id" @click="selectBattlePokemon(pokemon.id)">
          <div class="pokemon-battle-number">
            {{(selectedBattlePokemons.some(v => v == pokemon.id))? selectedBattlePokemons.findIndex(v => v == pokemon.id)+1: ''}}
          </div>
          <div class="pokemon-level">Lv. {{pokemon.level}}</div>
          <img :src="require('@/assets/images/' + pokemon.base_pokemon_id + '.png')">
        </div>
      </div>
      <div class="selct-pokemon-area your">
        <div class="pokemon-box" v-for="pokemon in yourPokemons" :key="pokemon.id">
          <div class="pokemon-battle-number"></div>
          <div class="pokemon-level">Lv. {{pokemon.level}}</div>
          <img :src="require('@/assets/images/' + pokemon.base_pokemon_id + '.png')">
        </div>
      </div>
    </div>
    <button :disabled="selectedBattlePokemons.length < 3" @click="onClickStart()">START</button>
  </div>
</template>

<script>

export default {
  name: 'ShowEach',
  components: {
  },
  data() {
    return {
      myPokemons: [],
      yourPokemons: [],
      selectedBattlePokemons: [],
    }
  },
  methods: {
    selectBattlePokemon(id) {
      if (this.selectedBattlePokemons.some(v => v == id)) {
        this.selectedBattlePokemons = this.selectedBattlePokemons.filter(v => v != id);
      } else if (this.selectedBattlePokemons.length < 3) {
        this.selectedBattlePokemons.push(id);
      }
    },
    onClickStart() {
    this.$http.post("/battle-pokemon", this.selectedBattlePokemons)
      .then(res => {
        console.log(res);
        this.$router.push("/battle");
      })
    }
  },
  created: function() {
    // this.$http.get(`/selected-pokemon/${this.$room_id}/${this.player_id}`)
    this.$http.get(`/selected-pokemon/14/1`)
        .then(res => {
          console.log(res);
          this.myPokemons = res.data[0];
          this.yourPokemons = res.data[1];
        })
  }
}
</script>

<style scoped lang="scss">
  .show-each-page {
    display: flex;
    justify-content: space-between;
    flex-direction: column;
    align-items: center;
    width: 500px;
    // height: 500px;
    padding: 50px 0;

    .name-area {
      display: flex;
      justify-content: space-between;
      width: 100%;

      div {
        width: 40%;
      }
    }

    .select-area {
      display: flex;
      justify-content: space-between;
      width: 100%;

      .selct-pokemon-area {
        background-color: white;
        width: 40%;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        border-radius: 1rem;
        margin: 20px 0;

        &.my .pokemon-box {
          cursor: pointer;
          &:hover {
            background-color: #eee;
          }
        }

        .pokemon-box {
          width: 80%;
          display: flex;
          justify-content: space-around;

          .pokemon-battle-number {
            color: #bd7440;
            font-size: 1.5rem;
          }
          .pokemon-level {

          }
          img {
            width: 80px;
          }
        }
      }
    }

    button {
      width: 30%;
      font-size: 1.2rem;
    }
  }
</style>
