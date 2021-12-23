@import "@/assets/sass/main.scss";
<template>
  <BattlePokemonChangePop v-if="isShowPokemonChangePop" @closePokemonChangePop="closePokemonChangePop"
   :pokemons="forPokemonChangePopValue" :fieldPokemonNumber="my_field_pokemon.battle.number"></BattlePokemonChangePop>

  <div class="battle-page">
    <div class="battle-area">
      <div class="field-area">
        <div class="pokemon-area my" v-if="showMyPokemonArea">
          <StatusBox :pokemon="my_field_pokemon" :nowHp="my_field_pokemon.now_hp"></StatusBox>
          <div class="pokemon-shadow"></div>
          <img :src="require('@/assets/images/' + my_field_pokemon.base.id + '.png')" :class="{death: my_field_pokemon.death}">
        </div>
        <div class="pokemon-area my" v-if="!showMyPokemonArea">
        </div>
        <div class="pokemon-area your" v-if="showYourPokemonArea">
          <StatusBox :pokemon="your_field_pokemon" :nowHp="your_field_pokemon.now_hp"></StatusBox>
          <div class="pokemon-shadow"></div>
          <img :src="require('@/assets/images/' + your_field_pokemon.base.id + '.png')" :class="{death: your_field_pokemon.death}">          
        </div>
      </div>
      <!-- メッセージ欄 -->
      <div class="message-area">
        <div class="left-message-box">
          <div class="moves-area" v-if="showMovesArea">
            <button v-for="move in my_field_pokemon.moves" :key="move.id" @click="onClickMove(move.id)" v-bind:style="{ backgroundColor: move.type_color }">
              <div class="move-name">{{move.name}}</div>
              <div class="move-under">
                <div class="move-type">{{move.type_name}}</div>
                <div class="move-pp">PP {{move.remain_pp}} / {{move.max_pp}}</div>
              </div>
            </button>
          </div>
          <div class="narration-area" v-if="narration != ''">
            {{narration}}
          </div>
        </div>
        <div class="right-message-box" v-if="showButtonArea">
          <button v-for="item in first_commands" :key="item.id" @click="onClickCommand(item.id)">
            {{item.name}}
          </button>
        </div>
        <div class="right-message-box" v-else></div>
      </div>
    </div>


  </div>
</template>

<script>
import BattlePokemonChangePop from '@/components/modules/BattlePokemonChangePop'
import StatusBox from '@/components/modules/StatusBox'
import const_modules from "@/const-data.js";
import { toRaw } from 'vue';

