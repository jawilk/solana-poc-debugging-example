# solana-poc-debugging-example
[gif1]: ./media/debugging_example.gif
![alt text][gif1]

We use the gdb remote protocol https://sourceware.org/gdb/onlinedocs/gdb/Remote-Protocol.html to connect to the rbpf vm using https://github.com/daniel5151/gdbstub. Please see credits at the bottom for the original idea and code.

## Testing the hello world program (https://github.com/solana-labs/example-helloworld)
You need to build GDB with bpf support and a patch to adjust register size of sp/pc to 64-bit  
```
git clone https://github.com/jawilk/binutils-gdb
./configure bpf   
make
cd gdb
cp <path-to-this-repo>/tests/elfs/helloworld_rust_debug.so .
./gdb helloworld_rust_debug.so
```
Then in this repos root
```
cargo run --bin helloworld
```

## Creating new debug files  
Please note: larger projects might not work as of now because unoptimized binaries may exceed the max stack size.

It's necessary to build rust with a slightly different version of llvm to enable useable dwarf info (this will take a while).
```
git clone https://github.com/jawilk/rust.git
cd rust
./build
```
Now we build two binary versions of the project you want to debug. Assuming you already have bpf-tools v1.23 installed.  
Go to the root of the project you want to debug.
```
cargo +bpf build --target bpfel-unknown-unknown
```
Make sure there is no --release flag since we need the unoptimized binary. Copy target/bpfel-unknown-unknown/debug/<your-project.so> to tests/elfs in this repo.  
Now the debugging version.
```
cp .cache/solana/v1.23/bpf-tools/rust/bin/cargo <path-to-cloned-rust-repo>/build/x86_64-unknown-linux-gnu/stage1/bin/
mv .cache/solana/v1.23/bpf-tools/rust .cache/solana/v1.23/bpf-tools/rust_normal/
ln -s <path-to-cloned-rust-repo>/build/x86_64-unknown-linux-gnu/stage1 .cache/solana/v1.23/bpf-tools/rust
```
Again in the project root 
```
cargo +bpf build --target bpfel-unknown-unknown
```
Rename target/bpfel-unknown-unknown/debug/<your-project.so> to something else and copy it to tests/elfs. Adjust the filenames in your poc code.
Don't forget to remove the symlink afterwards and rename the 'rust_normal' folder to 'rust'.  

## Changes made to GDB and llvm
- GDB remote was sending 88 bytes packages for the bpf target registers, i.e. 10 64-bit general purpose regs and 2x 32-bit for sp/pc. This was fine for instruction stepping but not memory access. The patch made them all 64-bit regs
- llvm relocs described here https://github.com/solana-labs/llvm-project/pull/25
- In the DWARF info, the DW_AT_location for local vars and function args had positive offsets (DW_OP_fbreg) from the frame base. This was (I think) calculated with the max stack usage for each call frame. This was changed to use the direct offset from R10. Now the value is a negative offset from the current (read-only) frame pointer.

## Useful GDB commands
```
(gdb) p _instruction_data[0]@<size>
```
This prints size many values after _instruction_data[0], i.e. array access.  
Don't forget to dereference (*) a pointer value to get to the real data.  
After function entry another step command might be needed to skip the prologue and make the function args accessible.

## Credits
This is mostly glued together code/work done by other people. Besides the obvious forks:  
gdbstub code:  
https://github.com/solana-labs/solana/issues/14756  
https://github.com/Sladuca/rbpf/blob/d98dc270083736d23b08972653e41f60561dc666/src/gdb_stub.rs  
important llvm relocation patch:  
https://github.com/solana-labs/llvm-project/pull/25  
This repo is based on https://github.com/neodyme-labs/neodyme-breakpoint-workshop
