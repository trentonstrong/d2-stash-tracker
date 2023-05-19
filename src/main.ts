import "./styles.css";
import App from "./App.svelte";
import * as d2s from "./lib/d2s/d2s";
import { constants } from "./lib/d2s/data/versions/99_constant_data"

const app = new App({
  target: document.getElementById("app"),
});

console.log("d2s", d2s);
d2s.setConstantData(99, constants)

export default app;
