CARGO ?= cargo
CBINDGEN ?= cbindgen

build-wasm-pkg :
	$(CARGO) build --package runtime --release
	$(CBINDGEN) --config=cbindgen.toml --crate runtime --output pkg/runtime-cpp/includes/runtime.h
	cp target/release/libruntime.so pkg/runtime-cpp/libruntime.so
	mkdir -p pkg/modules-wasm
	$(CARGO) wasi build --package dex --release
	cp target/wasm32-wasi/release/dex.wasm pkg/modules-wasm/dex.wasm

build-grpc-pkg:
	$(CARGO) build --package ain-grpc --release
	mkdir -p pkg/ain-grpc/include pkg/ain-grpc/lib
	cp target/release/libain_grpc.a pkg/ain-grpc/lib/
	cp target/libain.hpp pkg/ain-grpc/include/
	cp target/libain.cpp pkg/ain-grpc/

# TODO: Merge this with grpc package when ain links both
build-core-pkg:
	$(CARGO) build --package ain-core --release
	mkdir -p pkg/ain-core/include pkg/ain-core/lib
	cp target/release/libain_core.a pkg/ain-core/lib/
	cp target/libain_core.hpp pkg/ain-core/include/
	cp target/libain_core.cpp pkg/ain-core/
