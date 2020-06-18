<template>
  <div class="container">
    <div class="label">{{ label }}</div>
    <input class="input" type="number" v-model="val" pattern="[0-9]*" />
    <div class="unit">{{ unit }}</div>
  </div>
</template>

<script lang="ts">
import { ref, watchEffect, computed } from "vue";

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
    // needed because reactivity is not deep
    // const value = ref(props.value);
    // watchEffect(() => (value.value = props.value));

    const val = computed({
      get() {
        return props.value;
      },
      set(v) {
        emit("update:value", v);
      },
    });

    return { ...props, val };
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
