import React, {useEffect} from "../_snowpack/pkg/react.js";
import {SettingsProvider} from "./context/settings.js";
import {ApiProvider, useApi} from "./context/wasm-api.js";
import init, {start} from "../pkg/fluss_app.js";
import style from "./App.module.scss.proxy.js";
import Toolbar from "./panels/Toolbar.js";
import Layers from "./panels/Layers.js";
import {useApp, AppProvider} from "./context/app-state.js";
import {WidgetsProvider, useWidgets} from "./context/widgets.js";
import Details from "./panels/Details.js";
function App() {
  return /* @__PURE__ */ React.createElement(React.Fragment, null, /* @__PURE__ */ React.createElement(ApiProvider, null, /* @__PURE__ */ React.createElement(SettingsProvider, null, /* @__PURE__ */ React.createElement(AppProvider, null, /* @__PURE__ */ React.createElement(WidgetsProvider, null, /* @__PURE__ */ React.createElement(Main, null))))), /* @__PURE__ */ React.createElement("div", {
    id: "wasm",
    className: style.wasm
  }));
}
function Main() {
  const [, dispatchApi] = useApi();
  const [, dispatchAppState] = useApp();
  const [, dispatchWidgets] = useWidgets();
  useEffect(() => {
    window.update_app_state = function(app_state, widgets) {
      dispatchWidgets({type: "update", payload: widgets});
      dispatchAppState({type: "update", payload: app_state});
    };
    (async function() {
      await init("/pkg/fluss_app_bg.wasm");
      const [
        activate_events,
        add_widget,
        update_widget,
        select_widget,
        hover_widget,
        toggle_visibility_widget,
        delete_widget,
        swap_widget,
        update_settings
      ] = start();
      dispatchApi({
        type: "update",
        payload: {
          activate_events,
          add_widget,
          update_widget,
          select_widget,
          hover_widget,
          toggle_visibility_widget,
          delete_widget,
          swap_widget,
          update_settings
        }
      });
    })();
  }, []);
  return /* @__PURE__ */ React.createElement(React.Fragment, null, /* @__PURE__ */ React.createElement(Toolbar, null), /* @__PURE__ */ React.createElement(Layers, null), /* @__PURE__ */ React.createElement(Details, null));
}
export default App;
