
verify: lint test

test:
    cargo test

lint:
    cargo fmt --all -- --check
    cargo clippy

fmt:
    cargo fmt
    cargo fix --allow-dirty --allow-staged

build-linux:
    cargo build --release --target x86_64-unknown-linux-gnu
    mkdir -p dist
    cp target/x86_64-unknown-linux-gnu/release/sqlc-gen-rust dist/
    chmod +x dist/sqlc-gen-rust