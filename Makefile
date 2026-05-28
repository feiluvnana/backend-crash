.PHONY: help setup run check test fmt lint \
        docker\:up docker\:down docker\:build docker\:logs \
        db\:up db\:down \
        g\:env g\:feature \
        ci

help: ## Show this help message
	@echo "Available commands:"
	@grep -E '^[a-zA-Z_:.-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

setup: g\:env ## Bootstrap environment and install cargo-watch
	cargo install cargo-watch

# ─── App ──────────────────────────────────────────
run: ## Run the backend application with hot-reloading
	cargo watch -x run

check: ## Fast compilation check
	cargo check

test: ## Run all tests
	cargo test

fmt: ## Format code
	cargo fmt --all

lint: ## Run clippy lints
	cargo clippy --all-targets -- -D warnings

ci: ## Run full CI pipeline locally (fmt + lint + test)
	cargo fmt --all -- --check
	cargo clippy --all-targets -- -D warnings
	cargo test

# ─── Docker ───────────────────────────────────────
docker\:up: ## Start all containers (app + postgres)
	docker compose up -d

docker\:down: ## Stop all containers and remove volumes
	docker compose down -v

docker\:build: ## Build the Docker image
	docker compose build

docker\:logs: ## Tail container logs
	docker compose logs -f

# ─── Database ─────────────────────────────────────
db\:up: ## Run database migrations
	cargo run -p migration -- up

db\:down: ## Rollback the last migration
	cargo run -p migration -- down

# ─── Generators ───────────────────────────────────
g\:env: ## Generate .env from .env.example
	@cp .env.example .env
	@echo "Created .env from .env.example"

g\:feature: ## Generate a new feature module (usage: make g:feature name=xxx)
	cargo run -p generator -- $(name)
