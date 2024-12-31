if [[ -n $(git status --porcelain) ]]; then 
	echo "Please commit all changes before building"
	exit 1
fi

git checkout gh-pages
git reset --hard main

export RUSTFLAGS="--cfg=web_sys_unstable_apis"
cargo build --release --target wasm32-unknown-unknown

mkdir -p docs

wasm-bindgen --no-typescript --target web --out-dir ./docs/ --out-name "match-3" ./target/wasm32-unknown-unknown/release/match-3-game.wasm

cp ./src/index.html ./docs/

git add -A
git commit -m "Build"

git push --force
git checkout main
