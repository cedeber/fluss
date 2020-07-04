<template>
  <Toolbar />
  <Layers />
  <Panel />
  <div id="wasm" />
</template>

<script lang="ts">
import init, { start } from "/pkg/eukolia.js";
import { State, MUTATE_WIDGET, MUTATE_API, MUTATE_APP_STATE } from "./store";
import { useStore } from "vuex";
import Panel from "./components/Panel.vue";
import Toolbar from "./components/Toolbar.vue";
import Layers from "./components/Layers.vue";

export default {
  components: {
    Panel,
    Toolbar,
    Layers,
  },
  mounted() {
    const store = useStore<State>();

    // TODO deprecate
    (window as any).update_widget_panel = function updateWidgetPanel(payload: any) {
      // console.log("update_widget_panel", payload);
      store.commit(MUTATE_WIDGET, payload);
    };

    (window as any).update_app_state = function updateAppState(app_state, widgets) {
      // console.log("update", payload);
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
