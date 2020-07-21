<template>
  <div class="toolbar">
    <div class="title">Eukolia <span class="tag">beta</span></div>
    <button @click="add_widget('smiley')">
      <i class="fas fa-plus" />
    </button>
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
  /*border-bottom: 1px solid #a7a7a7;*/
  /*box-shadow: inset 0 -1px 0 0 #bdbdbd, inset 0 1px 0 0 #f5f5f5;*/
  align-items: center;
  background: var(--grey-panel);
  border-bottom: 2px solid var(--primary-ink);
  display: flex;
  height: 48px;
  left: 0;
  padding: 0 12px;
  position: absolute;
  top: 0;
  width: 100vw;
  z-index: 9;
}

.title {
  font-weight: bold;
  margin-right: 10px;
}

.tag {
  background: var(--orange-50);
  border-radius: 8px;
  bottom: 8px;
  color: white;
  display: inline-block;
  font-size: 10px;
  font-weight: normal;
  padding: 1px 6px;
  position: relative;
  right: 6px;
}

button {
  border-radius: 4px;
  display: flex;
  font-size: 16px;
  padding: 4px 6px;
  transition: 300ms;
  transition: box-shadow 300ms, background 300ms;
}

button:hover {
  background: white;
  box-shadow: 0 0 0 2px var(--primary-ink);
}
</style>
