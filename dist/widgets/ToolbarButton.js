import React, {useRef} from "../../_snowpack/pkg/react.js";
import {useButton} from "../../_snowpack/pkg/@react-aria/button.js";
import {useFocusRing} from "../../_snowpack/pkg/@react-aria/focus.js";
import {useHover} from "../../_snowpack/pkg/@react-aria/interactions.js";
import cx from "../../_snowpack/pkg/clsx.js";
import style from "./ToolbarButton.module.scss.proxy.js";
export default function ToolbarButton(props) {
  const ref = useRef(null);
  const {buttonProps} = useButton(props, ref);
  const {isFocusVisible, focusProps} = useFocusRing();
  const {hoverProps, isHovered} = useHover({});
  return /* @__PURE__ */ React.createElement("button", {
    ...buttonProps,
    ...focusProps,
    ...hoverProps,
    ref,
    className: cx(style.button, {
      [style.buttonFocus]: isFocusVisible,
      [style.buttonHover]: isHovered
    })
  }, props.children);
}
