build:
	cargo build

run-bundler:
	cargo run -- --mnemonic-file ${HOME}/.aa-bundler/0x129D197b2a989C6798601A49D89a4AEC822A17a3 --beneficiary 0x690B9A9E9aa1C9dB991C7721a92d351Db4FaC990 --gas-factor 600 --min-balance 1 --eth-client-address http://127.0.0.1:8545 --entry-points 0x0000000000000000000000000000000000000000 --chain-id 5 --helper 0x0000000000000000000000000000000000000000 --max-verification-gas 1500000

run-bundler-uopool:
	cargo run --bin bundler-uopool -- --entry-points 0x0000000000000000000000000000000000000000 --chain-id 5 --eth-client-address http://127.0.0.1:8545 --max-verification-gas 1500000

run-bundler-rpc:
	cargo run --bin bundler-rpc

run-create-wallet:
	cargo run --bin create-wallet -- --output-path ${HOME}/.aa-bundler

setup-thirdparty:
	git submodule update --init
	cd thirdparty/account-abstraction && yarn install && yarn compile && cd ../..
	cd thirdparty/bundler && yarn install && yarn preprocess && cd ../..

test:	
	cargo test 

format:
	cargo fmt --all

lint:
	cargo fmt --all -- --check
	cargo clippy -- -D warnings -A clippy::derive_partial_eq_without_eq -D clippy::unwrap_used

clean:
	cd thirdparty/account-abstraction && yarn clean && cd ../..
	cd thirdparty/bundler && yarn clear && cd ../..
	cargo clean
