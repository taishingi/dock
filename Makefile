DOCKER:=docker buildx build
SEND:=docker push  

all:
	@$(DOCKER) -t otechdo/mariadb:latest mariadb
	@$(DOCKER) -t otechdo/postgresql:latest postgresql
	@$(DOCKER) -t otechdo/zuu:latest zuu
	@$(DOCKER) -t otechdo/rlang:stable rlang/stable
	@$(DOCKER) -t otechdo/rlang:beta rlang/beta
	@$(DOCKER) -t otechdo/rlang:nightly rlang/nightly
	@$(DOCKER) -t otechdo/glang:latest glang
	@$(DOCKER) -t otechdo/dlang:latest dlang
	@$(DOCKER) -t otechdo/clang:latest clang
	@$(DOCKER) -t otechdo/jlang:latest jlang
send: all
	tux push otechdo/mariadb:latest  otechdo/postgresql:latest  otechdo/zuu:latest  otechdo/rlang:stable  otechdo/rlang:beta  otechdo/rlang:nightly  otechdo/glang:latest otechdo/dlang:latest otechdo/clang:latest otechdo/jlang:latest
