<template>
  <div class="layers" @click="onDeselect">
    <div
      class="layer"
      v-for="widget in widgets"
      :key="widget.uuid"
      :class="{ selected: widget.selected, hovered: widget.hovered }"
      @click.stop="onClick(widget.uuid)"
      @dblclick.stop="onDblClick(widget.uuid)"
      @mouseover.stop="onHover(widget.uuid)"
    >
      {{ widget.name }}
    </div>
  </div>
</template>

<script lang="ts">
import { reactive, toRefs, computed } from "vue";
import { useStore } from "vuex";
import { State } from "../store";

export default {
  setup() {
    const store = useStore<State>();
    const widgets = computed(() =>
      store.state.app?.widgets.map((widget) => ({
        name: widget.name,
        uuid: widget.uuid,
        selected: widget.uuid === store.state.app.active_widget_uuid,
        hovered: widget.uuid === store.state.app.pointer_widget_uuid,
      })),
    );

    function onClick(uuid: string) {
      store.state.api.select_widget(uuid);
    }

    function onHover(uuid: string) {
      console.log("hover", uuid);
      store.state.api.hover_widget(uuid);
    }

    function onDblClick(uuid: string) {
      alert("edit");
    }

    function onDeselect() {
      store.state.api.select_widget(null);
    }

    return { widgets, onClick, onHover, onDblClick, onDeselect };
  },
};
</script>

<style scoped>
.layers {
  position: absolute;
  top: 48px;
  left: 0;
  width: 240px;
  height: calc(100vh - 48px);
  background: var(--panel-background);
  z-index: 9;
  border-right: 1px solid var(--border-color);
  font-size: 12px;
  overflow-y: scroll;
  user-select: none;
  display: flex;
  flex-direction: column;
  /*gap: 1px;*/
  padding: 3px;
}

.layer:not(:last-child) {
  margin-bottom: 1px;
}

.layer {
  display: flex;
  padding: 4px 12px;
  border-radius: 4px;
  cursor: default;
}

/*.layer:hover:not(.selected),*/
.hovered:not(.selected) {
  box-shadow: inset 0 0 0 2px var(--blue);
}

.selected {
  background: var(--pink);
  color: white;
}
</style>
