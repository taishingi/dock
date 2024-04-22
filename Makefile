DOCKER:=docker buildx build
SEND:=docker push  

all:
	@$(DOCKER) -t taishingi/zuu:latest  zuu
	@$(DOCKER) -t taishingi/rlang:stable rlang/stable 
	@$(DOCKER) -t taishingi/rlang:beta rlang/beta 
	@$(DOCKER) -t taishingi/rlang:nightly rlang/nightly 
	@$(DOCKER) -t taishingi/glang:latest glang 
	@$(DOCKER) -t taishingi/dlang:latest dlang 
	@$(DOCKER) -t taishingi/clang:latest clang
	@$(DOCKER) -t taishingi/jlang:latest jlang
send: all
	@$(SEND) taishingi/zuu:latest 
	@$(SEND) taishingi/rlang:stable 
	@$(SEND) taishingi/rlang:beta 
	@$(SEND) taishingi/rlang:nightly 
	@$(SEND) taishingi/glang:latest
	@$(SEND) taishingi/dlang:latest 
	@$(SEND) taishingi/clang:latest 
	@$(SEND) taishingi/jlang:latest 
