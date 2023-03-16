.PHONY: build
build:
	cargo build --target wasm32-wasi

.PHONY: run
run:
	slight -c slightfile.toml run target/wasm32-wasi/debug/wasmio-demo.wasm
