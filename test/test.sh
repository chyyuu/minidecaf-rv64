#!/bin/bash

assert() {
    in=$1
    ans=$2
    cargo run -- $1 > out/asm.S

    riscv64-unknown-elf-gcc out/asm.S -o out/run
    spike pk out/run

    res=$?
    
    if [ $res == $ans ]; then
        echo "OK"
    else
        echo -e "\033[0;31mNG\033[0;39m"
        echo "Result: $res, Ans: $ans"
    fi
}


cd $(cd $(dirname ${BASH_SOURCE:-$0}); pwd)
mkdir -p out

assert "0;" 0
assert "10;" 10
assert "1+3;" 4
assert "1+3+8;" 12
assert "4-2;" 2
assert "4-18;" 242
assert "1+3-2;" 2 
assert "1*3+4;" 7
assert "3+2*4-1;" 10
assert "4/2+2*4/1;" 10
assert "1+2*3*4;" 25
assert "3*(2+4);" 18
assert "1+((2)+4/2);" 5
assert "(3+4*2)+3*(1+2)-4;" 16
assert "-3;" 253
assert "-3+5;" 2
assert "100+-3*-3;" 109
assert "-2*+8;" 240
assert "4==4;" 1
assert "4==2;" 0
assert "4!=4;" 0
assert "4!=2;" 1
assert "4<5;" 1
assert "4<4;" 0
assert "4<=4;" 1
assert "4<3;" 0
assert "4>4;" 0
assert "4>3;" 1
assert "4>=5;" 0
assert "4>=4;" 1
assert "4+ 4*2 >=15+3*(-2);" 1
assert "a=3; b=a+1; b;" 4
assert "a=3; b=1+a*a; b;" 10
assert "a=3*2; b=2; c=2+b*a+10+2*b;" 28
assert "abc=3*2; bcde=2; c=2+bcde*abc;" 14
assert "abc=3*2; xy=6; abc==xy;" 1
