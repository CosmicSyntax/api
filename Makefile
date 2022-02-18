db:
	docker compose up db -d

op?=up
migrate:
	docker run --network api_net --rm -v ${PWD}/db:/migrations \
		migrate/migrate -database "postgres://postgres:admin@db:5432/api?sslmode=require" -path /migrations $(op)

.PHONY: migrate db
