cd ./blog
cargo web build --release
cp ./target/wasm32-unknown-unknown/release/blog.js target/wasm32-unknown-unknown/release/blog.wasm ../static
cp ./static/* ../static
