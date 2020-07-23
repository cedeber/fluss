<template>
  <div class="container">
    <div class="group">
      <div class="toolbar">
        <div class="title">Eukolia <span class="tag">beta</span></div>
        <button class="button" @click="addWidget('smiley')">
          <i class="fas fa-plus fa-md" />
        </button>
      </div>
    </div>
    <div class="group">
      <div class="toolbar" v-if="showUi">
        <a class="button" href="https://docs.eukolia.design" target="_blank">
          <i class="fas fa-book fa-lg" />
        </a>
        <a
          class="button"
          :href="'https://twitter.com/intent/tweet\\?text=Hi%20@EukoliaDesign,'"
          target="_blank"
        >
          <i class="far fa-comment-alt fa-lg" />
        </a>
      </div>
      <div class="toolbar" :class="{ hide: !showUi }">
        <button class="button" @click="toggleUi">
          <i class="fas fa-border-none fa-lg" v-if="showUi" />
          <i class="fas fa-border-all fa-lg" v-else />
        </button>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { toRefs } from "vue";
import { useStore } from "vuex";
import { State, TOGGLE_UI } from "../store";
import { reactive, computed } from "vue";

export default {
  setup() {
    const store = useStore<State>();
    const state = reactive({
      showUi: computed(() => !store.state.localSettings.hideUserInterface),
    });

    function addWidget(widget: string) {
      // TODO dispatch action?
      store.state.api.add_widget(widget);
    }

    function toggleUi() {
      store.dispatch(TOGGLE_UI);
    }

    return { ...toRefs(state), addWidget, toggleUi };
  },
};
</script>

<style scoped>
.container {
  display: flex;
  justify-content: space-between;
  position: absolute;
  top: 10px;
  left: 10px;
  width: calc(100vw - 20px);
}

.group {
  display: flex;
}

.toolbar {
  /*border-bottom: 1px solid #a7a7a7;*/
  /*box-shadow: inset 0 -1px 0 0 #bdbdbd, inset 0 1px 0 0 #f5f5f5;*/
  align-items: center;
  background: var(--grey-panel);
  border-bottom: 2px solid var(--primary-ink);
  display: flex;
  height: 48px;
  padding: 0 12px;
  /* min-width: 250px; */
  z-index: 9;
  border: 0;
  background: rgba(255, 255, 255, 0.8);
  -webkit-backdrop-filter: blur(4px);
  backdrop-filter: blur(4px);
  border-radius: 5px;
  box-shadow: 0 3.2px 7.2px 0 rgba(0, 0, 0, 0.132), 0 0.6px 1.8px 0 rgba(0, 0, 0, 0.108);
}

.toolbar:not(:last-child) {
  margin-right: 5px;
}

.toolbar.invert {
  background: var(--primary-ink);
  color: white;
}

.toolbar.hide {
  background: var(--blue-50);
}

.title {
  font-weight: bold;
  margin-right: 10px;
}

.tag {
  background: var(--orange-50);
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

.button {
  border-radius: 4px;
  display: flex;
  /* font-size: 16px; */
  padding: 4px 6px;
  transition: 300ms;
}

.button:not(:last-child) {
  margin-right: 5px;
}

.button:focus,
.button:hover {
  outline: 0;
  color: var(--blue-50);
}

.toolbar.hide .button {
  color: white !important;
}
</style>
