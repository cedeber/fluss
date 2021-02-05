import React from "react";
import storage from "../utils/proxy-storage";
import type { AppSettings } from "../types";

type Actions = "toggleUi";
type Action = { type: Actions };
type Dispatch = (action: Action) => void;
type State = AppSettings;
type ProviderProps = React.PropsWithChildren<{}>;

const StateContext = React.createContext<State | undefined>(undefined);
const DispatchContext = React.createContext<Dispatch | undefined>(undefined);

function reducer(state: State, action: Action) {
    switch (action.type) {
        case "toggleUi": {
            const newSettings = { hideUserInterface: !state.hideUserInterface };
            storage.settings = newSettings;
            return newSettings;
        }
        default: {
            throw new Error(`Unhandled action type: ${action.type}`);
        }
    }
}

function SettingsProvider({ children }: ProviderProps): JSX.Element {
    const [state, dispatch] = React.useReducer(
        reducer,
        storage.settings ?? { hideUserInterface: false },
    );

    return (
        <StateContext.Provider value={state}>
            <DispatchContext.Provider value={dispatch}>{children}</DispatchContext.Provider>
        </StateContext.Provider>
    );
}

function useState(): State {
    const context = React.useContext(StateContext);
    if (context === undefined) {
        throw new Error("useState must be used within a Provider");
    }
    return context;
}

function useDispatch(): Dispatch {
    const context = React.useContext(DispatchContext);
    if (context === undefined) {
        throw new Error("useDispatch must be used within a Provider");
    }
    return context;
}

function useSettings(): [State, Dispatch] {
    return [useState(), useDispatch()];
}

export { SettingsProvider, useSettings };
