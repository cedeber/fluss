<template>
  <div class="container" :class="{ focus: isFocus, disabled }">
    <div class="label">{{ label }}</div>
    <input
      ref="input"
      class="input"
      type="text"
      v-model="value"
      @blur="onValid"
      @change="onValid"
      @focus="onEnter"
      :disabled="disabled"
    />
    <!--div class="unit">{{ unit }}</div-->
  </div>
</template>

<script lang="ts">
import { reactive, watchEffect, toRefs, ref } from "vue";
import { useStore } from "vuex";
import { State } from "../store";

interface InputProps {
  label: string;
  unit: string;
  value: number;
  min?: number;
  max?: number;
  disabled: boolean;
}

export default {
  props: {
    label: String,
    unit: String,
    value: Number,
    onChange: Function,
    min: { type: Number, required: false },
    max: { type: Number, required: false },
    disabled: Boolean,
  },
  setup(props: InputProps, { emit }) {
    const input = ref<HTMLInputElement>();
    const store = useStore<State>();
    const state = reactive({
      value: props.value,
      isFocus: false,
    });

    watchEffect(() => {
      state.value = props.value;
    });

    function onEnter() {
      store.state.api.activate_events(false);
      state.isFocus = true;
    }

    function onValid(event: Event) {
      // TODO Support expression
      const value = (<HTMLInputElement>event.target).value;
      const num = Number(value);

      if (
        isNaN(num) ||
        value === "" ||
        (props.min && num < props.min) ||
        (props.max && num > props.max)
      ) {
        // reset
        state.value = props.value;
      } else {
        emit("update:value", num);
      }

      input.value?.blur();
      store.state.api.activate_events(true);
      state.isFocus = false;
    }

    return { ...props, ...toRefs(state), onValid, onEnter, input };
  },
};
</script>

<style scoped>
.container {
  align-items: baseline;
  border-radius: 4px;
  border: 2px solid var(--primary-ink);
  display: flex;
}

.container.focus {
  border-color: var(--blue-50);
}

.container.disabled {
  opacity: 0.5;
}

.input {
  -moz-appearance: textfield;
  font-size: 11px;
  margin: 0;
  outline: 0;
  padding: 4px 4px 4px 0;
  text-align: right;
  width: 40px;
}

.label {
  color: var(--primary-ink);
  font-size: 11px;
  line-height: 1;
  text-align: center;
  width: 16px;
}

.container.focus .label {
  color: var(--blue-70);
}

.unit {
  color: var(--primary-ink);
  font-size: 11px;
  line-height: 1;
  padding-left: 4px;
  text-align: left;
  width: 22px;
}
</style>
