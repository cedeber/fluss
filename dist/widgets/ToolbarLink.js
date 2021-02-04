import React from "../../_snowpack/pkg/react.js";
import {useFocusRing} from "../../_snowpack/pkg/@react-aria/focus.js";
import {useHover} from "../../_snowpack/pkg/@react-aria/interactions.js";
import cx from "../../_snowpack/pkg/clsx.js";
import style from "./ToolbarButton.module.scss.proxy.js";
export default function ToolbarButton(props) {
  const {isFocusVisible, focusProps} = useFocusRing();
  const {hoverProps, isHovered} = useHover({});
  return /* @__PURE__ */ React.createElement("a", {
    ...props,
    ...focusProps,
    ...hoverProps,
    className: cx(style.button, {
      [style.buttonFocus]: isFocusVisible,
      [style.buttonHover]: isHovered
    }),
    role: "button"
  }, props.children);
}
