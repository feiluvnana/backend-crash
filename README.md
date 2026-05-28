# Rust Backend Boilerplate

Production-ready, modular Rust backend boilerplate using Axum, SeaORM, and PostgreSQL.

## Features

- **Modular Architecture**: Features are organized under `src/features/` with isolated handlers, services, and DTOs.
- **Database Migrations**: Integrated database migrations powered by `SeaORM` and managed via `migration` package.
- **Granular Configs**: Fully environment-aware using dotenv and validation.
- **Request Trace ID**: UUID request tracing middleware propagates `x-request-id` headers for seamless trace log mapping.
- **Auto API documentation**: Interactive Swagger API docs automatically generated via `utoipa` at `/swagger-ui`.
- **Docker Ready**: Multi-stage docker builds producing minimal (~30MB) images.
- **Local CI Pipeline**: Validate code locally before checking in using `make ci`.

## Quick Start

### 1. Set Up Environment
Copy the example environment file to generate `.env`:
```bash
make g:env
```

### 2. Spin Up Database
Start PostgreSQL using Docker:
```bash
make docker:up
```

### 3. Run Migrations & Start Server
Run migrations and launch the dev server:
```bash
make db:up
make run
```

Access the API documentation at `http://localhost:3000/swagger-ui`.

---

## Makefile Grouped Commands

Run `make help` to list all commands. The main namespaces are:

- **App**: `make run`, `make check`, `make test`, `make fmt`, `make lint`, `make ci`
- **Docker**: `make docker:up`, `make docker:down`, `make docker:build`, `make docker:logs`
- **Database**: `make db:up`, `make db:down`
- **Generators**: `make g:env`, `make g:feature name=<name>`

---

## Folder Structure

```
src/
├── main.rs                    # Server bootstrap
├── core/                      # Global extractors, pagination, errors, and configs
├── db/                        # Database connectivity and models/entities
├── features/                  # Business modules (auth, user, health)
├── middleware/                # Route middlewares (auth, request ID)
├── routes/                    # Route register and API Docs configurations
├── utils/                     # Generic crypto and token utilities
└── bin/                       # Boilerplate code generators
```
