cd ./blog
cargo web deploy --release
cp ./target/deploy/* ../static
