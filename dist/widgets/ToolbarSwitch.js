import React, {useRef} from "../../_snowpack/pkg/react.js";
import {useSwitch} from "../../_snowpack/pkg/@react-aria/switch.js";
import {useFocusRing} from "../../_snowpack/pkg/@react-aria/focus.js";
import {useHover} from "../../_snowpack/pkg/@react-aria/interactions.js";
import {VisuallyHidden} from "../../_snowpack/pkg/@react-aria/visually-hidden.js";
import {useToggleState} from "../../_snowpack/pkg/@react-stately/toggle.js";
import cx from "../../_snowpack/pkg/clsx.js";
import style from "./ToolbarButton.module.scss.proxy.js";
export default function ToolbarButton(props) {
  const ref = useRef(null);
  const state = useToggleState(props);
  const {inputProps} = useSwitch(props, state, ref);
  const {isFocusVisible, focusProps} = useFocusRing();
  const {hoverProps, isHovered} = useHover({});
  return /* @__PURE__ */ React.createElement(React.Fragment, null, /* @__PURE__ */ React.createElement(VisuallyHidden, null, /* @__PURE__ */ React.createElement("input", {
    ...inputProps,
    ...focusProps,
    ...hoverProps,
    ref
  })), /* @__PURE__ */ React.createElement("div", {
    className: cx(style.button, {
      [style.buttonFocus]: isFocusVisible,
      [style.buttonHover]: isHovered
    }),
    role: "button"
  }, props.children));
}
