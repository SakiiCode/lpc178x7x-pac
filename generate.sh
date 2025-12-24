svd2rust -i LPC178x7x-mod.svd --target cortex-m --reexport-interrupt --edition=2024
rm -rf src
form -i lib.rs -o src/ && rm lib.rs
cargo fmt