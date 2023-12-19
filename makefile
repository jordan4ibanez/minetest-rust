# Everyone gets their own make command. Woo!
default:
	@command |RUST_BACKTRACE=1 cargo run|:

debug:
	@command |RUST_BACKTRACE=1 cargo run|:

run-debug:
	@command |RUST_BACKTRACE=1 cargo run|:

build:
	@command |cargo build|:

build-release:
	@command |cargo build --release|:

run-release:
	@command |cargo run --release|:

run:
	@command |cargo run|:


