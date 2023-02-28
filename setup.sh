#!/bin/bash




echo "Updating System"

sudo pacman -Syyu

brew upgrade

sudo apt update

sudo apt upgrade

git pull

# shellcheck disable=SC2164
cd microService

cd firestore_send

go build

# shellcheck disable=SC2103
cd ..

cd ..

cargo clean

rustup update

rustup default nightly

cargo update

cargo build --release

./target/release/r1403_vison

