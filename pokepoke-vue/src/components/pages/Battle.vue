@import "@/assets/sass/main.scss";
<template>
  <div class="battle-page">

    <div class="battle-area">
      <div class="field-area">
        <div class="pokemon-area my">
          <div class="status-box" v-if="show_moves_area">
            <div class="status-top">
                <div class="pokemon-name">{{my_field_pokemon.base.name}}</div>
              <div class="status-top-right">
                <div class="pokemon-gender">{{my_field_pokemon.made.gender_name}}</div>
                <div class="pokemon-level">Lv. {{my_field_pokemon.made.level}}</div>
              </div>
            </div>
            <div class="hp-box">
              <div class="hp-base">
                <div class="hp-hp">HP</div>
                <div class="hp-bar-outer">
                  <div class="hp-bar-inner" v-bind:style="{ width: (my_hp_new / my_field_pokemon.made.max_hp)*100 + '%' }"></div>
                </div>
              </div>
            </div>
            <div class="status-under">
              <div class="hp-number">{{my_hp_new}} / {{my_field_pokemon.made.max_hp}}</div>
              <div class="ailment">やけど</div>
            </div>
          </div>
          <div class="pokemon-shadow"></div>
          <img src="@/assets/images/gaburias.png">
        </div>
        <div class="pokemon-area your">
          <div class="status-box" v-if="show_moves_area">
            <div class="status-top">
                <div class="pokemon-name">{{your_field_pokemon.base.name}}</div>
              <div class="status-top-right">
                <div class="pokemon-gender">{{your_field_pokemon.made.gender_name}}</div>
                <div class="pokemon-level">Lv. {{your_field_pokemon.made.level}}</div>
              </div>
            </div>
            <div class="hp-box">
              <div class="hp-base">
                <div class="hp-hp">HP</div>
                <div class="hp-bar-outer">
                  <div class="hp-bar-inner" v-bind:style="{ width: (your_hp_new / your_field_pokemon.made.max_hp)*100 + '%' }"></div>
                </div>
              </div>
            </div>
            <div class="status-under">
              <div class="hp-number">{{your_hp_new}} / {{your_field_pokemon.made.max_hp}}</div>
              <div class="ailment"></div>
            </div>
          </div>
          <div class="pokemon-shadow"></div>
          <img src="@/assets/images/kabarudon.png">          
        </div>
      </div>
      <!-- メッセージ欄 -->
      <div class="message-area">
        <div class="left-message-box">
          <div class="moves-area" v-if="show_moves_area">
            <button v-for="move in my_field_pokemon.moves" :key="move.id" @click="doMove(move.id)" v-bind:style="{ backgroundColor: move.type_color }">
              <div class="move-name">{{move.name}}</div>
              <div class="move-under">
                <div class="move-type">{{move.type_name}}</div>
                <div class="move-pp">PP {{move.remain_pp}} / {{move.max_pp}}</div>
              </div>
            </button>
          </div>
        </div>
        <div class="right-message-box">
          <button v-for="item in first_commands" :key="item.id" @click="getMadePokemons(item.id)">
            {{item.name}}
          </button>
        </div>
      </div>
    </div>


  </div>
</template>

<script>

import const_modules from "@/const-data.js";
import axios from 'axios'

