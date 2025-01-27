PKG_DIR = pkg

.PHONY: all clean build

build:
	wasm-pack build --target bundler
	cp ./argonia.js $(PKG_DIR)/argonia.js

publish:
	cargo test
	cd $(PKG_DIR) && npm publish --access=public

clean:
	rm -rf $(PKG_DIR)
