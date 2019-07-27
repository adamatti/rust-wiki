.DEFAULT_GOAL := help

clean:
	@rm -rf target

release: clean
	@cargo build --release

run:
	@cargo run

.PHONY: help
help: ## show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'