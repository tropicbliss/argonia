PKG_DIR = pkg

.PHONY: all clean build modify-js

all: build modify-js

build:
	wasm-pack build --scope tropicbliss --target bundler

modify-js:
	cat > $(PKG_DIR)/argonia.js << 'EOL'
	import * as imports from "./argonia_bg.js";
	import wkmod from "./argonia_bg.wasm";
	const instance = new WebAssembly.Instance(wkmod, {
	  "./argonia_bg.js": imports,
	});
	imports.__wbg_set_wasm(instance.exports);
	export * from "./argonia_bg.js";
	EOL

publish:
	cargo test
	cd $(PKG_DIR) && npm publish --access=public

clean:
	rm -rf $(PKG_DIR)