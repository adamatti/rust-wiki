.DEFAULT_GOAL := help

set-env: ## set nightly build
	@rustup default nightly

clean: ## remove temp files
	@rm -rf target

release: ## generate release version
	@cargo build --release

build: ## generate debug version
	@RUST_BACKTRACE=full RUSTUP_TOOLCHAIN=nightly cargo build

run: ## run project as dev
	@RUST_BACKTRACE=1 cargo run

run-prod: ## run project as prod
	@cargo run --release

test: ## run tests
	@cargo test

watch-test: ## run tests with watch
	@cargo watch -x test

watch-run: ## run project with watch
	@cargo watch -x run

build-all: build release

dc-stop:
	@docker-compose stop -t 0

dc-buid: dc-stop
	@docker-compose build

dc-up: dc-buid
	@docker-compose up app

dc-sh: dc-buid
	@docker-compose run app sh

docker-build:
	@docker build . -t rust-wiki:latest

.PHONY: help
help: ## show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'