.PHONY: run build test clippy migrate-up migrate-info compose-up help

help:
	@echo "run            CONFIG_INI=config.ini cargo run"
	@echo "build          release binary under target/release/"
	@echo "test / clippy  quality"
	@echo "compose-up     docker compose (Postgres, Redis, app) from project root"
	@echo "migrate-up     sqlx migrate run (sqlx-cli + DATABASE_URL matching config.ini)"
	@echo "migrate-info   sqlx migrate info"

run:
	cargo run

build:
	cargo build --release

test:
	cargo test

clippy:
	cargo clippy --all-targets -- -D warnings

migrate-up:
	sqlx migrate run

migrate-info:
	sqlx migrate info

compose-up:
	docker compose -f docker/docker-compose.yml up --build
