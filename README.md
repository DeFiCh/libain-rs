# Libain-rs

## Setup environment
### Install Rust 
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
### Install Wasm tools
```shell
curl https://wasmtime.dev/install.sh -sSf | bash
```
```shell
rustup target add wasm32-wasi
```
```shell
cargo install cargo-wasi
```

## Build
```shell
cargo build
```