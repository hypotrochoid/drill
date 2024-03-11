run-local:
    RUST_BACKTRACE=1 cargo run --target x86_64-unknown-linux-gnu -- -d samples

build:
    RUST_BACKTRACE=1 cargo build --target x86_64-unknown-linux-gnu