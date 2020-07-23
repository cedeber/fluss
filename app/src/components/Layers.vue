<template>
  <div class="left-panel" v-if="showUi">
    <div class="layer-tools">
      <div class="layer-tool" @click="onLayerMove(-2)">
        <i class="fad fa-bring-front fa-sm" />
      </div>
      <div class="layer-tool" @click="onLayerMove(-1)">
        <i class="fad fa-bring-forward fa-sm" />
      </div>
      <div class="layer-tool" @click="onLayerMove(1)">
        <i class="fad fa-send-backward fa-sm" />
      </div>
      <div class="layer-tool" @click="onLayerMove(2)">
        <i class="fad fa-send-back fa-sm" />
      </div>
    </div>
    <div class="layers" @click="onDeselect" @mouseover="onHover(null)">
      <div class="not-found" v-if="searchFor && widgets.length === 0">
        No layers with "{{ searchFor }}" found.
      </div>
      <div
        class="layer"
        v-for="widget in widgets"
        :key="widget.uuid"
        :class="{
          selected: widget.uuid === active_widget_uuid,
          hovered: widget.uuid === pointer_widget_uuid,
          hidden: !widget.visible,
          edit: editMode === widget.uuid,
        }"
        @click.stop="onClick(widget.uuid)"
        @mouseover.stop="onHover(widget.uuid)"
      >
        <div class="layer--name">
          <i class="layer-icon fad fa-vector-square fa-xs" />
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
          <i class="fad fa-eye-slash fa-xs" />
        </div>
      </div>
    </div>
    <div class="search" :class="{ active: searchFor }">
      <i class="fad fa-search fa-sm" />
      <input
        type="text"
        @blur="onExit"
        @change="onExit"
        @focus="onEnter"
        v-model="searchFor"
        placeholder="Search Layers"
      />
      <div class="clear-search" @click="onClearSearch" v-if="searchFor">
        <i class="fad fa-times-circle fa-sm" />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { reactive, toRefs, computed, nextTick, ComputedRef } from "vue";
import { useStore } from "vuex";
import Fuse from "fuse.js";
import { State, Widget } from "../store";

interface InnerState {
  editMode: string | null;
  searchFor: string;
  active_widget_uuid: ComputedRef<string | undefined | null>;
  pointer_widget_uuid: ComputedRef<string | undefined | null>;
  widgets: ComputedRef<Widget[]>;
  showUi: ComputedRef<boolean>;
}

export default {
  setup() {
    const store = useStore<State>();
    const state = reactive<InnerState>({
      editMode: null,
      searchFor: "",
      active_widget_uuid: computed(() => store.state.app?.active_widget_uuid),
      pointer_widget_uuid: computed(() => store.state.app?.pointer_widget_uuid),
      widgets: computed(() => {
        const widgets = store.state.app?.widgets ?? [];
        const fuse = new Fuse(widgets, { keys: ["name"] });
        return widgets?.filter((widget) => {
          if (!fuse || state.searchFor.trim() === "") return true;
          return fuse
            .search(state.searchFor)
            .map((r) => r.item.uuid)
            .includes(widget.uuid);
        });
      }),
      showUi: computed(() => !store.state.localSettings.hideUserInterface),
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
        const inputElement = <HTMLInputElement | null>document.getElementById(`layer-${uuid}`);
        if (inputElement) {
          // inputElement.focus();
          inputElement.select();
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

    function onRename(event: Event, widget: Widget) {
      const name = (<HTMLInputElement>event.target)?.value;
      store.state.api.update_widget({
        ...widget,
        name,
      });
      onExit();
    }

    function onLayeyKeyDown(event: KeyboardEvent) {
      if (event.key === "Escape") {
        onExit();
      }
    }

    function onClearSearch() {
      state.searchFor = "";
    }

    function onLayerSwap(uuid: string, a: number, b: number) {
      store.state.api.swap_widget({ uuid, a, b });
    }

    function onLayerMove(direction: number) {
      const len = state.widgets.length;
      let goTo = false;
      for (let i = 0; i < len; i += 1) {
        const widget = state.widgets[i];
        if (widget.uuid === store.state.app?.active_widget_uuid) {
          goTo = !goTo;
        }

        if (goTo) {
          if (Math.abs(direction) === 1) {
            // up, down: single swap
            onLayerSwap(widget.uuid, i, i + direction);
            // don't swap next
            goTo = direction > 0;
          } else if (direction > 0) {
            // bottom: swap all next
            onLayerSwap(widget.uuid, i, i + 1);
            // don't swap next
            goTo = false;
          } else {
            // top
            // swap all previous
            for (let y = i; y > 0; y -= 1) {
              onLayerSwap(widget.uuid, y, y - 1);
            }
            // don't swap next
            goTo = false;
          }
        }
      }
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
      onClearSearch,
      onLayerMove,
    };
  },
};
</script>

<style scoped>
.left-panel {
  position: absolute;
  top: 68px;
  left: 10px;
  width: 240px;
  max-height: calc(100vh - 78px);
  background: var(--grey-panel);
  z-index: 9;
  border: 2px solid var(--primary-ink);
  border-radius: 5px;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  border: 0;
  box-shadow: 0 3.2px 7.2px 0 rgba(0, 0, 0, 0.132), 0 0.6px 1.8px 0 rgba(0, 0, 0, 0.108);
  background: rgba(255, 255, 255, 0.8);
  -webkit-backdrop-filter: blur(40px);
  backdrop-filter: blur(40px);
}

.layers {
  overflow-y: auto;
  user-select: none;
  display: flex;
  flex-direction: column;
  /*gap: 1px;*/
  padding: 10px 3px;
  flex: 1;
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
  box-shadow: inset 0 0 0 2px var(--widget-blue);
}

.selected {
  background: var(--widget-pink);
  color: white;
}

.layer--name {
  display: flex;
  align-items: center;
  flex: 1;
  overflow: hidden;
}

.layer-icon {
  margin-right: 4px;
}

.name {
  min-width: 50px;
  white-space: nowrap;
  text-overflow: ellipsis;
  overflow: hidden;
  flex: 1;
  font-size: 12px;
}

.tool {
  display: none;
}

.selected .tool {
  color: white;
}

.layer.hidden:not(.edit) .tool,
.layer:hover:not(.edit) .tool {
  display: flex;
}

.edit,
input {
  width: 100%;
  line-height: 0;
}

input {
  font-size: 12px;
  background: white;
  border-radius: 2px;
  padding: 2px 4px;
  outline: 0;
  line-height: 1;
}

.search {
  display: flex;
  padding: 8px 4px;
  align-items: center;
  color: var(--secondary-grey);
}

.search.active {
  color: var(--teal-70);
}

.search input {
  background: transparent;
  color: var(--teal-70);
}

.search input::placeholder {
  color: var(--secondary-grey);
  opacity: 0.5;
}

.search .clear-search,
.search .fa-search {
  margin: 0 4px;
  line-height: 1;
}

.layer-tools {
  display: flex;
  padding: 8px 6px;
  justify-content: flex-end;
}

.layer-tool {
  line-height: 1;
}

.layer-tool:focus,
.layer-tool:hover {
  outline: 0;
  color: var(--blue-50);
}

.layer-tool:not(:last-child) {
  margin-right: 8px;
}

.not-found {
  text-align: center;
  padding: 10px;
  word-break: break-word;
  font-size: 12px;
  color: var(--teal-70);
}
</style>
