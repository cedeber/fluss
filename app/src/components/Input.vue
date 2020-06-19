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
import { reactive, watchEffect, toRefs, isReactive, ref } from "vue";
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
}

.input {
  font-size: 11px;
  width: 40px;
  text-align: right;
  padding: 4px;
  line-height: 1;
  border: 1px solid #cececf;
  border-left: 0;
  background: white;
  -moz-appearance: textfield;
  margin: 0;
  outline: 0;
  display: flex;
  align-items: center;
}

.input::-webkit-inner-spin-button {
  display: none;
}

.label {
  font-size: 11px;
  text-align: right;
  padding: 4px 0 4px 4px;
  line-height: 1;
  border-radius: 2px 0 0 2px;
  border: 1px solid #cececf;
  border-right: 0;
  background: white;
  color: #42434495;
  width: 20px;
}

.unit {
  font-size: 11px;
  text-align: left;
  padding: 4px;
  line-height: 1;
  color: #42434495;
  border: 1px solid #cececf;
  border-left: 0;
  border-radius: 0 2px 2px 0;
  width: 22px;
  display: flex;
  align-items: center;
}
</style>
