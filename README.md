# demo-rust-wasi

rustup target add wasm32-wasi

cargo build --target wasm32-wasi

wasmtime --dir=. target/wasm32-wasi/debug/demo-rust-wasi.wasm input.txt output.txt

千万不要忘记用 --dir=. 将本地目录映射到虚拟机环境下。

此DEMO虽然很简单，但却足以证明： 若我们开发一个可运行在wasmer或wasmtime环境下的公链，这个公链可将区块数据存储在本地磁盘上，可对文件正常进行读写操作。
