export PATH="/builder/rust/build/x86_64-unknown-linux-gnu/stage1/bin:$PATH"
export PATH="/builder/rust/build/x86_64-unknown-linux-gnu/stage1-tools-bin/:$PATH" 
cargo clean
cargo build  --target aarch64-unknown-freebsd-purecap 
