.PHONY: install build test lint lint-fix format format-fix help

BUN ?= bun
CARGO ?= cargo
MAKEFLAGS += --no-builtin-rules

install:
	$(BUN) install
	$(BUN) run prepare

build:
	$(CARGO) build --workspace

test:
	$(CARGO) test --workspace

lint:
	$(CARGO) clippy --workspace --all-targets --all-features -- -D warnings

lint-fix:
	$(CARGO) clippy --fix --allow-dirty --allow-staged --workspace --all-targets --all-features -- -D warnings

format:
	$(CARGO) fmt --all --check
	$(BUN) run format:ts:check

format-fix:
	$(CARGO) fmt --all
	$(BUN) run format:ts

help:
	@printf '%s\n' \
		'make install' \
		'make build' \
		'make test' \
		'make lint' \
		'make lint:fix' \
		'make format' \
		'make format:fix'

%:
	@case "$@" in \
		lint:fix) $(MAKE) lint-fix ;; \
		format:fix) $(MAKE) format-fix ;; \
		*) printf 'Unknown target: %s\n' "$@"; exit 2 ;; \
	esac
