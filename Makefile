db:
	docker compose up db -d

db_ns:
	docker compose up db_ns -d

down:
	docker compose down

# op?=up
# migrate:
# 	docker run --network api_net --rm -v ${PWD}/db:/migrations \
# 		migrate/migrate -database "postgres://postgres:admin@db:5432/api?sslmode=require" -path /migrations $(op)

migrate:
	sqlx migrate run

.PHONY: migrate db db_ns down
