#!/bin/bash

source $HOME/.cargo/env
export LD_LIBRARY_PATH=/home/ubuntu/spdk_install/lib
export RUST_BACKTRACE=1
export SPDK_INCLUDE=/home/ubuntu/spdk_install/include
export SPDK_LIB=/home/ubuntu/spdk_install/lib
cargo test --all
