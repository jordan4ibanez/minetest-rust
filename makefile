# Everyone gets their own make command. Woo!
default:
	RUST_BACKTRACE=1 cargo run

debug:
	RUST_BACKTRACE=1 cargo run

run-debug:
	RUST_BACKTRACE=1 cargo run

build:
	cargo build

build-release:
	cargo build --release

run-release:
	cargo run --release

run:
	cargo run


