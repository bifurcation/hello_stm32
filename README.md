# Rust on STM32 Hello World

Borrowed from [this article] on embedded Rust.

Quickstart:

```
# Set up the toolchain
rustup update
rustup component add llvm-tools-preview
rustup target add thumbv7em-none-eabihf
cargo install cargo-binutils cargo-embed cargo-flash cargo-expand

# To build, flash, and run automagically (I have not tried)
cargo embed

# To create a .bin
cargo build
cargo objcopy --release -- -O binary app.bin
```

[this article]: https://medium.com/digitalfrontiers/rust-on-a-stm32-microcontroller-90fac16f6342
