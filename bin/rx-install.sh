#!/usr/bin/env bash

src=/home/jiang/rs/rx/target/release
dst=/usr/local/bin
sudo cp ${src}/dwh-api ${dst}

src=/home/jiang/rs/dwh/bin/
sudo cp ${src}/dwh*.sh ${dst}

ls -l ${dst} | grep dwh-