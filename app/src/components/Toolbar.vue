<template>
  <div class="toolbar">
    <button @click="add_widget('smiley')">add widget</button>
  </div>
</template>

<script lang="ts">
import { useStore } from "vuex";
import { State } from "../store";

export default {
  setup() {
    const store = useStore<State>();

    function add_widget(widget: string) {
      store.state.api.add_widget(widget);
    }

    return { add_widget };
  },
};
</script>

<style scoped>
.toolbar {
  position: absolute;
  top: 0;
  height: 48px;
  left: 0;
  width: 100vw;
  background: black;
  color: white;
  z-index: 9;
}
</style>
