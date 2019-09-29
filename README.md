### A Rust OS from https://os.phil-opp.com/

#### How to run
* `cargo bootimage`
* `qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/debug/bootimage-rust_os.bin`