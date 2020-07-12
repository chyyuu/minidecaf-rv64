#!/bin/bash

cd $(cd $(dirname ${BASH_SOURCE:-$0}); pwd)
mkdir -p out

cargo run "$@" > out/asm.S
riscv64-unknown-elf-gcc out/asm.S -o out/run
spike pk out/run
echo $?
