:loop
cargo build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/cake_test.wasm --target web --out-dir target/web
cd target/web
node server.js
cd ../..
goto loop