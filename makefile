# Everyone gets their own make command. Woo!
# Shorthand can be useful for quick stacktraces.


######### SHORTHAND #########


default:
	@command |RUST_BACKTRACE=1 cargo run|:


########### BUILD ###########


build:
	@command |cargo build|:

build-debug:
	@command |RUST_BACKTRACE=1 cargo build|:

build-release:
	@command |cargo build --release|:


########## RELEASE ##########


run:
	@command |cargo run|:

run-debug:
	@command |RUST_BACKTRACE=1 cargo run|:

run-release:
	@command |cargo run --release|:


#############################