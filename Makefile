.PHONY: help setup run check test fmt lint \
        docker\:up docker\:down docker\:build docker\:logs \
        db\:up db\:down \
        g\:env g\:feature \
        ci

help: ## Show this help message
	@echo "Available commands:"
	@grep -E '^[a-zA-Z_:.-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

setup: ## Bootstrap environment and install cargo-watch
	$(MAKE) g\:env
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
db\:up: ## Run database migrations via sea-orm-cli
	sea-orm-cli migrate -d db/migrations up

db\:down: ## Rollback the last migration via sea-orm-cli
	sea-orm-cli migrate -d db/migrations down

db\:migration: ## Create a new migration (usage: make db:migration name=xxx)
	@if [ -z "$(name)" ]; then echo "Error: name is required. Usage: make db:migration name=xxx"; exit 1; fi
	sea-orm-cli migrate -d db/migrations generate $(name)

db\:entity: ## Generate entity models from database
	@if [ ! -f .env ]; then \
		echo "Error: .env file not found"; \
		exit 1; \
	fi; \
	if grep -q -E '^DATABASE_URL=' .env; then \
		DATABASE_URL=$$(grep -E '^DATABASE_URL=' .env | cut -d'=' -f2- | tr -d '\r\n"'); \
	else \
		USER=$$(grep -E '^POSTGRES_USER=' .env | cut -d'=' -f2- | tr -d '\r\n"'); \
		PASSWORD=$$(grep -E '^POSTGRES_PASSWORD=' .env | cut -d'=' -f2- | tr -d '\r\n"'); \
		HOST=$$(grep -E '^POSTGRES_HOST=' .env | cut -d'=' -f2- | tr -d '\r\n"'); \
		PORT=$$(grep -E '^POSTGRES_PORT=' .env | cut -d'=' -f2- | tr -d '\r\n"'); \
		DB=$$(grep -E '^POSTGRES_DB=' .env | cut -d'=' -f2- | tr -d '\r\n"'); \
		DATABASE_URL="postgres://$$USER:$$PASSWORD@$$HOST:$$PORT/$$DB"; \
	fi; \
	echo "Generating entities..."; \
	sea-orm-cli generate entity --database-url "$$DATABASE_URL" -o db/models

# ─── Generators ───────────────────────────────────
g\:env: ## Generate .env from .env.example
	@cp .env.example .env
	@echo "Created .env from .env.example"

g\:feature: ## Generate a new feature module (usage: make g:feature name=xxx)
	cargo run -p g -- $(name)
