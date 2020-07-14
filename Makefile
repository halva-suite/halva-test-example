.PHONY: build
build:
	cargo build --release

.PHONY: dev
dev:
	./target/release/node-template --dev

.PHONY: dev/purge
dev/purge:
	./target/release/node-template purge-chain --dev

.PHONY: token/purge
token/purge:
	./target/release/node-template purge-chain --base-path /tmp/token01
	./target/release/node-template purge-chain --base-path /tmp/token02

.PHONY: token01
token01:
	./target/release/node-template \
		--base-path /tmp/token01 \
		--chain=./customSpecRaw.json \
		--port 30333 \
		--ws-port 9944 \
		--rpc-port 9933 \
		--validator \
		--name MyToken01 \
		-l=debug

.PHONY: token02
token02:
	./target/release/node-template \
		--base-path /tmp/token02 \
		--chain=./customSpecRaw.json \
		--port 30334 \
		--ws-port 9945 \
		--rpc-port 9934 \
		--name MyToken02 --unsafe-rpc-expose \
		-l=debug
