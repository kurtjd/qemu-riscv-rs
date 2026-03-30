# Delete old files
rm device.x
rm build.rs
rm -rf src

# Generate the PAC using the config
svd2rust --target riscv --edition=2024 --settings config.yml -i qemu-riscv.svd
# Finally properly split up the monolithic file into a nice folder structure
form -i lib.rs -o src
# And cleanup
rm lib.rs
cargo fmt