#!/bin/bash


echo "Updating System"

sudo pacman -Syyu

git pull

cargo clean

rustup update

rustup default nightly

cargo update

cargo build --release

./target/release/csa_backend

