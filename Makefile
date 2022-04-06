CARGO ?= cargo
CBINDGEN ?= cbindgen

build-pkg :
	$(CARGO) build --package runtime --release
	$(CBINDGEN) --config=cbindgen.toml --crate runtime --output pkg/runtime-cpp/includes/runtime.h
	cp target/release/libruntime.so pkg/runtime-cpp/libruntime.so
	mkdir -p pkg/modules-wasm
	$(CARGO) wasi build --package dex --release
	cp target/wasm32-wasi/release/dex.wasm pkg/modules-wasm/dex.wasm
