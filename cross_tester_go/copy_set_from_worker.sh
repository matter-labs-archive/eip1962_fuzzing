#!/bin/sh
FILES="SIGABRT.PC.7ffff7be73eb.STACK.e07bfc9b2.CODE.-6.ADDR.(nil).INSTR.mov____0x108(%rsp),%rax.2020-01-07.06:56:16.30846.fuzz
SIGABRT.PC.7ffff7be73eb.STACK.e07bfc9b2.CODE.-6.ADDR.(nil).INSTR.mov____0x108(%rsp),%rax.2020-01-06.23:41:40.10648.fuzz
SIGABRT.PC.7ffff7be73eb.STACK.e07bfc9b2.CODE.-6.ADDR.(nil).INSTR.mov____0x108(%rsp),%rax.2020-01-07.06:48:40.29878.fuzz
SIGABRT.PC.7ffff7be73eb.STACK.e07bfc9b2.CODE.-6.ADDR.(nil).INSTR.mov____0x108(%rsp),%rax.2020-01-07.00:51:14.21100.fuzz"
for f in $FILES
do
	echo "Processing $f"
    scp root@165.22.69.114:/root/eip1962_fuzzing/honggfuzz/hfuzz_workspace/fuzz_target_compare_ops/"'$f'" ./fuzz_vectors/
done
