import { createStore } from "vuex";

export interface RectGeometry {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface Widget {
  name: string;
  uuid: string;
  geometry: RectGeometry;
  initial_geometry: RectGeometry;
  visible: boolean;
}

export interface Settings {
  keep_ratio: boolean;
}

export interface State {
  api: any;
  app: {
    // ui_state: UiGlobalState
    cursor: any; // TODO
    canvas_geometry: any; // TODO
    active_widget_uuid: string | null;
    pointer_widget_uuid: string | null;
    settings: Settings;
    // widgets: Vec<Smiley>
    widgets: Widget[];
  } | null;
  localSettings: {
    hideUserInterface: boolean;
  };
}

// --- State ---
const state: State = {
  api: {}, // from wasm
  app: null,
  localSettings: {
    hideUserInterface: false,
  },
};

// --- Getters ---
const getters = {
  widgetPanel: (state: State): Widget | undefined => {
    return state.app?.widgets.filter((widget) => widget.uuid === state.app?.active_widget_uuid)[0];
  },
};

// --- Mutations: MUTATE ---
export const MUTATE_API = "MUTATE_API";
export const MUTATE_APP_STATE = "MUTATE_APP_STATE";
const MUTATE_UI = "MUTATE_UI";

const mutations = {
  [MUTATE_API](state: State, payload: State["api"]) {
    state.api = payload;
  },
  // TODO replace any for payload
  [MUTATE_APP_STATE](state: State, payload: any) {
    let app_state = payload.app_state;
    app_state["widgets"] = payload.widgets;
    state.app = app_state;
  },
  [MUTATE_UI](state: State, payload: boolean) {
    state.localSettings.hideUserInterface = payload;
  },
};

// --- Actions: UPDATE/SHOW/HIDE/TOGGLE ---
export const TOGGLE_UI = "TOGGLE_UI";
export const HIDE_UI = "HIDE_UI";

const actions = {
  [TOGGLE_UI]({ commit, state }) {
    commit(MUTATE_UI, !state.localSettings.hideUserInterface);
  },
};

export const store = createStore({ state, getters, mutations, actions });
