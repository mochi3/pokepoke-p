@import "@/assets/sass/main.scss";

<template>
  <div class="status-box">
    <div class="status-top">
        <div class="pokemon-name">{{pokemon.made.nickname}}</div>
      <div class="status-top-right">
        <div class="pokemon-gender">{{pokemon.made.gender_name}}</div>
        <div class="pokemon-level">Lv. {{pokemon.made.level}}</div>
      </div>
    </div>
    <div class="hp-box">
      <div class="hp-base">
        <div class="hp-hp">HP</div>
        <div class="hp-bar-outer">
          <div class="hp-bar-inner" :class="{red: (nowHp / pokemon.made.max_hp)<0.1, yellow: (nowHp / pokemon.made.max_hp)<0.3}" 
          v-bind:style="{ width: (nowHp / pokemon.made.max_hp)*100 + '%' }"></div>
        </div>
      </div>
    </div>
    <div class="status-under">
      <div class="hp-number">{{nowHp}} / {{pokemon.made.max_hp}}</div>
      <div class="ailment"></div>
    </div>
  </div>
</template>

<script>

export default {
  name: 'StatusBox',
  props: {
    pokemon: {},
    nowHp: Number,
  },
  data() {
    return {
    }
  },
  methods: {
  }
}
</script>

<style scoped lang="scss">
  //------------ステータス共通--------------------------------------
  .status-box {
    border: solid 2px #3c3c3c;
    width: 20vw;
    padding: 1% 2% 0;
    border-radius: 0.3rem;
    box-shadow: 6px 7px 4px -1px #a8ada8;
    background-color: white;

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
            transition: width 1s ease, background-color 1s ease;
            &.red {
              background-color: #e53a3a;
            }
            &.yellow {
              background-color: #f1cf27;
            }
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
        // background-color: #af4c13;
        color: white;
        display: flex;
        align-items: center;
        // box-shadow: 3px 4px 3px -2px #686868;
      }
    }
  }
  //--------------ステータス個別--------------------------
  // .pokemon-area.my .status-box {
  //   right: -7vw;
  //   bottom: 11vw;
  // }
  // .pokemon-area.your .status-box {
  //   top: 3vw;
  //   left: -8vw;
  // }
</style>