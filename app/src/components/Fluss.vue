<template>
  <div id="wasm" />
</template>

<script lang="ts">
import init, { Api, start } from "/pkg/fluss.js";

window.csl = function csl(name: string) {
  console.log("csl", name);
};

export default {
  name: "Fluss",
  mounted() {
    (async function () {
      await init();
      const [add_widget] = start();

      add_widget("smiley");

      let bar = new Api("test");
      console.log(bar.name);
      bar.name = "new world";
      console.log(bar.name);
      bar.test("another world");
      console.log(bar.name);
    })();
  },
};
</script>

<style>
#wasm canvas {
  display: block;
  cursor: none;
}
</style>
