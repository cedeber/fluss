import React, { useEffect, useRef, useState } from "react";
import cx from "clsx";
import { useApi } from "../context/wasm-api";
import style from "./DetailsInput.module.scss";
import { useTextField } from "@react-aria/textfield";
import { AriaTextFieldProps } from "@react-types/textfield";

interface DetailsInputProps {
  label: string;
  unit: string;
  value?: number;
  min?: number;
  max?: number;
  isDisabled?: boolean;
  isLockedTo?: "right" | "left";
  onChange: (value: number) => void;
}

// TODO only manage logic here, like min/max,etc
export default function DetailsInput(props: DetailsInputProps): JSX.Element {
  const [api] = useApi();
  const [isFocused, setFocused] = useState(false);
  const [value, setValue] = useState<string>(String(props.value ?? ""));

  useEffect(() => {
    setValue(String(props.value));
  }, [props.value]);

  function onValid() {
    const num = Number(value);

    if (
      isNaN(num) ||
      value === "" ||
      (props.min && num < props.min) ||
      (props.max && num > props.max)
    ) {
      // reset
      setValue(String(props.value ?? ""));
    } else {
      if (typeof props.onChange === "function") props.onChange(num);
    }
  }

  return (
    <div
      className={cx(style.container, {
        [style.focus]: isFocused,
        [style.disabled]: props.isDisabled,
        [style.locked]: props.isLockedTo,
        [style.right]: props.isLockedTo === "right",
        [style.left]: props.isLockedTo === "left",
      })}
    >
      <Input
        value={value}
        label={props.label}
        isDisabled={props.isDisabled}
        onChange={(value) => setValue(value.trim())}
        onKeyDown={(e) => {
          if (e.key === "Enter") {
            onValid();
          }
        }}
        onBlur={onValid}
        onFocusChange={(isFocused) => {
          setFocused(isFocused);
          api.activate_events(!isFocused);
        }}
      />
    </div>
  );
}

// TODO input as input component
function Input(props: AriaTextFieldProps): JSX.Element {
  const input = useRef<HTMLInputElement>(null);
  const { labelProps, inputProps } = useTextField(props, input);

  return (
    <>
      <label className={style.label} {...labelProps}>
        {props.label}
      </label>
      <input ref={input} className={style.input} {...inputProps} />
      {/*<div className={style.unit}>{ unit }</div>*/}
    </>
  );
}
