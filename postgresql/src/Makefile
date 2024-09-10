DOCKER:=docker buildx build
SEND:=docker push  

all:
	@$(DOCKER) -t otechdo/mariadb:latest dock/mariadb
	@$(DOCKER) -t otechdo/postgresql:latest dock/postgresql
	@$(DOCKER) -t otechdo/sqlite:latest dock/sqlite
send: all
	@$(SEND) otechdo/mariadb:latest
	@$(SEND) otechdo/postgresql:latest
	@$(SEND) otechdo/sqlite:latest

run:
	@docker compose down .
	@docker compose pull .
	@docker compose up -d .
