.PHONY: help up down run check test fmt lint migrate-up migrate-down

help: ## Show this help message
	@echo "Available commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

up: ## Start the Postgres database container in the background
	docker-compose up -d

down: ## Stop the database container and remove volumes
	docker-compose down -v

run: ## Run the backend application with environment variables
	cargo run

check: ## Fast compilation check
	cargo check

test: ## Run tests
	cargo test

fmt: ## Format the code automatically
	cargo fmt --all

lint: ## Run cargo clippy lints
	cargo clippy --all-targets -- -D warnings

migrate-up: ## Run database migrations (manual command)
	cargo run -p migration -- up

migrate-down: ## Rollback the last database migration
	cargo run -p migration -- down
