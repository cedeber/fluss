<template>
  <div class="panel">
    <div v-if="isShow" class="part">
      <div class="title">{{ name }}</div>
      <div class="category">Geometry</div>
      <div class="group">
        <Input v-model:value="valX" label="CX" unit="px" />
        <Input v-model:value="valY" label="CY" unit="px" />
      </div>
      <div class="group">
        <Input v-model:value="valR" label="r" unit="px" />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { useStore } from "vuex";
import { computed, ref } from "vue";
import Input from "./Input.vue";
import { State } from "../store";

export default {
  components: {
    Input,
  },
  setup() {
    const store = useStore();
    const widget = computed<any>(() => store.state.currentWidget);
    const name = computed<string>(() => widget.value?.name || "");
    const isShow = computed(() => widget.value != null);

    const valX = computed<number>({
      get() {
        return widget.value?.geometry?.center[0] || 0;
      },
      set(v) {
        console.log("setX", v);
        store.state.api.update_widget(widget.value.uuid, {
          center: [v, valY.value],
          radius: valR.value,
        });
      },
    });

    const valY = computed<number>({
      get() {
        return widget.value?.geometry?.center[1] || 0;
      },
      set(v) {
        console.log("setY", v);
        store.state.api.update_widget(widget.value.uuid, {
          center: [valX.value, v],
          radius: valR.value,
        });
      },
    });

    const valR = computed<number>({
      get() {
        return widget.value?.geometry?.radius || 0;
      },
      set(v) {
        console.log("setR", v);
        store.state.api.update_widget(widget.value.uuid, {
          center: [valX.value, valY.value],
          radius: v,
        });
      },
    });

    return { isShow, name, valX, valY, valR };
  },
};
</script>

<style scoped>
.panel {
  --border-color: #e2e2e2;
  position: absolute;
  top: 48px;
  right: 0;
  width: 240px;
  height: calc(100vh - 48px);
  background: white;
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
  margin-bottom: 4px;
  font-size: 13px;
  font-weight: bold;
}

.category {
  text-transform: uppercase;
  margin: 8px 0 4px;
  font-size: 11px;
  opacity: 0.45;
}

.group {
  display: flex;
  gap: 8px;
  margin: 8px 0;
}
</style>