export default {
  name: 'Battle',
  props: {
    // msg: String,
  },
  data() {
    return {
      show_moves_area: false,
      first_commands: [
        {
          name: "たたかう",
          id: 1
        },
        {
          name: "どうぐ",
          id: 2
        },
        {
          name: "ポケモン",
          id: 3
        },
        {
          name: "にげる",
          id: 4
        },
      ],
      // moves: [],
      my_field_pokemon: {},
      my_hp_new: 0,
      your_field_pokemon: {},
      your_hp_new: 0,
    }
  },
  methods: {
    getMadePokemons(id) {
        if (id == 1) {
          axios.get('/pokemon-and-relative/1')
            .then(res => {
              console.log(res);
              this.my_field_pokemon = this.addStatusForDisplay(res);
              this.my_hp_new = this.my_field_pokemon.made.max_hp - this.my_field_pokemon.battle.hp_minus;
              // this.show_moves_area = true;
              axios.get('/pokemon-and-relative/2')
                .then(res => {
                  console.log(res);
                  this.your_field_pokemon = this.addStatusForDisplay(res);
                  this.your_hp_new = this.your_field_pokemon.made.max_hp - this.your_field_pokemon.battle.hp_minus;
                  this.show_moves_area = true;
              console.log(136);
                });
            });
        }
    },
    addStatusForDisplay(res) {
      //技名、タイプ名、色いれる
      res.data.moves[1].map((v,i) => {
        v.remain_pp = v.max_pp - eval("res.data.battle_pokemon[0].move" + (i+1) + "_pp_minus");
        v.type_name = const_modules.TYPES.filter(t => t.id == v.type_id)[0].name;
        v.type_color = const_modules.TYPES.filter(t => t.id == v.type_id)[0].color;
      });
      //性別名いれる
      res.data.made_pokemon[0].gender_name = const_modules.GENDER.filter(t => t.id == res.data.made_pokemon[0].gender_id)[0].name;
      let field_pokemon = {
        base: res.data.base_pokemon[0],
        made: res.data.made_pokemon[0],
        battle: res.data.battle_pokemon[0],
        moves: res.data.moves[1],
      };
      return field_pokemon;
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">
  .battle-page {
    background-color: green(base);
    display: flex;
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

      .field-area {
        height: 75%;
        display: flex;
        padding: 0 6vw;
        position: relative;

        //-------------ポケモン画像--------------
        img {
          width: 20vw;
          z-index: 3;
        }
        .pokemon-area {
          width: 50%;
          display: flex;
          flex-wrap: wrap;
          flex-direction: column;
          position: relative;
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
        //------------ステータス共通--------------------------------------
        .status-box {
          border: solid 2px #3c3c3c;
          width: 20vw;
          padding: 1% 2% 0;
          border-radius: 0.3rem;
          box-shadow: 6px 7px 4px -1px #a8ada8;
          position: absolute;
          //-----------ステータス上部------------------
          .status-top {
            display: flex;
            justify-content: space-between;
            
            .pokemon-name {

            }
            .status-top-right {
              display: flex;
              width: 35%;
              justify-content: space-between;
              .pokemon-gender {
                padding-top: 6%;
                font-size: 78%;
              }
              .pokemon-level {
                font-size: 94%;
                margin-top: 4%;
              }
            }
          }//---------------HP-------------------
          .hp-box {
            .hp-base {
              width: 99%;
              height: 0.5rem;
              background-color: #3c3c3c;
              border-radius: 0.5rem;
              display: flex;
              padding: 0.05rem;
              align-items: center;
              justify-content: space-around;

              .hp-hp {
                color: #cf4e4e;
                font-weight: bold;
                font-size: 0.2rem;
                width: 9%;
              }
              .hp-bar-outer {
                width: 83%;
                height: 60%;
                .hp-bar-inner {
                  width: 100%;
                  height: 100%;
                  background-color: #7ac175;
                  border-radius: 0.3rem;
                }
              }
            }
          }//-----------ステータス下部------------------
          .status-under {
            display: flex;
            justify-content: space-between;
            font-size: 0.9rem;
            .hp-number {

            }
            .ailment {
              font-size: 0.5rem;
              border-radius: 1rem;
              padding: 0 0.5rem;
              margin: 0.1rem 0;
              background-color: #af4c13;
              color: white;
              display: flex;
              align-items: center;
              box-shadow: 3px 4px 3px -2px #686868;
            }
          }
        }
        //--------------ステータス個別--------------------------
        .pokemon-area.my .status-box {
          right: -7vw;
          bottom: 11vw;
        }
        .pokemon-area.your .status-box {
          top: 3vw;
          left: -8vw;        }
      }
      //------------メッセージ欄----------------------
      .message-area {
        display: flex;
        height: 25%;

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

          //-----------------わざ-----------------------------
          button {
            border: solid 2px #3c3c3c;
            border-radius: 5px;
            box-shadow: 3px 5px 4px -1px #a8ada8;
            color: #3c3c3c;
            padding: 0.2rem 1rem;
            display: flex;
            align-items: center;

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
        .right-message-box {
          @extend %message-box;
          width: 30%;
          margin-left: 3px;
          flex-wrap: wrap;
          justify-content: center;
          padding: 0.6rem 0;
          width: 30%;
          margin-left: 3px;

          button {
            font-size: 1rem;
          }
        }
      }
    }
  }

</style>
