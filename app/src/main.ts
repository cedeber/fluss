import { createApp } from "vue";
import { store } from "./store";
import App from "./App.vue";

import "@fortawesome/fontawesome-free/css/all.css";
import "./index.css";

const app = createApp(App);

app.use(store);
app.mount("#app");
