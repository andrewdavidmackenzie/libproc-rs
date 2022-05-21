.PHONY: all
all: clippy run-tests build-docs

.PHONY: clippy
clippy:
	cargo clippy --all --tests --all-targets --all-features -- -D warnings

.PHONY: configure-coverage
configure-coverage:
	cargo install grcov
	rustup component add llvm-tools-preview
	export RUSTFLAGS="-Zinstrument-coverage"
	echo RUSTFLAGS="-Zinstrument-coverage" >> "$GITHUB_ENV"
	export LLVM_PROFILE_FILE="libproc-%p-%m.profraw"
	echo LLVM_PROFILE_FILE="libproc-%p-%m.profraw" >> "$GITHUB_ENV"

.PHONY: run-tests
run-tests:
	cargo test

.PHONY: run-tests-root
run-tests-root:
	sudo cargo test

.PHONY: upload-coverage
uppload-coverage:
	grcov . --binary-path target/debug/ -s . -t lcov --branch --ignore-not-existing --ignore "/*" -o lcov.info
	bash <(curl -s https://codecov.io/bash) -f lcov.info
	rm -f lcov.info

.PHONY: build-docs
build-docs:
	cargo doc --workspace --quiet --all-features --no-deps --target-dir=target