import React, {useContext, useEffect, useRef, useState} from "../../_snowpack/pkg/react.js";
import style from "./Layers.module.scss.proxy.js";
import {useSettings} from "../context/settings.js";
import {useWidgets} from "../context/widgets.js";
import cx from "../../_snowpack/pkg/clsx.js";
import {useApp} from "../context/app-state.js";
import Fuse from "../../_snowpack/pkg/fusejs.js";
import {useApi} from "../context/wasm-api.js";
import {useFocus, useHover, useKeyboard, usePress} from "../../_snowpack/pkg/@react-aria/interactions.js";
import {FocusRing, useFocusRing} from "../../_snowpack/pkg/@react-aria/focus.js";
import {useButton} from "../../_snowpack/pkg/@react-aria/button.js";
const SearchContext = React.createContext(["", () => {
}]);
export default function Layers() {
  const searchState = useState("");
  return /* @__PURE__ */ React.createElement(SearchContext.Provider, {
    value: searchState
  }, /* @__PURE__ */ React.createElement(LayersPanel, null));
}
function LayersPanel() {
  const [settings] = useSettings();
  let [widgets] = useWidgets();
  const [search] = useContext(SearchContext);
  if (search) {
    const fuse = new Fuse(widgets, {keys: ["name"]});
    widgets = widgets?.filter((widget) => {
      if (!fuse || search.trim() === "")
        return true;
      return fuse.search(search).map((r) => r.item.uuid).includes(widget.uuid);
    });
  }
  if (settings.hideUserInterface || !widgets || !widgets.length && !search) {
    return null;
  }
  return /* @__PURE__ */ React.createElement("div", {
    className: style.panel
  }, /* @__PURE__ */ React.createElement(Tools, null), /* @__PURE__ */ React.createElement("div", {
    className: style.layers
  }, widgets.map((widget) => /* @__PURE__ */ React.createElement(Layer, {
    key: widget.uuid,
    widget
  })), search && widgets.length === 0 && /* @__PURE__ */ React.createElement("div", {
    className: style.notFound
  }, 'No layers with "', search, '" found.')), /* @__PURE__ */ React.createElement(Search, null));
}
export function Tools() {
  const [api] = useApi();
  const [app] = useApp();
  const [widgets] = useWidgets();
  function onLayerSwap(uuid, a, b) {
    api.swap_widget({uuid, a, b});
  }
  function onLayerMove(direction) {
    const len = widgets.length;
    let goTo = false;
    for (let i = 0; i < len; i += 1) {
      const widget = widgets[i];
      if (widget.uuid === app?.active_widget_uuid) {
        goTo = !goTo;
      }
      if (goTo) {
        if (Math.abs(direction) === 1) {
          onLayerSwap(widget.uuid, i, i + direction);
          goTo = false;
        } else if (direction > 0) {
          onLayerSwap(widget.uuid, i, i + 1);
          goTo = false;
        } else {
          for (let y = i; y > 0; y -= 1) {
            onLayerSwap(widget.uuid, y, y - 1);
          }
          goTo = false;
        }
      }
    }
  }
  return /* @__PURE__ */ React.createElement("div", {
    className: style.layerTools
  }, /* @__PURE__ */ React.createElement(Tool, {
    iconClassName: "fa-bring-front",
    onPress: () => onLayerMove(-2)
  }), /* @__PURE__ */ React.createElement(Tool, {
    iconClassName: "fa-bring-forward",
    onPress: () => onLayerMove(-1)
  }), /* @__PURE__ */ React.createElement(Tool, {
    iconClassName: "fa-send-backward",
    onPress: () => onLayerMove(1)
  }), /* @__PURE__ */ React.createElement(Tool, {
    iconClassName: "fa-send-back",
    onPress: () => onLayerMove(2)
  }));
}
function Tool(props) {
  const ref = useRef(null);
  const {buttonProps} = useButton(props, ref);
  const {focusProps, isFocusVisible} = useFocusRing();
  const {hoverProps, isHovered} = useHover({});
  return /* @__PURE__ */ React.createElement("button", {
    className: cx(style.layerTool, style.icon, {focus: isFocusVisible, hover: isHovered}),
    ref,
    ...buttonProps,
    ...focusProps,
    ...hoverProps
  }, /* @__PURE__ */ React.createElement("i", {
    className: cx("fad", "fa-md", props.iconClassName)
  }));
}
function Layer({widget}) {
  const [app] = useApp();
  const [api] = useApi();
  const ref = useRef(null);
  const inputRef = useRef(null);
  const [editMode, setEditMode] = useState(false);
  const [name, setName] = useState(widget.name);
  const {hoverProps} = useHover({
    onHoverChange: (isHovering) => {
      if (isHovering)
        api.hover_widget(widget.uuid);
      else
        api.hover_widget(null);
    }
  });
  const {focusProps} = useFocus({
    onFocus: () => api.select_widget(widget.uuid)
  });
  const {pressProps} = usePress({
    onPress: () => {
      api.toggle_visibility_widget(widget.uuid);
    }
  });
  useEffect(() => {
    if (widget.uuid === app?.active_widget_uuid) {
      ref.current?.focus();
    } else {
      setEditMode(false);
    }
  }, [app?.active_widget_uuid]);
  useEffect(() => {
    if (editMode) {
      inputRef.current?.select();
      api.activate_events(false);
    } else {
      try {
        api.activate_events(true);
      } catch {
      }
    }
  }, [editMode]);
  if (!app) {
    return null;
  }
  function updateName(event) {
    if (event.key === "Escape") {
      setName(widget.name);
      setEditMode(false);
    } else if (event.key === "Enter") {
      api.update_widget({
        ...widget,
        name
      });
      setEditMode(false);
    }
  }
  return /* @__PURE__ */ React.createElement("div", {
    className: cx(style.layer, {
      [style.selected]: widget.uuid === app.active_widget_uuid,
      [style.hovered]: widget.uuid === app.pointer_widget_uuid,
      [style.hidden]: !widget.visible,
      [style.edit]: editMode
    }),
    ...hoverProps,
    ...focusProps,
    tabIndex: 0,
    role: "button",
    ref
  }, /* @__PURE__ */ React.createElement("div", {
    className: style.layerName
  }, /* @__PURE__ */ React.createElement("i", {
    className: cx(style.layerIcon, "fad", "fa-vector-square", "fa-sm")
  }), editMode ? /* @__PURE__ */ React.createElement("div", {
    className: style.edit
  }, /* @__PURE__ */ React.createElement("input", {
    value: name,
    ref: inputRef,
    onChange: (event) => setName(event.currentTarget.value),
    onKeyDown: updateName
  })) : /* @__PURE__ */ React.createElement("div", {
    className: style.name,
    onDoubleClick: () => setEditMode(true)
  }, widget.name || " ")), /* @__PURE__ */ React.createElement("div", {
    className: style.tool,
    ...pressProps
  }, /* @__PURE__ */ React.createElement("i", {
    className: "fad fa-eye-slash fa-sm"
  })));
}
function Search() {
  const [search, setSearch] = useContext(SearchContext);
  const [api] = useApi();
  const ref = useRef(null);
  const {focusProps} = useFocus({
    onFocus: () => api.activate_events(false),
    onBlur: () => api.activate_events(true)
  });
  const {keyboardProps} = useKeyboard({
    onKeyDown: (e) => {
      if (e.key === "Escape")
        setSearch("");
    }
  });
  const {pressProps} = usePress({
    onPress: () => {
      setSearch("");
      ref.current?.focus();
    }
  });
  return /* @__PURE__ */ React.createElement("div", {
    className: cx(style.search, {[style.active]: search})
  }, /* @__PURE__ */ React.createElement("i", {
    className: cx("fad fa-search fa-md", style.icon),
    "aria-hidden": "true"
  }), /* @__PURE__ */ React.createElement("input", {
    type: "text",
    placeholder: "Search Layers",
    value: search,
    ref,
    onChange: (event) => setSearch(event.target.value),
    ...focusProps,
    ...keyboardProps
  }), search && /* @__PURE__ */ React.createElement(FocusRing, {
    focusRingClass: style.clearFocus
  }, /* @__PURE__ */ React.createElement("button", {
    className: cx(style.clearSearch, style.icon),
    ...pressProps
  }, /* @__PURE__ */ React.createElement("i", {
    className: "fad fa-times-circle fa-sm"
  }))));
}
