import * as reactPlugin from "vite-plugin-react";
import type { UserConfig } from "vite";

const config: UserConfig = {
  jsx: "react",
  plugins: [reactPlugin],
  outDir: "../public/",
  base: "./",
  // esbuildTarget: "es2020",
};

export default config;
