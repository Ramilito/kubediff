.PHONY: build
build:
	cargo build

.PHONY: run
run:
	cargo run

.PHONY: deploy_local
deploy_local: run
	cp ./src/assets/diff.sh ./target/debug/


