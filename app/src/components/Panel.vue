<template>
  <div class="panel">
    <div v-if="widget != null" class="object">
      <div class="title">{{ widget.name }}</div>
      <div class="category">Geometry</div>
      <div class="group">
        <Input :value="widget.geometry.center[0]" label="CX" unit="px" />
        <Input :value="widget.geometry.center[1]" label="CY" unit="px" />
      </div>
      <div class="group">
        <Input :value="widget.geometry.radius" label="r" unit="px" />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { useStore } from "vuex";
import { computed } from "vue";
import Input from "./Input.vue";
import { State } from "../store";

export default {
  components: {
    Input,
  },
  setup() {
    const store = useStore<State>();
    const widget = computed(() => store.state.currentWidget);

    return { widget };
  },
};
</script>

<style scoped>
.panel {
  --border-color: #e2e2e2;
  position: absolute;
  top: 48px;
  right: 0;
  width: 300px;
  height: calc(100vh - 48px);
  background: white;
  z-index: 9;
  border-left: 1px solid var(--border-color);
  font-size: 12px;
  overflow-y: scroll;
  user-select: none;
}

.object {
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
