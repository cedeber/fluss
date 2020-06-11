import { createStore } from "vuex";

interface State {
    count: number;
}

const state: State = {
    count: 1,
};

const mutations = {
    increment(state: State) {
        state.count += 1;
    },
    decrement(state: State) {
        state.count -= 1;
    },
}

export const store = createStore({ state, mutations });