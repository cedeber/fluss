import { createStore } from "vuex";

export interface State {
  currentWidget: Widget | null;
  api: any;
}

export interface Widget {
  uuid: string;
  type: string; // Geometry Type?
  geometry: any;
}

// --- State ---
const state: State = {
  currentWidget: undefined, // for panel
  api: {}, // from wasm
};

// --- Getters ---
const getters = {};

// --- Mutations: MUTATE ---
export const MUTATE_WIDGET = "MUTATE_WIDGET";
export const MUTATE_API = "MUTATE_API";

const mutations = {
  [MUTATE_WIDGET](state: State, payload: Widget) {
    state.currentWidget = payload;
  },
  [MUTATE_API](state: State, payload: any) {
    state.api = payload;
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
