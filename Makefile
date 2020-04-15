.DEFAULT_GOAL := all

.PHONY: default test

build:
	cargo build --target=arm-unknown-linux-gnueabihf -v
	#cargo xrustc --target=arm-unknown-linux-gnueabihf -v
release:
	cargo build --target=arm-unknown-linux-gnueabihf --realease