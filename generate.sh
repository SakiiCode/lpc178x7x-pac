svd2rust -i LPC178x_7x_v0.8.svd --target cortex-m
rm -rf src
form -i lib.rs -o src/ && rm lib.rs
cargo fmt