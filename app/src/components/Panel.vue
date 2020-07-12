<template>
  <div class="panel">
    <div v-if="widget" class="part">
      <div class="title">Geometry</div>
      <div class="group">
        <Input v-model:value="x" label="X" unit="px" />
        <Input v-model:value="y" label="Y" unit="px" />
      </div>
      <div class="group">
        <Input v-model:value="width" label="W" unit="px" :min="1" />
        <Input v-model:value="height" label="H" unit="px" :min="1" />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { computed, reactive, toRefs } from "vue";
import { useStore } from "vuex";
import Input from "./Input.vue";
import { State } from "../store";

export default {
  components: {
    Input,
  },
  setup() {
    const store = useStore<State>();
    const state = reactive({
      widget: computed(() => store.getters.widgetPanel),
      x: computedValue("x"),
      y: computedValue("y"),
      width: computedValue("width"),
      height: computedValue("height"),
    });

    function computedValue(prop: string) {
      return computed<number>({
        get: () => state.widget.geometry[prop] ?? 0,
        set: (value) => {
          store.state.api.update_widget({
            ...state.widget,
            geometry: {
              ...state.widget.geometry,
              [prop]: value,
            },
          });
        },
      });
    }

    return { ...toRefs(state) };
  },
};
</script>

<style scoped>
.panel {
  background: var(--panel-background);
  border-left: 1px solid var(--border-color);
  font-size: 12px;
  height: calc(100vh - 48px);
  overflow-y: scroll;
  position: absolute;
  right: 0;
  top: 48px;
  user-select: none;
  width: 240px;
  z-index: 9;
}

.part {
  border-bottom: 1px solid var(--border-color);
  padding: 8px 12px;
  width: 100%;
}

.title {
  font-size: 11px;
  margin-bottom: 4px;
  opacity: 0.45;
  text-transform: uppercase;
}

.group {
  /*gap: 8px;*/
  display: flex;
  margin: 8px 0;
}

.group > *:not(:last-child) {
  margin-right: 8px;
}
</style>
