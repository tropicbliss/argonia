PKG_DIR = pkg

.PHONY: all clean build modify-js

all: build modify-js

build:
	wasm-pack build --target bundler

modify-js:
	cat > $(PKG_DIR)/argonian.js << 'EOL'
	import * as imports from "./argonian_bg.js";
	import wkmod from "./argonian_bg.wasm";
	const instance = new WebAssembly.Instance(wkmod, {
	  "./argonian_bg.js": imports,
	});
	imports.__wbg_set_wasm(instance.exports);
	export * from "./argonian_bg.js";
	EOL

deploy:
	cargo test
	cd $(PKG_DIR) && npm publish --access=public

clean:
	rm -rf $(PKG_DIR)