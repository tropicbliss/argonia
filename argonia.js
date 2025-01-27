import * as imports from "./argonia_bg.js";
import wkmod from "./argonia_bg.wasm";
const instance = new WebAssembly.Instance(wkmod, {
  "./argonia_bg.js": imports,
});
imports.__wbg_set_wasm(instance.exports);
export * from "./argonia_bg.js";