export default {
  name: 'Battle',
  components: {
    BattlePokemonChangePop,
    StatusBox
  },
  props: {
    // msg: String,
  },
  data() {
    return {
      showButtonArea: false,
      showNarrationArea: false,
      showMovesArea: false,
      showMyPokemonArea: false,
      showYourPokemonArea: false,
      isShowPokemonChangePop: false,
      deathFlg: false,
      first_commands: const_modules.COMMANDS,
      // moves: [],
      my_field_pokemon: {now_hp: 0},
      // my_hp_new: 0,
      myPokemonDeath: false,
      your_field_pokemon: {now_hp: 0},
      // your_hp_new: 0,
      yourPokemonDeath: false,
      narration: "",
      forPokemonChangePopValue: [],
      sleep: (ms) => new Promise((resolve) => setTimeout(resolve, ms)),
    }
  },
  methods: {
    // getMadePokemons(id) {
    //     if (id == 1) {
    //       axios.get('/pokemon-and-relative/1')
    //         .then(res => {
    //           console.log(res);
    //           this.my_field_pokemon = this.formatPokemonForDisplay(res);
    //           this.my_hp_new = this.my_field_pokemon.made.max_hp - this.my_field_pokemon.battle.hp_minus;
    //           // this.showMovesArea = true;
    //           axios.get('/pokemon-and-relative/3')
    //             .then(res => {
    //               console.log(res);
    //               this.your_field_pokemon = this.formatPokemonForDisplay(res);
    //               this.your_hp_new = this.your_field_pokemon.made.max_hp - this.your_field_pokemon.battle.hp_minus;
    //               this.showMovesArea = true;
    //           console.log(136);
    //             });
    //         });
    //     }
    // },
    formatPokemonForDisplay(pokemon) {
      //技名、タイプ名、色いれる
      pokemon.moves.map((v,i) => {
        v.remain_pp = v.max_pp - pokemon.battle_pokemon["move" + (i+1) + "_pp_minus"];
        v.type_name = const_modules.TYPES.filter(t => t.id == v.type_id)[0].name;
        v.type_color = const_modules.TYPES.filter(t => t.id == v.type_id)[0].color;
      });
      //性別名いれる
      pokemon.made_pokemon.gender_name = const_modules.GENDERS.filter(t => t.id == pokemon.made_pokemon.gender_id)[0].name;
      let field_pokemon = {
        base: pokemon.base_pokemon,
        made: pokemon.made_pokemon,
        battle: pokemon.battle_pokemon,
        moves: pokemon.moves,
        now_hp: pokemon.made_pokemon.max_hp - pokemon.battle_pokemon.hp_minus
      };
      return field_pokemon;
    },
    onClickCommand(id) {
      this.narration = '';
      this.showMovesArea = false;
      switch (id) {
        case 1: //たたかう
          this.showMovesArea = true;
          break;
        case 3: //ポケモン交換
          this.openPokemonChangePop();
          break;
        case 2:
        case 4:
          this.onClickNgButton(id)
          break;
      }
    },
    openPokemonChangePop() { //ポケモン交換
      this.$http.get(`/battle-change-pokemons/${this.$player_id}`)
        .then(res => {
          console.log(res);
          this.forPokemonChangePopValue = res.data;
          this.isShowPokemonChangePop = true;
        })
    },
    closePokemonChangePop(selectId) {
      if (this.deathFlg && !selectId) { //死んでる時は選択しないと閉じれない
        return;
      }
      this.isShowPokemonChangePop = false;
      if (selectId) { //交換ポケモン選択時
        this.postBattle(2, selectId)
      }
    },
    onClickMove(id) { //技選択
      this.postBattle(1, id);
    },
    postBattle(command_type, command_id) {
      this.showButtonArea = false;
      this.showMovesArea = false;
      this.narration = '通信待機中...';
      let req = {
        id: 0,
        room_id: 14,
        player_id: this.$player_id,
        turn: 0,
        command_type: command_type,
        command_id: command_id,
      }
      this.$http.post(`/do-command/14/${this.$player_id}`, req) 
        .then(res => {
          console.log(res);
          this.doShowBattle(res.data);
        })
    },
    async firstTurn() {
      this.narration = `相手は ${this.your_field_pokemon.made.nickname} をくり出してきた！`;
      await this.sleep(1000);
      this.showYourPokemonArea = true;
      await this.sleep(1000);
      this.narration = `いけ！ ${this.my_field_pokemon.made.nickname} ！`;
      await this.sleep(1000);
      this.showMyPokemonArea = true;
      await this.sleep(1000);
      this.narration = `${this.my_field_pokemon.made.nickname} はどうする？`;
      this.showButtonArea = true;
    },
    async onClickNgButton(id) {
      this.narration = '';
      await this.sleep(500);
      this.narration = id == 4? `逃げられない！`: `どうぐは使えない！`;
      await this.sleep(1000);
      this.showButtonArea = true;
      this.narration = `${this.my_field_pokemon.made.nickname} はどうする？`;
    },
    async doShowBattle(data) {
      let showBattles = data[0];
      let changePokemons = data[1];
      await this.sleep(1000);
      for (let showBattle of showBattles) {
        let pokemon = showBattle.player_id == this.$player_id? toRaw(this.my_field_pokemon): toRaw(this.your_field_pokemon);
        switch (showBattle.kind) {
          case 100: //技使うメッセージ
            await this.sleep(1000);
            this.narration = `${pokemon.made.nickname} の ${showBattle.name_string} ！`;
            break;
          case 101:  //HP減らす処理
            await this.sleep(1000);
            pokemon.now_hp = (pokemon.now_hp - showBattle.value > 0)? pokemon.now_hp - showBattle.value: 0;
            await this.sleep(500);
            break;
          case 102: //急所メッセージ
            await this.sleep(1000);
            this.narration = `急所に当たった！`;
            break;
          case 110: //技外し
            await this.sleep(1000);
            this.narration = `しかし ${pokemon.made.nickname} のこうげきは外れた！`;
            break;
          case 120: //死亡
            await this.sleep(1000);
            this.narration = ` ${pokemon.made.nickname} は倒れた！`;
            pokemon.death = true;
            if (pokemon == this.my_field_pokemon) { //自分のポケモンが倒れた
              await this.sleep(1000);
              this.openPokemonChangePop();
              this.deathFlg = true;
            } else { //相手のポケモンが倒れた
              this.$http.get(`/wait-your-death/14/${this.$player_id}`) //相手のレスポンスがあるまで向こうで待機
                .then(res => {
                  console.log(res);
                })
            }
            break;
          case 200: //交換
            {await this.sleep(1000);
            let changePokemon = changePokemons.filter(v => v.made_pokemon.id == showBattle.pokemon_id)[0];
            this.narration = (showBattle.player_id == this.$player_id)? `いけ！ ${changePokemon.made_pokemon.nickname} ！`: `相手は ${changePokemon.made_pokemon.nickname} をくり出してきた！`;
            pokemon.death = true;
            await this.sleep(500);
            Object.assign(pokemon, this.formatPokemonForDisplay(changePokemon));//再代入すると新参照になってしまうため
            await this.sleep(500);
            pokemon.death = false;
          }
        }
      }
      await this.sleep(1000);
      this.narration = `${this.my_field_pokemon.made.nickname} はどうする？`;
      this.showButtonArea = true;
    }
  },
  created: function () {
    this.$http.get(`/battle-pokemon/14/${this.$player_id}`)
      .then(res => {
        console.log(res);
        this.my_field_pokemon = this.formatPokemonForDisplay(res.data[0]);
        this.your_field_pokemon = this.formatPokemonForDisplay(res.data[1]);
        this.firstTurn();
      })
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">
  .battle-page {
    padding: 50px;
    font-weight: bold;

    .battle-area {
      display: flex;
      flex-direction: column;
      width: 80vw;
      height: 60vh;
      // border: solid 5px #839583;
      border-radius: 10px;
      background-color: #ffffff;
      max-width: 50rem;
      // background-image: url(../../assets/images/ground.jpeg);
      // background-size: cover;

      .field-area {
        height: 75%;
        display: flex;
        padding: 0 6vw;
        position: relative;

        //-------------ポケモン画像--------------
        img {
          width: 20vw;
          z-index: 3;
          &.death {
            animation: death 1s linear 0s;
            animation-fill-mode: forwards;
          }
        }
        @keyframes death {
          0% {
            opacity: 1;
          }
          100% {
            opacity: 0;
            display:none;
          }
        }
        .pokemon-area {
          width: 50%;
          display: flex;
          flex-wrap: wrap;
          flex-direction: column;
          position: relative;
          // animation: move 1s infinite;
        }
        @keyframes move {
          0% {
            padding: 0 0;
          }
          50% {
            padding: 4px 0;
          }
        }
        .pokemon-shadow {
            background-color: #a9a9a9;
            height: 2vw;
            width: 12vw;
            border-radius: 100%;
            position: absolute;
        }
        .pokemon-area.my {
          justify-content: flex-end;
          align-items: flex-start;

          .pokemon-shadow {
            bottom: 11px;
            left: 4vw;
          }
        }
        .pokemon-area.your {
          justify-content: flex-start;
          align-items: flex-end;

          .pokemon-shadow {
            top: 15vw;
            right: 3vw;
          }
        }
        //--------------ステータス個別--------------------------
        .status-box {
          position: absolute;
        }
        .pokemon-area.my .status-box {
          right: -7vw;
          bottom: 11vw;
        }
        .pokemon-area.your .status-box {
          top: 3vw;
          left: -8vw;
        }
      }
      //------------メッセージ欄----------------------
      .message-area {
        display: flex;
        height: 28%;

        %message-box {
          border: solid 4px green(light-gray);
          margin: 6px;
          border-radius: 10px;
          display: flex;
          box-shadow: 2px 3px 4px -2px #bcc9bd;
        }
        .left-message-box {
          @extend %message-box;
          width: 70%;
          margin-right: 3px;
          padding: 0.5rem 1.2rem;
          background-color: white;

          //-----------------わざ-----------------------------
          .moves-area {
            display: flex;
            flex-wrap: wrap;
            justify-content: space-between;
            align-items: center;

            button {
              border: solid 2px #3c3c3c;
              border-radius: 5px;
              box-shadow: 3px 5px 4px -1px #a8ada8;
              color: #3c3c3c;
              padding: 0.1rem 0.6rem;
              display: flex;
              align-items: center;
              justify-content: space-between;
              width: 48%;
              height: 45%;
              font-size: 0.8rem;

              .move-name {
              }
              .move-under {
                display: flex;
                align-items: center;
                font-size: 0.3rem;
                flex-direction: column;
                margin-left: 1rem;
                color: white;

                .move-type {
                  border: solid 1px #ffffff;
                  border-radius: 2rem;
                  padding: 0.05rem 0.4rem;
                  box-shadow: 2px 3px 4px -2px #686868;
                  font-size: 0.2rem;        
                  background-color: #00000038;      
                }
                .move-pp {
                }
              }
            }
          }
        }
        .right-message-box {
          @extend %message-box;
          width: 30%;
          margin-left: 3px;
          flex-wrap: wrap;
          justify-content: center;
          padding: 0.6rem 0;
          width: 30%;
          margin-left: 3px;
          background-color: white;

          button {
            font-size: 1rem;
          }
        }
      }
    }
  }

</style>
