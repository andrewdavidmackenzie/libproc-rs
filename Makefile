RUST_MIN_VERSION := 1.72.0
ACT := $(shell command -v act 2> /dev/null)
UNAME := $(shell uname -s)

.PHONY: all
all: clippy test build-docs

.PHONY: clippy
clippy:
	@cargo clippy --all --tests --no-deps --all-targets --all-features -- --warn clippy::pedantic -D warnings

.PHONY: test
test:
ifeq ($(UNAME),Darwin)
	@echo "On macos, process tests are required to be run as root - so please enter your password at the prompt"
	@sudo env "PATH=$$PATH" cargo test
else
	@env "PATH=$$PATH" cargo test
endif

.PHONY: coverage
coverage: test-with-coverage gen-coverage view-coverage

.PHONY: test-with-coverage
test-with-coverage:
	@RUSTFLAGS="-C instrument-coverage" LLVM_PROFILE_FILE="libproc-%p-%m.profraw" cargo build
ifeq ($(UNAME),Darwin)
	@echo "On macos, process tests are required to be run as root - so please enter your password at the prompt"
	@RUSTFLAGS="-C instrument-coverage" LLVM_PROFILE_FILE="libproc-%p-%m.profraw" sudo cargo test
else
	@RUSTFLAGS="-C instrument-coverage" LLVM_PROFILE_FILE="libproc-%p-%m.profraw" cargo test
endif
	@make upload-coverage

.PHONY: gen-coverage
gen-coverage:
	@grcov . --binary-path target/debug/ -s . -t lcov --branch --ignore-not-existing --ignore "/*" -o coverage.info
	#@lcov --remove coverage.info '/Applications/*' 'target/debug/build/**' 'target/release/build/**' '/usr*' '**/errors.rs' '**/build.rs' 'examples/**' '*tests/*' -o coverage.info
	#@find . -name "*.profraw" | xargs rm -f

.PHONY: upload-coverage
upload-coverage:
	bash <(curl -s https://codecov.io/bash) -f coverage.info

.PHONY: view-coverage
view-coverage:
	@genhtml -o target/coverage --quiet coverage.info
	@echo "View coverage report using 'open target/coverage/index.html'"

.PHONY: build-docs
build-docs:
	cargo doc --workspace --quiet --all-features --no-deps --target-dir=target

.PHONY: matrix
matrix:
	@for rust_version in stable beta nightly $(RUST_MIN_VERSION) ; do \
        echo rust: $$rust_version ; \
        rustup override set $$rust_version ; \
        make clippy ; \
        make test ; \
    done
ifeq ($(UNAME),Darwin)
ifneq ($(ACT),)
	@echo "Running Linux GH Action workflow using `act` on macos"
	@act -W .github/workflows/clippy_build_test.yml
else
	@echo "`act` is not installed so skipping running Linux matrix"
endif
else
	@echo "Cannot run Linux parts of matrix on macos, create PR and make sure all checks pass"
endif
