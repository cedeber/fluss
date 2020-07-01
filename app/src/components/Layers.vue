<template>
  <div class="left-panel">
    <div class="search">
      <i class="fas fa-search" />
      <input type="text" @blur="onExit" @change="onExit" @focus="onEnter" v-model="searchFor" />
    </div>
    <div class="layers" @click="onDeselect" @mouseover="onHover(null)">
      <div
        class="layer"
        v-for="widget in widgets"
        :key="widget.uuid"
        :class="{
          selected: widget.selected,
          hovered: widget.hovered,
          hidden: !widget.visible,
          edit: editMode === widget.uuid,
        }"
        @click.stop="onClick(widget.uuid)"
        @mouseover.stop="onHover(widget.uuid)"
      >
        <div class="layer--name">
          <i class="fas fa-vector-square" />
          <div v-if="editMode === widget.uuid" class="edit">
            <input
              :value="widget.name"
              @blur="onExit"
              @change="(e) => onRename(e, widget)"
              @keydown="(e) => onLayeyKeyDown(e)"
              :id="`layer-${widget.uuid}`"
            />
          </div>
          <div v-else @dblclick="onEditLayerName(widget.uuid)" class="name">
            {{ widget.name || "&nbsp;" }}
          </div>
        </div>
        <div class="tool" @click.stop="onHide(widget.uuid)">
          <i class="fas fa-eye-slash" />
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { reactive, toRefs, computed, nextTick, watchEffect, ComputedRef } from "vue";
import { useStore } from "vuex";
import Fuse from "fuse.js";
import { State, Widget } from "../store";

interface InnerState {
  editMode: string | null;
  searchFor: string;
  widgets: ComputedRef<Widget[]>;
}

export default {
  setup() {
    const store = useStore<State>();
    const state = reactive<InnerState>({
      editMode: null,
      searchFor: "",
      widgets: computed(() => {
        const widgets = store.state.app?.widgets;
        const fuse = new Fuse(widgets, { keys: ["name"] });
        return widgets
          ?.map((widget) => ({
            ...widget,
            selected: widget.uuid === store.state.app.active_widget_uuid,
            hovered: widget.uuid === store.state.app.pointer_widget_uuid,
          }))
          .filter((widget) => {
            if (!fuse || state.searchFor.trim() === "") return true;
            return fuse
              .search(state.searchFor)
              .map((r) => r.item.uuid)
              .includes(widget.uuid);
          });
      }),
    });

    function onClick(uuid: string) {
      store.state.api.select_widget(uuid);
    }

    function onHover(uuid: string | null) {
      store.state.api.hover_widget(uuid);
    }

    function onEditLayerName(uuid: string) {
      onEnter();
      state.editMode = uuid;
      nextTick(() => {
        const el = document.getElementById(`layer-${uuid}`);
        if (el) {
          el.focus();
        }
      });
    }

    function onEnter() {
      store.state.api.activate_events(false);
    }

    function onExit() {
      store.state.api.activate_events(true);
      state.editMode = null;
    }

    function onDeselect() {
      store.state.api.select_widget(null);
    }

    function onHide(uuid: string) {
      store.state.api.toggle_visibility_widget(uuid);
    }

    function onRename(event, widget: Widget) {
      const name = event.target.value;
      store.state.api.update_widget({
        ...widget,
        name,
      });
      onExit();
    }

    function onLayeyKeyDown(event) {
      if (event.key === "Escape") {
        onExit();
      }
    }

    function onSearch(event) {
      const searchFor = event.target.value;
    }

    return {
      ...toRefs(state),
      onClick,
      onHover,
      onEditLayerName,
      onDeselect,
      onHide,
      onRename,
      onExit,
      onLayeyKeyDown,
      onEnter,
      onSearch,
    };
  },
};
</script>

<style scoped>
.left-panel {
  position: absolute;
  top: 48px;
  left: 0;
  width: 240px;
  height: calc(100vh - 48px);
  background: var(--panel-background);
  z-index: 9;
  border-right: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
}

.layers {
  overflow-y: auto;
  font-size: 12px;
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
  flex: 1;
  overflow: hidden;
}

.layer--name .fas {
  margin-right: 4px;
}

.name {
  min-width: 50px;
  white-space: nowrap;
  text-overflow: ellipsis;
  overflow: hidden;
  flex: 1;
}

.tool {
  display: none;
  opacity: 0.7;
  font-size: 12px;
}

.layer.hidden:not(.edit) .tool,
.layer:hover:not(.edit) .tool {
  display: flex;
}

.edit,
input {
  width: 100%;
}

input {
  font-size: 12px;
  background: white;
  border-radius: 2px;
  padding: 1px 4px;
}

.search {
  background: white;
  border-bottom: 1px solid var(--border-color);
  display: flex;
  padding: 6px 3px;
  align-items: center;
}

.search .fa-search {
  margin: 0 4px;
  font-size: 12px;
}
</style>
