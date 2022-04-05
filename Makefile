CARGO ?= cargo
CBINDGEN ?= cbindgen

build-pkg :
	$(CARGO) build --package runtime --release
	$(CBINDGEN)  --config=cbindgen.toml --crate runtime --output pkg/runtime-cpp/include/runtime.h
	mkdir -p pkg/runtime-cpp/lib
	cp target/release/libruntime.a pkg/runtime-cpp/lib
	mkdir -p pkg/modules-wasm
	$(CARGO) build --package dex --release --target wasm32-wasi
	cp target/wasm32-wasi/release/dex.wasm pkg/modules-wasm/dex.wasm

clean:
	cargo clean

clean-pkg:
	rm -rf pkg

clean-all: clean clean-pkg

