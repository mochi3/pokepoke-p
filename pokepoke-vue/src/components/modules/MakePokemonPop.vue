@import "@/assets/sass/main.scss";

<template>
  <div class="pop-overwrap" @click.self="close()">
    <div class="make-pokemon-box">
      <div class="make-area basic">
        <div class="pokemon-img">
          <img v-if="true" :src="require('@/assets/images/' + confirmPokemon.id + '.png')">
        </div>
        <div class="basic-datas">
          <label for="base-name">ポケモン名
            <input type="text" id="search" placeholder="検索" v-model="searchPokemonTerm" @input="showSearchPokemon=true">
            <ul v-if="searchPokemons.length && showSearchPokemon">
              <li v-for="pokemon in searchPokemons" :key="pokemon.id" @click="onClickPokemonName(pokemon)">
                {{ pokemon.name }}
              </li>
            </ul>
          </label>
          <label for="nickname">ニックネーム
            <input type="text" id="nickname" v-model="nickname"> 
          </label>
          <div>
            <label for="ability">とくせい
              <select id="ability" v-if="abilities.length" v-model="ability">
                <option v-for="ability in abilities" :key="ability.id" :value="ability.id">{{ability.name}}</option>
              </select>
            </label>
            <label for="ability">性別
              <select id="gender" v-if="genders.length" v-model="gender">
                <option v-for="gender in genders" :key="gender.id" :value="gender.id">{{gender.name}}</option>
              </select>
            </label>
          </div>
        </div>
      </div>
      <div class="make-area status">
        <div class="level-nature">
          <label>Lv.
            <input type="number" id="level" min="1" max="50" v-model="level">
          </label>
          <label>性格
            <select id="nature" v-if="natures.length" v-model="nature">
              <option v-for="nature in natures" :key="nature.id" :value="nature">{{nature.name}}</option>
            </select>
          </label>
        </div>
        <div class="set-status">
          <div class="status" v-for="sta in STATUS" :key="sta.id">
            <p>{{sta.name}}</p>
            <input type="number" v-model="iv[sta.id]" min="0" max="31">
            <button @click="ev[sta.id]=0">0</button>
            <input type="number" v-model="ev[sta.id]" min="0" max="252">
            <button @click="ev[sta.id]=252">252</button>
            <p>{{status(sta.id)}}</p>
          </div>
        </div>
      </div>
      <div class="make-area move-item">
        <div class="move">
          <label v-for="n of 4" :key="n">わざ{{n}}
            <input type="text" placeholder="検索" v-model="searchMoveTerms[n-1]" @input="showSearchMoves[n-1]=true">
            <ul v-if="searchMoves(n).length && showSearchMoves[n-1]">
              <li v-for="move in searchMoves(n)" :key="move.id" @click="onClickMove(n, move)">
                {{ move.name }}
              </li>
            </ul>
          </label>
        </div>
        <div class="item-button">
          <label>どうぐ
            <input type="text" placeholder="検索" v-model="searchItemTerm" @input="showSearchItem=true">
            <ul v-if="searchItems.length && showSearchItem">
              <li v-for="item in searchItems" :key="item.id" @click="onClickItem(item)">
                {{ item.name }}
              </li>
            </ul>
          </label>
          <button class="submit-button green" @click="onClickSubmit()" :disabled="submitDisabled">作成</button>
        </div>
      </div>

    </div>
  </div>
</template>

<script>
import const_modules from "@/const-data.js";
// import VueSimpleSuggest from 'vue-simple-suggest'
// import 'vue-simple-suggest/dist/styles.css' // Optional CSS
// import {ref, computed} from 'vue'

