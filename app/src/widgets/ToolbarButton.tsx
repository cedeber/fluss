import React, { useRef } from "react";
import { useButton } from "@react-aria/button";
import { useFocusRing } from "@react-aria/focus";
import { useHover } from "@react-aria/interactions";
import { AriaButtonProps } from "@react-types/button";
import cx from "clsx";
import style from "./ToolbarButton.module.scss";

export default function ToolbarButton(props: AriaButtonProps): React.ReactElement {
  const ref = useRef<HTMLButtonElement>(null);
  const { buttonProps } = useButton(props, ref);
  const { isFocusVisible, focusProps } = useFocusRing();
  const { hoverProps, isHovered } = useHover({});

  return (
    <button
      {...buttonProps}
      {...focusProps}
      {...hoverProps}
      ref={ref}
      className={cx(style.button, { [style.focus]: isFocusVisible, [style.hover]: isHovered })}
    >
      {props.children}
    </button>
  );
}
