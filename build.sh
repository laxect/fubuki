yarn install
yarn run build
cp public/* dist/ -r
cargo run --bin index_gen --release -- public/post dist
cp app.yaml dist/app.yaml
