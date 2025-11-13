#!/usr/bin/env just --justfile

set shell := ["bash", "-uc"]

export RUSTDOCFLAGS := "--cfg docsrs"

check:
    cargo check

fmt toolchain="+nightly":
    cargo {{toolchain}} fmt

fmt-check toolchain="+nightly":
    cargo {{toolchain}} fmt --check

lint:
    cargo clippy --no-deps -- -D warnings

fix:
	cargo fix --allow-dirty --allow-staged

[group('doc')]
doc:
    cargo +nightly doc --all-features
    ln -s target/doc doc/rs || true

graph:
    rm -f cargo-graph.dot
    rm -f cargo-graph.png
    cargo depgraph | dot -Tpng > cargo-graph.png

all: check fmt lint
