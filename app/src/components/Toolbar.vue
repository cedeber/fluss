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
  background: #e5e5e5;
  border-bottom: 1px solid var(--border-color);
  border-top: 1px solid white;
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
  background: var(--light-orange);
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
  border: 1px solid transparent;
  color: #565656;
  display: flex;
  font-size: 16px;
  padding: 4px 6px;
  transition: 300ms;
}

button:hover {
  background: #f6f6f6;
  border-color: #c2c2c2;
}
</style>
