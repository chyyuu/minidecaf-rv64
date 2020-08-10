cargo run -- $1 >asm.S
riscv64-unknown-elf-gcc asm.S -o out 
qemu-riscv64 ./out
echo $?

