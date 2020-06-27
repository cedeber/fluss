<template>
  <div class="container">
    <div class="label">{{ label }}</div>
    <input
      ref="input"
      class="input"
      type="text"
      v-model="value"
      v-on:blur="onValid"
      v-on:change="onValid"
      v-on:focus="onEnter"
    />
    <div class="unit">{{ unit }}</div>
  </div>
</template>

<script lang="ts">
import { reactive, watchEffect, toRefs, ref, computed } from "vue";
import { useStore } from "vuex";
import { State } from "../store";

interface InputProps {
  label: string;
  unit: string;
  value: number;
}

export default {
  name: "Input",
  props: {
    label: String,
    unit: String,
    value: Number,
    onChange: Function,
  },
  setup(props: InputProps, { emit }) {
    const input = ref(null);
    const store = useStore<State>();
    const state = reactive({
      value: props.value,
    });

    watchEffect(() => {
      state.value = props.value;
    });

    function onEnter() {
      store.state.api.activate_events(false);
    }

    function onValid(e) {
      // TODO Support expression
      // TODO, reactivate wasm JS events (subscribe_events/unsubscribe_events)
      const val = e.target.value;
      const num = Number(val);

      if (isNaN(num) || val === "") {
        // reset
        state.value = props.value;
      } else {
        emit("update:value", num);
      }

      input.value.blur();
      store.state.api.activate_events(true);
    }

    return { ...props, ...toRefs(state), onValid, onEnter, input };
  },
};
</script>

<style scoped>
.container {
  display: flex;
  align-items: baseline;
  border: 1px solid var(--border-color);
  background: white;
  border-radius: 4px;
}

.input {
  font-size: 11px;
  width: 40px;
  text-align: right;
  padding: 4px;
  -moz-appearance: textfield;
  margin: 0;
  outline: 0;
  border-right: 1px solid var(--border-color);
}

.label {
  font-size: 11px;
  line-height: 1;
  text-align: right;
  padding-right: 4px;
  color: #42434495;
  width: 20px;
}

.unit {
  font-size: 11px;
  text-align: left;
  padding-left: 4px;
  line-height: 1;
  color: #42434495;
  width: 22px;
}
</style>
