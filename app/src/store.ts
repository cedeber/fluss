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

export interface State {
  api: any;
  app: {
    cursor: any; // TODO
    canvas_geometry: any; // TODO
    pointer_widget_uuid: string | null;
    active_widget_uuid: string | null;
    widgets: Widget[];
  } | null;
}

// --- State ---
const state: State = {
  api: {}, // from wasm
  app: null,
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
};

// --- Actions: UPDATE ---
//export const UPDATE_WIDGET = "UPDATE_WIDGET";

const actions = {
  //[UPDATE_WIDGET]({ commit }, payload) {
  //  commit(MUTATION_UPDATE_WIDGET, payload);
  //},
};

export const store = createStore({ state, getters, mutations, actions });
