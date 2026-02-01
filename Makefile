.PHONY: help build test check fmt clippy clean run example all docs

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-15s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

all: check ## Run full check (format, clippy, test)

build: ## Build all crates
	cargo build --all-features

check: fmt clippy test ## Run all checks

test: ## Run tests
	cargo test --all-features

test-watch: ## Run tests in watch mode
	cargo watch -x test

fmt: ## Check formatting
	cargo fmt --all -- --check

fmt-fix: ## Fix formatting
	cargo fmt --all

clippy: ## Run clippy
	cargo clippy --all-targets --all-features -- -D warnings

clean: ## Clean build artifacts
	cargo clean
	rm -rf examples/output

run: ## Run CLI (example: make run run --help)
	cargo run --release --bin synton --

docs: ## Generate documentation
	cargo doc --no-deps --all-features --open

docs-deps: ## Generate documentation with dependencies
	cargo doc --all-features --open

bench: ## Run benchmarks
	cargo bench --all-features

coverage: ## Generate code coverage
	cargo tarpaulin --out Html --all-features --timeout 120 -- -Z unstable-options

release: ## Build release binary
	cargo build --release --bin synton

install: ## Install CLI locally
	cargo install --path cli

example-%: ## Run example (e.g., make example-fib)
	cargo run --release --bin synton -- run examples/$*.synton

repl: ## Start REPL
	cargo run --release --bin synton -- repl

lint: ## Run all linters
	@$(MAKE) fmt
	@$(MAKE) clippy
	@cargo check --all-targets --all-features

watch: ## Watch for changes and rebuild
	cargo watch -x build

# Development shortcuts
dev: fmt-fix clippy ## Format and check
	cargo build
