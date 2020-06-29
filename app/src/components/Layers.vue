<template>
  <div class="layers" @click="onDeselect" @mouseover="onHover(null)">
    <div
      class="layer"
      v-for="widget in widgets"
      :key="widget.uuid"
      :class="{ selected: widget.selected, hovered: widget.hovered, hidden: !widget.visible }"
      @click.stop.="onClick(widget.uuid)"
      @mouseover.stop="onHover(widget.uuid)"
    >
      <div class="layer--name">
        <i class="fas fa-vector-square" />
        <div @dblclick.stop="onDblClick(widget.uuid)" class="name">
          {{ widget.name }}
        </div>
      </div>
      <div class="tool" @click.stop="onHide(widget.uuid)">
        <i class="fas fa-eye-slash" />
      </div>
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
        visible: widget.visible,
        selected: widget.uuid === store.state.app.active_widget_uuid,
        hovered: widget.uuid === store.state.app.pointer_widget_uuid,
      })),
    );

    function onClick(uuid: string) {
      store.state.api.select_widget(uuid);
    }

    function onHover(uuid: string | null) {
      // console.log("hover", uuid);
      store.state.api.hover_widget(uuid);
    }

    function onDblClick(uuid: string) {
      alert("edit");
    }

    function onDeselect() {
      store.state.api.select_widget(null);
    }

    function onHide(uuid: string) {
      store.state.api.toggle_visibility_widget(uuid);
    }

    return { widgets, onClick, onHover, onDblClick, onDeselect, onHide };
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
  padding: 4px 8px;
  border-radius: 4px;
  cursor: default;
  justify-content: space-between;
  align-items: center;
}

/*.layer:hover:not(.selected),*/
.hovered:not(.selected) {
  box-shadow: inset 0 0 0 2px var(--blue);
}

.selected {
  background: var(--pink);
  color: white;
}

.layer--name {
  display: flex;
  align-items: center;
}

.layer--name .fas {
  margin-right: 4px;
}

.name {
  min-width: 20px;
}

.tool {
  display: none;
  opacity: 0.7;
}

.layer.hidden .tool,
.layer:hover .tool {
  display: flex;
}
</style>
