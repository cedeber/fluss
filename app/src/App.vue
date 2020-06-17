<template>
  <Toolbar />
  <Panel />
  <div id="wasm" />
</template>

<script lang="ts">
import init, { start } from "/pkg/fluss.js";
import { State, MUTATE_WIDGET } from "./store";
import { useStore } from "vuex";
import Panel from "./components/Panel.vue";
import Toolbar from "./components/Toolbar.vue";

export default {
  components: {
    Panel,
    Toolbar,
  },
  mounted() {
    const store = useStore<State>();

    function updateWidget(payload: any) {
      // console.log("update_widget", payload);
      store.commit(MUTATE_WIDGET, payload);
    }

    // Available for WASM
    window.update_widget = updateWidget;

    // Load WASM
    (async function () {
      await init();
      const [add_widget] = start();

      store.commit("api", {
        add_widget,
      });
    })();
  },
};
</script>

<style>
#wasm canvas {
  display: block;
  position: fixed;
  top: 0;
  left: 0;
  cursor: none;
}
</style>
