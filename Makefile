.PHONY: start-server
start-server: # CTRL+C to stop
	docker run -it --rm \
		-p 26657:26657 -p 26656:26656 -p 1337:1337 \
		-v $$(pwd):/root/code \
		--name secretdev enigmampc/secret-network-sw-dev:v1.2.0-1

.PHONY: compile
compile: # CTRL+C to stop
	RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown

.PHONY: setup-devchain
setup-devchain: # CTRL+C to stop
	make compile
	bash devChain/devChain_setup.sh