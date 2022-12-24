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

certs:
	@openssl req -x509 -newkey rsa:2048 -nodes -keyout certs/key.pem -out certs/cert.pem -days 365 -subj '/CN=localhost'

gen_curve:
	@openssl ecparam -name prime256v1 -genkey -noout -out certs/private.ec.key
	@openssl ec -in certs/private.ec.key -pubout -out certs/public-key.pem

.PHONY: migrate db db_ns down certs gen_curve
