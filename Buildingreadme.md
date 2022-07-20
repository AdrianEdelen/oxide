Due to the nature of kernel development, there are some odd steps that must be taken in order to build the project

Additionally, for testing, it is Highly recommended that Qemu is used with this configuration.

Qemu is used for unit tests, integration tests, and for running the software. Otherwise, you will have to deploy the kernel to bare metal to run.

first build may take a long time since core needs to be compiled, this project does not use the std lib. (since we don't have it on bare metal)

## Windows
Download Qemu and make sure that it is in your PATH
make sure that the target is x86_64-os.json in config.toml, this will build for bare metal x86-64 when no target is specified.
run the command `rustup override set nightly` in your terminal. this project requires expiremental features that may not have been implemented in the current version of rust.

when running the bootimage crate should start qemu automatically.

cargo test to run all tests (unit and integration).
cargo build to built
cargo run to start Qemu and load the kernel.

##