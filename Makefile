build:
	docker-compose build

db:
	docker-compose up

test:
	cargo test

fmt:
	cargo fmt

dev:
	sqlx db create
	sqlx migrate run
	cargo watch -x run
