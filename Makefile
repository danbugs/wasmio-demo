.PHONY: build
build:
	cargo build --target wasm32-wasi

.PHONY: run
run:
	slight -c slightfile.toml run target/wasm32-wasi/debug/wasmio-demo.wasm

.PHONY: image
image:
	docker buildx build --platform=wasi/wasm -t danstaken/wasmio2023-demo .
	docker push danstaken/wasmio2023-demo:latest

