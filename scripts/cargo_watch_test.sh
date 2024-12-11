#/bin/bash
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
export RUST_BACKTRACE="${RUST_BACKTRACE:-1}"
RUSTFLAGS=-Awarnings "$SCRIPT_DIR/cargo.sh" watch -- cargo test "$@"
