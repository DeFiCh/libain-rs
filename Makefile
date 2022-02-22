CARGO ?= cargo
CBINDGEN ?= cbindgen

build-pkg :
	$(CARGO) build --package runtime --release
	$(CBINDGEN)  --config=cbindgen.toml --crate runtime --output pkg/runtime-cpp/include/runtime.h
	cp target/release/libruntime.so pkg/runtime-cpp/libruntime.so || cp target/release/libruntime.dylib pkg/runtime-cpp/libruntime.so
	mkdir -p pkg/modules-wasm
	$(CARGO) build --package dex --release --target wasm32-wasi
	cp target/wasm32-wasi/release/dex.wasm pkg/modules-wasm/dex.wasm

clean:
	cargo clean

clean-pkg:
	rm -rf pkg

clean-all: clean clean-pkg