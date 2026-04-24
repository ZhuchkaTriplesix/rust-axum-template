# rust-axum-template

Production-style **[Axum](https://github.com/tokio-rs/axum)** service layout with **Domain-Driven Design (DDD) layering** (compare with [go-gin-template](https://github.com/Reei-dp/go-gin-template) and [fastapi-template](https://github.com/Reei-dp/fastapi-template)): `config.ini`, health check (Postgres + Redis), OpenAPI + Swagger UI, optional Basic auth for docs, **SQLx** migrations, Docker Compose.

## DDD / hexagonal map

| Layer | Path | Role |
|--------|------|------|
| **API** (infrastructure entry) | `src/api` | HTTP routes, auth for `/api/docs`, DTO/JSON, Axum `Router` |
| **Application** | `src/application` | Use cases / application services; orchestrate domain + ports |
| **Domain** | `src/domain` | Entities, value objects, `DomainError`, **ports** (traits) |
| **Infrastructure** | `src/infrastructure` | Postgres (SQLx), Redis, adapters implementing domain ports |
| **Config** | `src/config.rs` | `config.ini` (and `CONFIG_INI` / `CONFIG_PATH` override) |

Add new bounded context under `src/domain/<name>/`, a repository **port** in `domain/ports/`, an application service, an adapter in `infrastructure/`, and routes under `src/api/`.

## Quick start (local)

1. Copy `config.ini.example` to `config.ini` and set `[POSTGRES]`, `[REDIS]`, and optionally `[DOCS]` for Basic-protected docs.
2. Start Postgres and Redis, then run `cargo run` (default port `8000`).
3. **Migrations** run automatically on boot unless `SKIP_MIGRATIONS=1`.

Endpoints:

- `GET /` → redirect to `/api/docs`
- `GET /api/root/health` — JSON (`database` / `redis` + HTTP 200 vs 503)
- `GET /api/root/welcome` — example domain + application use case
- `GET /api/docs` — Swagger UI (from CDN, loads `openapi.json` below)
- `GET /api/openapi.json` — embedded `assets/openapi.json`

## Docker

From the **repository root**:

```bash
make compose-up
# or: docker compose -f docker/docker-compose.yml up --build
```

`docker/config.docker.ini` is baked into the image; `migrations/` can be bind-mounted for local iteration.

## Makefile

- `make run` / `make build` / `make test` / `make clippy`
- `make compose-up` — full stack
- `make migrate-up` — **requires** [sqlx-cli](https://github.com/launchbadge/sqlx) and `DATABASE_URL` matching your `config.ini` DSN

## License

MIT
