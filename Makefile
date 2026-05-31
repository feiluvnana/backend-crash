run:
	cargo watch -x run

db\:up:
	sea-orm-cli migrate -d database/migrations up

db\:down:
	sea-orm-cli migrate -d database/migrations down

db\:migration:
	sea-orm-cli migrate -d database/migrations generate $(name)

ifeq ($(OS),Windows_NT)
db\:entity:
	@powershell -NoProfile -Command "\
		if (Test-Path .env) { \
			$$env:DATABASE_URL = (Get-Content .env | Select-String '^DATABASE_URL=' | ForEach-Object { $$_.Line.Split('=', 2)[1].Trim('\"''') }); \
		} \
		if (-not $$env:DATABASE_URL) { \
			Write-Error 'DATABASE_URL is not set'; \
			exit 1; \
		} \
		Write-Host 'Generating entities from' $$env:DATABASE_URL '...'; \
		sea-orm-cli generate entity --database-url $$env:DATABASE_URL -o database/models \
	"

env\:setup:
	@if not exist .env (copy .env.example .env & echo Created .env from .env.example) else (echo .env already exists)
else
db\:entity:
	@if [ -f .env ]; then \
		export $$(grep -v '^#' .env | xargs); \
	fi; \
	if [ -z "$$DATABASE_URL" ]; then \
		echo "DATABASE_URL is not set"; \
		exit 1; \
	fi; \
	echo "Generating entities from $$DATABASE_URL..."; \
	sea-orm-cli generate entity --database-url "$$DATABASE_URL" -o database/models

env\:setup:
	@if [ ! -f .env ]; then \
		cp .env.example .env; \
		echo "Created .env from .env.example"; \
	else \
		echo ".env already exists"; \
	fi
endif
scaffold\:feature:
	cargo run --bin scaffold feature $(name)

scaffold\:middleware:
	cargo run --bin scaffold middleware $(name)

scaffold\:extractor:
	cargo run --bin scaffold extractor $(name)

docker\:build:
	docker compose build

docker\:up:
	docker compose up -d

docker\:down:
	docker compose down

docker\:logs:
	docker compose logs -f

format:
	cargo fmt

lint:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test

clean:
	cargo clean

.PHONY: run db\:up db\:down db\:migration db\:entity env\:setup scaffold\:feature scaffold\:middleware scaffold\:extractor docker\:build docker\:up docker\:down docker\:logs format lint test clean

