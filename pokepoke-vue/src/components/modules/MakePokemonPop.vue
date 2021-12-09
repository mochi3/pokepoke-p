@import "@/assets/sass/main.scss";

<template>
  <div class="pop-overwrap" @click.self="close()">
    <div class="make-pokemon-box">
    </div>
  </div>
</template>

<script>
import app from '@/main.js'

export default {
  name: 'MakePokemonPop',
  props: {
    // checkedPokemons: []
  },
  data() {
    return {
    }
  },
  methods: {
    close() {
      console.log("close");
      this.$emit('closeMakePokemonPop');
    },
  }
}
</script>

<style scoped lang="scss">
  .pop-overwrap {

    .make-pokemon-box {
    }
  }
</style>