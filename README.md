# demo-rust-wasi

rustup target add wasm32-wasi

cargo build --target wasm32-wasi

wasmtime --dir=. target/wasm32-wasi/debug/demo-rust-wasi.wasm input.txt output.txt

千万不要忘记用 --dir=. 将本地目录映射到虚拟机环境下。

