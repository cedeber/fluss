<template>
  <Toolbar />
  <Layers />
  <Panel />
  <div id="wasm" />
</template>

<script lang="ts">
import { onMounted } from "vue";
import { useStore } from "vuex";
import init, { start } from "/pkg/eukolia.js";
import { State, MUTATE_API, MUTATE_APP_STATE } from "./store";
import Panel from "./components/Panel.vue";
import Toolbar from "./components/Toolbar.vue";
import Layers from "./components/Layers.vue";

export default {
  components: {
    Panel,
    Toolbar,
    Layers,
  },
  setup() {
    onMounted(() => {
      const store = useStore<State>();

      window.update_app_state = function updateAppState(app_state, widgets) {
        store.commit(MUTATE_APP_STATE, { app_state, widgets });
      };

      // Load WASM
      (async function () {
        await init("/pkg/eukolia_bg.wasm");
        const [
          activate_events,
          add_widget,
          update_widget,
          select_widget,
          hover_widget,
          toggle_visibility_widget,
          delete_widget,
          swap_widget,
          update_settings,
        ] = start();

        store.commit(MUTATE_API, {
          activate_events,
          add_widget,
          update_widget,
          select_widget,
          hover_widget,
          toggle_visibility_widget,
          delete_widget,
          swap_widget,
          update_settings,
        });
      })();
    });
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
  width: 100%;
  height: 100%;
}
</style>
