<template>
  <div class="panel">
    <div v-if="widget" class="part">
      <div class="title">Geometry</div>
      <div class="group">
        <Input v-model:value="valX" label="X" unit="px" />
        <Input v-model:value="valY" label="Y" unit="px" />
      </div>
      <div class="group">
        <Input v-model:value="valWidth" label="W" unit="px" />
        <Input v-model:value="valHeight" label="H" unit="px" />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { useStore } from "vuex";
import { computed, reactive, watchEffect, toRefs } from "vue";
import Input from "./Input.vue";
import { State } from "../store";
import { clone } from "ramda";

export default {
  components: {
    Input,
  },
  setup() {
    const store = useStore();
    const state = reactive({
      widget: computed(() => store.getters.widgetPanel),
    });
    const x = computedValue("x");
    const y = computedValue("y");
    const valWidth = computedValue("width");
    const valHeight = computedValue("height");

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

    return { ...toRefs(state), valX: x, valY: y, valWidth, valHeight };
  },
};
</script>

<style scoped>
.panel {
  position: absolute;
  top: 48px;
  right: 0;
  width: 240px;
  height: calc(100vh - 48px);
  background: var(--panel-background);
  z-index: 9;
  border-left: 1px solid var(--border-color);
  font-size: 12px;
  overflow-y: scroll;
  user-select: none;
}

.part {
  width: 100%;
  border-bottom: 1px solid var(--border-color);
  padding: 8px 12px;
}

.title {
  text-transform: uppercase;
  margin-bottom: 4px;
  font-size: 11px;
  opacity: 0.45;
}

.group {
  display: flex;
  /*gap: 8px;*/
  margin: 8px 0;
}

.group > *:not(:last-child) {
  margin-right: 8px;
}
</style>
