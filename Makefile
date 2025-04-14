.PHONY: build docs clean

build:
    cargo build --release --target wasm32-unknown-unknown

docs:
    cargo doc --no-deps --open

clean:
    cargo clean
