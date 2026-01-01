SHELL := /bin/bash

LATEST_TAG := $(shell git describe --abbrev=0 --tags $(git rev-list --tags --max-count=1))

define bundle_release
	@echo ""
	if [[ "$(1)" == *"windows"* ]]; then  \
		tar -czvf ./target/$(1)/release/kubediff_$(1).tar.gz src/assets/ ./target/$(1)/release/kubediff.exe; \
	else \
		tar -czvf ./target/$(1)/release/kubediff_$(1).tar.gz src/assets/ ./target/$(1)/release/kubediff; \
	fi
endef

.PHONY: bundle_release
bundle_release:
	$(call bundle_release,${TARGET})
	
.PHONY: build
build:
	cargo build
	cp ./src/assets/config.yaml ./target/debug/

.PHONY: build_local
build_local: build
	cargo build --release
	mkdir -p ~/.kube/kubediff
	cp ./target/release/kubediff ~/.kube/kubediff/
	cp ./src/assets/config.yaml ~/.kube/kubediff/

.PHONY: run
run:
	cargo run -- -e local

.PHONY: deploy_local
deploy_local: build_local run