export default {
  name: 'MakePokemonPop',
  components: {
    // VueSimpleSuggest
  },
  props: {
    // checkedPokemons: []
  },
  data() {
    return {
      natures: const_modules.NATURES,
      STATUS: const_modules.STATUS,
      genders: [],
      gender: null,
      abilities: [],
      ability: null,
      baseName: '',
      basePokemons: [],
      searchPokemonTerm: '',
      showSearchPokemon: true,
      confirmPokemon: {id: 0},
      nickname: '',
      nature: const_modules.NATURES.filter(v => v.id==21)[0],
      level: 50,
      base_v: [],
      iv: new Array(6).fill(31),
      ev: new Array(6).fill(0),
      v: [],
      searchMoveTerms: new Array(4).fill(''),
      showSearchMoves: new Array(4).fill(true),
      baseMoves: [{id:1, name: "たいあたり"}, {id:2, name: "ころがる"}],
      confirmMoveIds: new Array(4).fill(0),
      searchItemTerm: '',
      showSearchItem: true,
      baseItems: [{id: 1, name: "あかいいと"}, {id: 2, name: "こだわりハチマキ"}],
      confirmItemId: 0,
    }
  },
  methods: {
    close() {
      this.$emit('closeMakePokemonPop', false);
    },
    returnBasePokemons() {
      return ['aaa','bbb'];
    },
    onClickPokemonName(pokemon) {
      this.searchPokemonTerm = pokemon.name;
      this.showSearchPokemon = false;
      this.confirmPokemon = pokemon;
      this.genders = const_modules.GENDERS.filter(v => (pokemon.gender_flg == 0 && v.id != 3) || (pokemon.gender_flg == 1 && v.id == 3));
      this.abilities = const_modules.ABILITIES.filter(v => v.id == pokemon.ability1_id || v.id == pokemon.ability2_id || v.id == pokemon.ability3_id);
      this.base_v = this.STATUS.map(v => pokemon[v.value + "_base"]);
      this.gender = this.genders[0].id;
      this.ability = this.abilities[0].id;
      // this.v = this.STATUS.map(v => this.calculateStatus(v.id));
    },
    onClickMove(n, move) {
      this.searchMoveTerms[n-1] = move.name;
      this.showSearchMoves[n-1] = false;
      this.confirmMoveIds[n-1] = move.id;
    },
    onClickItem(item) {
      this.searchItemTerm = item.name;
      this.showSearchItem = false;
      this.confirmItemId = item.id;
    },
    onClickSubmit() {
      let madePokemon = {
        id: 0, 
        player_id: this.$player_id,
        base_pokemon_id: this.confirmPokemon.id,
        nickname: this.nickname != ''? this.nickname: this.confirmPokemon.name,
        level: this.level,
        gender_id: this.gender,
        ability_id: this.ability,
        nature_id: this.nature.id,
        item_id: this.searchItemTerm == ''? 0: this.confirmItemId,
      };
      console.log(this.confirmMoveIds[0]);
      [1,2,3,4].map(v => madePokemon["move" + v + "_id"] = this.confirmMoveIds[v-1]);
      this.STATUS.map(v => {
        madePokemon[v.value + "_iv"] = this.iv[v.id];
        madePokemon[v.value + "_ev"] = this.ev[v.id];
        if (v.value=="h") {
          madePokemon.max_hp = this.status(v.id)
        } else {
          madePokemon[v.value + "_v"] = this.status(v.id);
        }
      })
      this.$http.post("/make-pokemon", madePokemon)
        .then(res => {
          console.log(res);
          this.$emit('closeMakePokemonPop', true);
        })
    },
    // onClickEvInput(value) {

    // }
    // calculateStatus(id) {
    //   if (id==0) {
    //     return Math.floor((this.base_v[id]*2+this.iv[id]+Math.floor(this.ev[id]/4))*this.level/100)+10+this.level
    //   } else {
    //     let natureTimes = this.nature.up == id ? 1.1: this.nature.down == id ? 0.9: 1;
    //     console.log(natureTimes);
    //     return Math.floor((Math.floor((this.base_v[id]*2+this.iv[id]+Math.floor(this.ev[id]/4))*this.level/100)+5)*natureTimes)
    //   }
    // }
  },
  created: function() {
    this.$http.get("/make-pokemon")
      .then(res => {
        console.log(res);
        this.basePokemons = res.data[0];
        this.baseMoves = res.data[1];
      })
  },
  computed: {
    searchPokemons: function() {
      if (this.searchPokemonTerm === '') {
        return []
      }
      return this.basePokemons.filter(v => v.name.includes(this.searchPokemonTerm)).slice(0,10);
      // let matches = 0
      // return this.basePokemons.filter(v => {
      //   if (v.name.includes(this.searchPokemonTerm) && matches < 10) {
      //     matches++
      //     return v
      //   }
      // })
    },
    searchMoves: function() {
      return function(n) {
        if (this.searchMoveTerms[n-1] === '') {
          return []
        }
        return this.baseMoves.filter(v => v.name.includes(this.searchMoveTerms[n-1])).slice(0,10)
      }
    },
    searchItems: function() {
      if (this.searchItemTerm === '') {
        return []
      }
      return this.baseItems.filter(v => (v.name.includes(this.searchItemTerm))).slice(0,10)
    },
    status: function() {
      return function(id) {
        if (this.base_v.length==0) {
          return ''
        }else if (id==0) {
          return Math.floor((this.base_v[id]*2+this.iv[id]+Math.floor(this.ev[id]/4))*this.level/100)+10+this.level
        } else {
          let natureTimes = this.nature.up == id ? 1.1: (this.nature.down == id ? 0.9: 1);
          console.log(natureTimes);
          return Math.floor((Math.floor((this.base_v[id]*2+this.iv[id]+Math.floor(this.ev[id]/4))*this.level/100)+5)*natureTimes)
        }
      }
    },
    submitDisabled: function() {
      return !(this.confirmPokemon.id != 0 && 
        this.confirmMoveIds.some(v => v!=0) && 
        this.ev.reduce((sum, ele) => sum + ele, 0) <= 510)
    }
  },
  watch: {
    // searchPokemonTerm: function() {
    //   console.log("watch");
    //   this.showSearchPokemon = true;
    // },
    level: function() {
        if (this.level <= 50 && this.level >= 1) {
          return
        }
        this.level = this.level > 50 ? 50: (this.level < 1 ? 1: this.level)
    },
    iv: {
      handler() {
        if (this.iv.every(v => v <= 31 && v >= 0)) {
          return
        }
        this.iv = this.iv.map(v => v > 31 ? 31: (v < 0 ? 0: v))
      },
      deep: true
    },
    ev: {
      handler() {
        if (this.ev.every(v => v <= 252 && v >= 0)) {
          return
        }
        this.ev = this.ev.map(v => v > 252 ? 252: (v < 0 ? 0: v))
      },
      deep: true
    },
  }
}
</script>

