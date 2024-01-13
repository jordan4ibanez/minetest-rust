# Everyone gets their own make command. Woo!
# Shorthand can be useful for quick stacktraces.


######### SHORTHAND #########


default:
	RUST_BACKTRACE=1 cargo run

server:
	RUST_BACKTRACE=1 cargo run -- -s

server-debug:
	RUST_BACKTRACE=1 cargo run -- -s

server-release:
	cargo run --release -- -s

########### BUILD ###########


build:
	cargo build

build-debug:
	RUST_BACKTRACE=1 cargo build

build-release:
	cargo build --release


########## RELEASE ##########


run:
	cargo run

run-debug:
	RUST_BACKTRACE=1 cargo run

run-release:
	cargo run --release


########## TESTING ##########


# Useful for seeing println output during unit tests.
test:
	cargo test -- --nocapture


#############################