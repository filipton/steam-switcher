#/bin/bash

if ! command -v rustup &> /dev/null
then
    echo "Rustup could not be found"
    exit
fi

rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu

sudo cp ./target/x86_64-unknown-linux-gnu/release/rusty-switcher /usr/local/bin/rusty-switcher
