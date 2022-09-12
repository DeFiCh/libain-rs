CARGO ?= cargo
CBINDGEN ?= cbindgen
TARGET ?=

build-wasm-pkg :
	$(CARGO) build --package runtime --release
	$(CBINDGEN) --config=cbindgen.toml --crate runtime --output pkg/runtime-cpp/includes/runtime.h
	cp target/release/libruntime.so pkg/runtime-cpp/libruntime.so
	mkdir -p pkg/modules-wasm
	$(CARGO) wasi build --package dex --release
	cp target/wasm32-wasi/release/dex.wasm pkg/modules-wasm/dex.wasm

# NOTE: CRATE_CC_NO_DEFAULTS=1 is necessary so that cxx doesn't cause any cross-compilation
# issues when using `cc-rs`. Tracking issue: https://github.com/rust-lang/cc-rs/issues/710

# TODO: Merge this with core package when ain links both
build-grpc-pkg:
	CRATE_CC_NO_DEFAULTS=1 $(CARGO) build --package ain-grpc --release $(if $(TARGET),--target $(TARGET),)
	mkdir -p pkg/ain-grpc/include pkg/ain-grpc/lib
	cp target/release/libain_grpc.a pkg/ain-grpc/lib/
	cp target/libain_rpc.hpp pkg/ain-grpc/include/
	cp target/libain_rpc.cpp pkg/ain-grpc/

build-core-pkg:
	CRATE_CC_NO_DEFAULTS=1 $(CARGO) build --package ain-core --release $(if $(TARGET),--target $(TARGET),)
	mkdir -p pkg/ain-core/include pkg/ain-core/lib
	cp target/$(if $(TARGET),$(TARGET)/,)release/libain_core.a pkg/ain-core/lib/
	cp target/libain_core.hpp pkg/ain-core/include/
	cp target/libain_core.cpp pkg/ain-core/
