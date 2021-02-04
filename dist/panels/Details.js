import React, {useEffect, useState} from "../../_snowpack/pkg/react.js";
import cx from "../../_snowpack/pkg/clsx.js";
import {useApp} from "../context/app-state.js";
import {useWidgets} from "../context/widgets.js";
import style from "./Details.module.scss.proxy.js";
import {useSettings} from "../context/settings.js";
import DetailsInput from "../widgets/DetailsInput.js";
import {useApi} from "../context/wasm-api.js";
export default function Details() {
  const [{update_settings, update_widget}] = useApi();
  const [app] = useApp();
  const [widgets] = useWidgets();
  const [settings] = useSettings();
  const [keepRatio, setKeepRatio] = useState();
  const [currentWidget, setCurrentWidget] = useState();
  const [x, setX] = useState();
  const [y, setY] = useState();
  const [width, setWidth] = useState();
  const [height, setHeight] = useState();
  useEffect(() => {
    if (!app)
      return;
    setCurrentWidget(widgets.find((widget) => widget.uuid === app.active_widget_uuid));
    setKeepRatio(app.settings.keep_ratio || false);
  }, [widgets, app]);
  useEffect(() => {
    if (currentWidget) {
      setX(currentWidget.geometry.x);
      setY(currentWidget.geometry.y);
      setWidth(currentWidget.geometry.width);
      setHeight(currentWidget.geometry.height);
    }
  }, [currentWidget?.geometry]);
  function toggleKeepRatio() {
    if (update_settings && app) {
      update_settings({
        ...app.settings,
        keep_ratio: !app.settings.keep_ratio
      });
    }
  }
  function onChange(prop) {
    return function(value) {
      if (currentWidget && update_widget) {
        update_widget({
          ...currentWidget,
          geometry: {
            ...currentWidget.geometry,
            [prop]: value
          }
        });
      }
    };
  }
  if (!currentWidget || settings.hideUserInterface)
    return null;
  return /* @__PURE__ */ React.createElement("div", {
    className: style.panel
  }, /* @__PURE__ */ React.createElement("div", {
    className: style.part
  }, /* @__PURE__ */ React.createElement("div", {
    className: style.title
  }, "Geometry"), /* @__PURE__ */ React.createElement("div", {
    className: style.group
  }, /* @__PURE__ */ React.createElement(DetailsInput, {
    label: "X",
    unit: "px",
    value: x,
    onChange: onChange("x")
  }), /* @__PURE__ */ React.createElement(DetailsInput, {
    label: "Y",
    unit: "px",
    value: y,
    onChange: onChange("xy")
  }), /* @__PURE__ */ React.createElement(DetailsInput, {
    label: "\xB0",
    unit: "px",
    value: 0,
    onChange: () => {
    },
    isDisabled: true
  })), /* @__PURE__ */ React.createElement("div", {
    className: style.group
  }, /* @__PURE__ */ React.createElement(DetailsInput, {
    label: "W",
    unit: "px",
    value: width,
    min: 1,
    isLockedTo: keepRatio ? "right" : void 0,
    onChange: onChange("width")
  }), /* @__PURE__ */ React.createElement("div", {
    className: style.icon,
    onClick: () => toggleKeepRatio()
  }, keepRatio ? /* @__PURE__ */ React.createElement("i", {
    className: cx("fad", "fa-lock-alt", style.iconActive)
  }) : /* @__PURE__ */ React.createElement("i", {
    className: "fad fa-unlock-alt"
  })), /* @__PURE__ */ React.createElement(DetailsInput, {
    label: "H",
    unit: "px",
    value: height,
    min: 1,
    isLockedTo: keepRatio ? "left" : void 0,
    onChange: onChange("height")
  }))));
}
