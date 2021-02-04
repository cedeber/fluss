import React, {useEffect, useRef, useState} from "../../_snowpack/pkg/react.js";
import cx from "../../_snowpack/pkg/clsx.js";
import {useApi} from "../context/wasm-api.js";
import style from "./DetailsInput.module.scss.proxy.js";
import {useTextField} from "../../_snowpack/pkg/@react-aria/textfield.js";
export default function DetailsInput(props) {
  const [{activate_events}] = useApi();
  const [isFocused, setFocused] = useState(false);
  const [value, setValue] = useState(String(props.value ?? ""));
  useEffect(() => {
    return function cleanup() {
      if (typeof activate_events === "function")
        activate_events(true);
    };
  }, [activate_events]);
  useEffect(() => {
    setValue(String(props.value));
  }, [props.value]);
  function onValid() {
    const num = Number(value);
    if (isNaN(num) || value === "" || props.min && num < props.min || props.max && num > props.max) {
      setValue(String(props.value ?? ""));
    } else {
      if (typeof props.onChange === "function")
        props.onChange(num);
    }
  }
  return /* @__PURE__ */ React.createElement("div", {
    className: cx(style.container, {
      [style.focus]: isFocused,
      [style.disabled]: props.isDisabled,
      [style.locked]: props.isLockedTo,
      [style.right]: props.isLockedTo === "right",
      [style.left]: props.isLockedTo === "left"
    })
  }, /* @__PURE__ */ React.createElement(Input, {
    value,
    label: props.label,
    isDisabled: props.isDisabled,
    onChange: (value2) => setValue(value2.trim()),
    onKeyDown: (e) => {
      if (e.key === "Enter") {
        onValid();
      }
    },
    onBlur: onValid,
    onFocusChange: (isFocused2) => {
      setFocused(isFocused2);
      activate_events(!isFocused2);
    }
  }));
}
function Input(props) {
  const input = useRef(null);
  const {labelProps, inputProps} = useTextField(props, input);
  return /* @__PURE__ */ React.createElement(React.Fragment, null, /* @__PURE__ */ React.createElement("label", {
    className: style.label,
    ...labelProps
  }, props.label), /* @__PURE__ */ React.createElement("input", {
    ref: input,
    className: style.input,
    ...inputProps
  }));
}
