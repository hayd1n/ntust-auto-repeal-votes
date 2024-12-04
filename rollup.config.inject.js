import { nodeResolve } from "@rollup/plugin-node-resolve";
import typescript from "@rollup/plugin-typescript";

/** @type {import('rollup').RollupOptions} */
export default {
  input: "src-inject/index.ts",
  output: {
    file: "inject/index.js",
    format: "esm",
  },
  plugins: [
    nodeResolve(),
    typescript({
      tsconfig: "./tsconfig.inject.json",
    }),
  ],
};
