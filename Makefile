CARGO ?= cargo
CBINDGEN ?= cbindgen

build-pkg :
	$(CARGO) build --package runtime --release
	$(CBINDGEN)  --config=cbindgen.toml --crate runtime --output pkg/include/runtime.h
	mkdir -p pkg/lib
	cp target/release/libruntime.a pkg/lib
	mkdir -p pkg/wasm
	$(CARGO) build --package dex --release --target wasm32-wasi
	cp target/wasm32-wasi/release/dex.wasm pkg/wasm/dex.wasm

clean:
	cargo clean

clean-pkg:
	rm -rf pkg

clean-all: clean clean-pkg

