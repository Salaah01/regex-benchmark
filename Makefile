SHELL := /bin/bash

compile-windows:
	@echo "Compiling for Windows"
	@cargo build --target x86_64-pc-windows-gnu --release

compile-linux:
	@echo "Compiling for Linux"
	@cargo build --target x86_64-unknown-linux-gnu --release

compile-macos:
	@echo "Compiling for MacOS"
	@cargo build --target x86_64-apple-darwin --release