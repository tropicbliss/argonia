wasm-pack build --scope tropicbliss --target bundler
# https://github.com/rustwasm/wasm-bindgen/pull/3510
cat > pkg/argonian.js << 'EOL'
import * as imports from "./argonian_bg.js";
import wkmod from "./argonian_bg.wasm";
const instance = new WebAssembly.Instance(wkmod, {
  "./argonian_bg.js": imports,
});
imports.__wbg_set_wasm(instance.exports);
export * from "./argonian_bg.js";
EOL
