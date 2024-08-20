#!/usr/bin/just

format:
	# Use cargo nightly features for formating
	cargo +nightly fmt

clippy:
	cargo clippy

build:
	cargo build

release:
	cargo build --release

audit:
	cargo audit

common: clippy format


