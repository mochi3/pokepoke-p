@import "@/assets/sass/main.scss";

<template>
  <div class="pop-overwrap" @click.self="close()">
    <div class="battle-pokemon-change-box">
      <div class="pokemon-box" v-for="pokemon in formattedPokemons" :key="pokemon.made.id" :class="{field: pokemon.battle.number==fieldPokemonNumber}">
        <div class="pokemon-box-top">
          <img :src="require('@/assets/images/' + pokemon.made.base_pokemon_id + '.png')">
          <StatusBox :pokemon="pokemon" :nowHp="pokemon.now_hp"></StatusBox>
        </div>
        <button class="submit-button green" :disabled="pokemon.battle.number==fieldPokemonNumber" @click="onClickPokemonChange(pokemon.made.id)">交換する</button>
      </div>
    </div>
  </div>
</template>

<script>
import StatusBox from '@/components/modules/StatusBox'

export default {
  name: 'BattlePokemonChangePop',
  components: {
    StatusBox
  },
  props: {
    pokemons: [],
    fieldPokemonNumber: Number,
  },
  data() {
    return {
      formattedPokemons: [],
    }
  },
  methods: {
    close() {
      console.log("close");
      this.$emit('closePokemonChangePop', 0);
    },
    onClickPokemonChange(id) {
      this.$emit('closePokemonChangePop', id);
    }
  },
  created: function() {
    this.formattedPokemons = this.pokemons.map(v => v = {made: v[1], battle: v[0], now_hp: v[1].max_hp - v[0].hp_minus});
    console.log(this.formattedPokemons);
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
      justify-content: center;

      .pokemon-box {
        display: flex;
        flex-direction: column;
        align-items: center;
        margin-bottom: 20px;
        &:last-child {
          margin-bottom: 0;
        }

        .pokemon-box-top {
          display: flex;
          align-items: center;

          img {
            width: 80px;
          }
        }

        button {
          font-size: 0.9rem;
        }
      }
    }
  }
</style>