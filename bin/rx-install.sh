#!/usr/bin/env bash

p=/home/jiang/rs/rx

cd ${p}
cargo build --release

src=${p}/target/release
dst=/usr/local/bin
sudo cp ${src}/binder ${dst}

src=${p}/bin
sudo cp ${src}/*.sh ${dst}

ls -l ${dst} | grep -E "rx-|binder"
