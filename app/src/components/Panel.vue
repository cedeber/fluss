<template>
  <div class="panel" v-if="showUi">
    <div v-if="widget" class="part">
      <div class="title">Geometry</div>
      <div class="group">
        <Input v-model:value="x" label="X" unit="px" />
        <Input v-model:value="y" label="Y" unit="px" />
        <Input :value="0" label="°" unit="px" :disabled="true" />
      </div>
      <div class="group">
        <Input v-model:value="width" label="W" unit="px" :min="1" :locked="keepRatio && 'right'" />
        <div class="icon" @click="keepRatio = !keepRatio">
          <i class="fad fa-lock-alt" :class="{ 'icon-active': keepRatio }" v-if="keepRatio" />
          <i class="fad fa-unlock-alt" v-else />
        </div>
        <Input v-model:value="height" label="H" unit="px" :min="1" :locked="keepRatio && 'left'" />
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
      ratio: computed(() => state.x / state.y),
      keepRatio: computedSettings("keep_ratio"),
      showUi: computed(() => !store.state.localSettings.hideUserInterface),
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

    function computedSettings(prop: string) {
      return computed<any>({
        get: () => store.state.app.settings[prop] ?? null,
        set: (value: any) => {
          store.state.api.update_settings({
            ...store.state.app.settings,
            [prop]: value,
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
  background: var(--grey-panel);
  border-left: 2px solid var(--primary-ink);
  font-size: 12px;
  max-height: calc(100vh - 20px);
  overflow-y: scroll;
  position: absolute;
  right: 10px;
  top: 68px;
  user-select: none;
  width: 250px;
  z-index: 9;
  border: 0;
  border-radius: 5px;
  box-shadow: 0 3.2px 7.2px 0 rgba(0, 0, 0, 0.132), 0 0.6px 1.8px 0 rgba(0, 0, 0, 0.108);
  background: white;
  -webkit-backdrop-filter: blur(40px);
  backdrop-filter: blur(40px);
}

.part {
  padding: 8px 12px;
  width: 100%;
}

.title {
  font-size: 12px;
  margin-bottom: 4px;
  text-transform: uppercase;
}

.group {
  /*gap: 8px;*/
  display: flex;
  margin: 8px 0;
  align-items: center;
}

.group > *:not(:last-child):not(.icon) {
  margin-right: 22px;
}

.icon {
  position: relative;
  width: 0;
  color: var(--secondary-grey);
  right: 16.5px;
  font-size: 12px;
}

.icon-active {
  color: var(--magenta-70);
}

button {
  background: var(--primary-ink);
  color: white;
  padding: 5px 10px;
  border-radius: 3px;
  transition: background 300ms;
  text-transform: uppercase;
}

button:focus,
button:hover {
  outline: 0;
  background: var(--blue-50);
}
</style>
