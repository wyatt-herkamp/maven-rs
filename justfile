check:
    cargo check --all-targets --all-features
    cargo clippy --all-targets --all-features -- -D warnings
test-all:
    cargo test --all-targets --all-features -- --nocapture