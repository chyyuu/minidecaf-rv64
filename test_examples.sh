
riscv64-unknown-elf-gcc ./examples/step1/return_0.c            #compile with gcc
qemu-riscv64 ./a.out                 	#run it
expected=$?             	#get exit code
/media/chyyuu/ca8c7ba6-51b7-41fc-8430-e29e31e5328f/thecode/rust/compilers/minidecaf-rv64/target/debug/minidecaf 0  > asm.S 
riscv64-unknown-elf-gcc asm.S -o out   #compile with YOUR COMPILER or some shell script with YOUR COMPILER
base="examples/step1/return_0"
qemu-riscv64 ./out                      #run the thing we assembled
actual=$?                  #get exit code
echo -n "$base:    "
if [ "$expected" -ne "$actual" ]
then
echo "FAIL"
else
echo "OK"
fi
rm -f out a.out asm.S 2>/dev/null


riscv64-unknown-elf-gcc ./examples/step1/return_2.c            #compile with gcc
qemu-riscv64 ./a.out                 	#run it
expected=$?             	#get exit code
/media/chyyuu/ca8c7ba6-51b7-41fc-8430-e29e31e5328f/thecode/rust/compilers/minidecaf-rv64/target/debug/minidecaf 2  > asm.S 
riscv64-unknown-elf-gcc asm.S -o out   #compile with YOUR COMPILER or some shell script with YOUR COMPILER
base="examples/step1/return_2"
qemu-riscv64 ./out                      #run the thing we assembled
actual=$?                  #get exit code
echo -n "$base:    "
if [ "$expected" -ne "$actual" ]
then
echo "FAIL"
else
echo "OK"
fi
rm -f out a.out asm.S 2>/dev/null

#tested in ubutnu 20.04 x86-64
