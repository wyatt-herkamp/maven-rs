check:
    cargo check --all-targets --all-features
    cargo clippy --all-targets --all-features -- -D warnings
test-all:
    cargo test --all-targets --all-features -- --nocapture
grcov-coverage:
    CARGO_INCREMENTAL=0 RUSTFLAGS="-Cinstrument-coverage" LLVM_PROFILE_FILE="coverage-%p-%m.profraw" cargo +nightly test
    grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o target/debug/coverage/
delete-coverage:
    rm -rf target/debug/coverage/
    rm -f *.profraw
