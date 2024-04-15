# smtp-test

## Compiling

### For Alpine

1. 
    ```sh
    dnf install musl-gcc
    ```
2. 
    ```sh
    rustup target add x86_64-unknown-linux-musl
    ```
3. 
    ```sh
    cargo build --release --target x86_64-unknown-linux-musl
    ```
