#!/usr/bin/env bash

p=/home/jiang/rs/rx

cd ${p}
cargo build

src=${p}/target/debug
dst=/usr/local/bin
sudo cp ${src}/binder ${dst}

src=${p}/bin
sudo cp ${src}/*.sh ${dst}

ls -lh ${dst} | grep -E "rx-|binder"
