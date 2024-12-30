export RUSTFLAGS="--cfg=web_sys_unstable_apis"
cargo build --release --target wasm32-unknown-unknown

wasm-bindgen --no-typescript --target web --out-dir ./dist/ --out-name "match-3" ./target/wasm32-unknown-unknown/release/match-3-game.wasm

cp ./src/index.html ./dist/
