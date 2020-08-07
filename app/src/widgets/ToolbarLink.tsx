import React, { AnchorHTMLAttributes } from "react";
import { useFocusRing } from "@react-aria/focus";
import { useHover } from "@react-aria/interactions";
import cx from "clsx";
import style from "./ToolbarButton.module.scss";

export default function ToolbarButton(props: AnchorHTMLAttributes<Element>): React.ReactElement {
  const { isFocusVisible, focusProps } = useFocusRing();
  const { hoverProps, isHovered } = useHover({});

  return (
    <a
      {...props}
      {...focusProps}
      {...hoverProps}
      className={cx(style.button, { [style.focus]: isFocusVisible, [style.hover]: isHovered })}
      role="button"
    >
      {props.children}
    </a>
  );
}