<style scoped lang="scss">
  .pop-overwrap {

    .make-pokemon-box {
      width: 500px;
      height: 500px;
      border-radius: 1rem;
      background-color: white;
      display: flex;
      // justify-content: center;
      align-items: center;
      flex-direction: column;
      padding: 8px 12px;

      .make-area {
        display: flex;
        align-items: flex-end;
        justify-content: center;
        justify-content: space-between;
        width: 80%;
        padding: 17px 0;
        border-bottom: solid 3px #c5dbcf;
        &:last-child {
          border-bottom: none;
        }

        label {
          font-size: 0.2rem;
          text-align: start;
          color: #5b605d;
          display: flex;
          flex-direction: column; 
          margin: 2px 5px;
          position: relative;

          ul {
            position: absolute;
            top: 33px;
            z-index: 5;
            background-color: white;
            border: solid 1px #738179;
            padding: 0;
            width: 80%;
            margin: 0;

            li {
              list-style-type: none;
              padding: 0 5px;

              &:hover {
                background-color: #eee;
              }
            }
          }
        } 
        input {
          border: none;
          border-bottom: solid 2px #73817a;
          background-color: #f1f1f1;
          &:focus-visible {
            outline: #4ea177 auto 1px;
          }
        }
        select {
          border: #919f8e solid 2px;
          border-radius: 3px;
          &:focus-visible {
            outline: #4ea177 auto 1px;
          }
        }
        &.basic {
          input {
            width: 200px;
          }
          .pokemon-img {
            img {
              width: 100px;
            }
          }
          .basic-datas {
            display: flex;
            flex-direction: column;
            // margin-left: 8px;

            div {
              display: flex;
            }
          }
        }
        &.status {
          align-items: flex-start;
          input {
            width: 3rem;
          }
          p {
            margin: 0;
            font-size: 0.8rem;
            width: 4rem;
          }
          .set-status {
            width: 70%;
            & > div {
              display: flex;
              justify-content: space-between;

              button {
                border: solid 1px #738179;
                padding: 1px 4px;
                border-radius: 10px;

                &:hover {
                  background-color: #eee;
                  color: green(dark);
                }
              }
            }
          }
        }
        &.move-item {
          .item-button {
            display: flex;
            flex-direction: column;
            justify-content: space-between;
            height: 100%;
            align-items: flex-end;

            button {
              width: 7rem;
            }
          }
        }
      }

    }
  }
</style>