import React, { useEffect } from "react";
import { SettingsProvider } from "./context/settings";
import { ApiProvider, useApi } from "./context/wasm-api";

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
import init, { start } from "./wasm/canvas.js";
// Need to import it so that Vite treats it as an asset
import wasm from "./wasm/canvas_bg.wasm?url";

import style from "./App.module.scss";
import Toolbar from "./panels/Toolbar";
import Layers from "./panels/Layers";
import { useApp, AppProvider } from "./context/app-state";
import { WidgetsProvider, useWidgets } from "./context/widgets";
import type { UiGlobalState, Widget } from "./types";
import Details from "./panels/Details";

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
            <div id="wasm" className={style.wasm} />
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
            await init(wasm);
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
            <Details />
        </>
    );
}

export default App;
