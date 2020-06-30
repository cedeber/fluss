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
  position: absolute;
  top: 0;
  height: 48px;
  left: 0;
  width: 100vw;
  background: #e5e5e5;
  z-index: 9;
  /*border-bottom: 1px solid #a7a7a7;*/
  border-top: 1px solid white;
  border-bottom: 1px solid var(--border-color);
  /*box-shadow: inset 0 -1px 0 0 #bdbdbd, inset 0 1px 0 0 #f5f5f5;*/
  display: flex;
  align-items: center;
  padding: 0 12px;
}

.title {
  font-weight: bold;
  margin-right: 10px;
}

.tag {
  display: inline-block;
  font-weight: normal;
  font-size: 10px;
  position: relative;
  bottom: 8px;
  right: 6px;
  background: var(--light-orange);
  padding: 1px 6px;
  border-radius: 8px;
  color: white;
}

button {
  color: #565656;
  padding: 4px 6px;
  display: flex;
  border-radius: 4px;
  transition: 300ms;
  border: 1px solid transparent;
  font-size: 16px;
}

button:hover {
  background: #f6f6f6;
  border-color: #c2c2c2;
}
</style>
