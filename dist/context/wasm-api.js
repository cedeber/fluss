import React from "../../_snowpack/pkg/react.js";
const StateContext = React.createContext(void 0);
const DispatchContext = React.createContext(void 0);
function reducer(state, action) {
  switch (action.type) {
    case "update": {
      return {...action.payload};
    }
    default: {
      throw new Error(`Unhandled action type: ${action.type}`);
    }
  }
}
function ApiProvider({children}) {
  const [state, dispatch] = React.useReducer(reducer, {});
  return /* @__PURE__ */ React.createElement(StateContext.Provider, {
    value: state
  }, /* @__PURE__ */ React.createElement(DispatchContext.Provider, {
    value: dispatch
  }, children));
}
function useState() {
  const context = React.useContext(StateContext);
  if (context === void 0) {
    throw new Error("useState must be used within a Provider");
  }
  return context;
}
function useDispatch() {
  const context = React.useContext(DispatchContext);
  if (context === void 0) {
    throw new Error("useDispatch must be used within a Provider");
  }
  return context;
}
function useApi() {
  return [useState(), useDispatch()];
}
export {ApiProvider, useApi};
