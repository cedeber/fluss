import React from "react";
import { UiGlobalState } from "../types";

type Actions = "update";
type Action = { type: Actions; payload: UiGlobalState };
type Dispatch = (action: Action) => void;
type State = UiGlobalState;
type ProviderProps = React.PropsWithChildren<{}>;

const StateContext = React.createContext<UiGlobalState | undefined>(undefined);
const DispatchContext = React.createContext<Dispatch | undefined>(undefined);

function reducer(state: State, action: Action) {
  switch (action.type) {
    case "update": {
      return { ...action.payload };
    }
    default: {
      throw new Error(`Unhandled action type: ${action.type}`);
    }
  }
}

function AppProvider({ children }: ProviderProps): JSX.Element {
  const [state, dispatch] = React.useReducer(reducer, (null as unknown) as State);

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

function useApp(): [State, Dispatch] {
  return [useState(), useDispatch()];
}

export { AppProvider, useApp };
