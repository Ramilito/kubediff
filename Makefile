.PHONY: build
build:
	cargo build
	cp ./src/assets/diff.sh ./target/debug/
	cp ./src/assets/config.yaml ./target/debug/

.PHONY: build_local
build_local: build
	cargo build --release
	mkdir -p ~/.kube/kubediff
	cp ./target/release/kubediff ~/.kube/kubediff/
	cp ./src/assets/diff.sh ~/.kube/kubediff/
	cp ./src/assets/config.yaml ~/.kube/kubediff/

.PHONY: run
run:
	cargo run -- -e local

.PHONY: deploy_local
deploy_local: build_local run

