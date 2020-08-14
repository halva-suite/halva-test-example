.PHONY: build
build:
	cargo build --release

.PHONY: dev
dev:
	./target/release/node-template --dev

.PHONY: dev/purge
dev/purge:
	./target/release/node-template purge-chain --dev
