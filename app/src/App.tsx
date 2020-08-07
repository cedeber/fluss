import React, { useEffect } from "react";
import { SettingsProvider } from "./context/settings";
import { ApiProvider, useApi } from "./context/wasm-api";

import init, { start } from "../public/pkg/eukolia.js";

import styles from "./App.module.scss";
import Toolbar from "./panels/Toolbar";
import Layers from "./panels/Layers";
import { useApp, AppProvider } from "./context/app-state";
import { WidgetsProvider, useWidgets } from "./context/widgets";
import { UiGlobalState, Widget } from "./types";

function App(): React.ReactElement {
  return (
    <>
      <ApiProvider>
        <SettingsProvider>
          <AppProvider>
            <WidgetsProvider>
              <Main />
            </WidgetsProvider>
          </AppProvider>
        </SettingsProvider>
      </ApiProvider>
      <div id="wasm" className={styles.wasm} />
    </>
  );
}

function Main() {
  const [, dispatchApi] = useApi();
  const [, dispatchAppState] = useApp();
  const [, dispatchWidgets] = useWidgets();

  useEffect(() => {
    window.update_app_state = function (app_state: UiGlobalState, widgets: Widget[]) {
      dispatchWidgets({ type: "update", payload: widgets });
      dispatchAppState({ type: "update", payload: app_state });
    };

    // Load WASM
    (async function () {
      await init("/pkg/eukolia_bg.wasm");
      const [
        activate_events,
        add_widget,
        update_widget,
        select_widget,
        hover_widget,
        toggle_visibility_widget,
        delete_widget,
        swap_widget,
        update_settings,
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
          update_settings,
        },
      });
    })();
  }, []);

  return (
    <>
      <Toolbar />
      <Layers />
    </>
  );
}

export default App;
