import React, { useRef } from "react";
import { useSwitch } from "@react-aria/switch";
import { useFocusRing } from "@react-aria/focus";
import { useHover } from "@react-aria/interactions";
import { VisuallyHidden } from "@react-aria/visually-hidden";
import { useToggleState } from "@react-stately/toggle";
import cx from "clsx";
import style from "./ToolbarButton.module.scss";
import { AriaSwitchProps } from "@react-types/switch";

export default function ToolbarButton(props: AriaSwitchProps): React.ReactElement {
  const ref = useRef<HTMLInputElement>(null);
  const state = useToggleState(props);
  const { inputProps } = useSwitch(props, state, ref);
  const { isFocusVisible, focusProps } = useFocusRing();
  const { hoverProps, isHovered } = useHover({});

  return (
    <>
      <VisuallyHidden>
        <input {...inputProps} {...focusProps} {...hoverProps} ref={ref} />
      </VisuallyHidden>
      <div
        className={cx(style.button, { [style.focus]: isFocusVisible, [style.hover]: isHovered })}
        role="button"
      >
        {props.children}
      </div>
    </>
  );
}
