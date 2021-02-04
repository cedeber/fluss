import React from "../../_snowpack/pkg/react.js";
import cx from "../../_snowpack/pkg/clsx.js";
import {useSettings} from "../context/settings.js";
import {useApi} from "../context/wasm-api.js";
import style from "./Toolbar.module.scss.proxy.js";
import ToolbarButton from "../widgets/ToolbarButton.js";
import ToolbarLink from "../widgets/ToolbarLink.js";
export default function Toolbar() {
  const [settings, dispatchSettings] = useSettings();
  const [{add_widget}] = useApi();
  const showUI = !settings.hideUserInterface;
  return /* @__PURE__ */ React.createElement("div", {
    className: style.container
  }, /* @__PURE__ */ React.createElement("div", {
    className: style.group
  }, /* @__PURE__ */ React.createElement("div", {
    className: style.toolbar
  }, /* @__PURE__ */ React.createElement("div", {
    className: style.title
  }, "Fluss ", /* @__PURE__ */ React.createElement("span", {
    className: style.tag
  }, "beta")), add_widget && /* @__PURE__ */ React.createElement(ToolbarButton, {
    onPress: () => add_widget("smiley")
  }, /* @__PURE__ */ React.createElement("i", {
    className: "fad fa-smile-plus fa-lg"
  }))), /* @__PURE__ */ React.createElement("div", {
    className: style.toolbar,
    hidden: true
  }, /* @__PURE__ */ React.createElement("button", {
    className: style.button
  }, /* @__PURE__ */ React.createElement("i", {
    className: "fad fa-undo fa-lg"
  })), /* @__PURE__ */ React.createElement("button", {
    className: style.button
  }, /* @__PURE__ */ React.createElement("i", {
    className: "fad fa-redo fa-lg"
  }))), /* @__PURE__ */ React.createElement("div", {
    className: style.toolbar,
    hidden: true
  }, /* @__PURE__ */ React.createElement("div", {
    className: style.button
  }, /* @__PURE__ */ React.createElement("i", {
    className: "fad fa-spinner-third fa-spin fa-lg"
  })), showUI && /* @__PURE__ */ React.createElement("span", null, "My_Design_File.eukolia"))), /* @__PURE__ */ React.createElement("div", {
    className: style.group
  }, showUI && /* @__PURE__ */ React.createElement("div", {
    className: style.toolbar,
    hidden: true
  }, /* @__PURE__ */ React.createElement("div", {
    className: style.button
  }, /* @__PURE__ */ React.createElement("i", {
    className: "fad fa-user-secret fa-lg"
  })), /* @__PURE__ */ React.createElement("div", {
    className: style.button
  }, /* @__PURE__ */ React.createElement("i", {
    className: "fad fa-user-injured fa-lg"
  })), /* @__PURE__ */ React.createElement("div", {
    className: style.button
  }, /* @__PURE__ */ React.createElement("i", {
    className: "fad fa-user-alien fa-lg"
  })), /* @__PURE__ */ React.createElement("div", {
    className: style.button
  }, /* @__PURE__ */ React.createElement("i", {
    className: "fad fa-user-astronaut fa-lg"
  }))), showUI && /* @__PURE__ */ React.createElement("div", {
    className: style.toolbar
  }, /* @__PURE__ */ React.createElement("div", {
    className: style.button,
    hidden: true
  }, /* @__PURE__ */ React.createElement("i", {
    className: "fad fa-sliders-v fa-lg"
  })), /* @__PURE__ */ React.createElement(ToolbarLink, {
    href: "https://docs.eukolia.design",
    target: "_blank",
    rel: "noreferrer"
  }, /* @__PURE__ */ React.createElement("i", {
    className: "fad fa-book fa-lg"
  })), /* @__PURE__ */ React.createElement(ToolbarLink, {
    href: "https://twitter.com/intent/tweet\\?text=Hi%20@EukoliaDesign",
    target: "_blank",
    rel: "noreferrer"
  }, /* @__PURE__ */ React.createElement("i", {
    className: "fad fa-comment-alt-smile fa-lg"
  }))), /* @__PURE__ */ React.createElement("div", {
    className: cx(style.toolbar, {[style.invert]: !showUI})
  }, /* @__PURE__ */ React.createElement(ToolbarButton, {
    onPress: () => dispatchSettings({type: "toggleUi"})
  }, showUI ? /* @__PURE__ */ React.createElement("i", {
    className: "fad fa-window-frame-open fa-lg"
  }) : /* @__PURE__ */ React.createElement("i", {
    className: "fad fa-window-frame fa-lg"
  })))));
}
