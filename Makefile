################################################################################
# 変数
################################################################################
HEROKU_APP_NAME := todo-actixweb

################################################################################
# タスク
################################################################################
.PHONY: heroku-login
heroku-login: ## heroku login
	heroku login --interactive

.PHONY: heroku-create-app
heroku-create-app: ## heroku create ${HEROKU_APP_NAME}
	(heroku apps | grep ${HEROKU_APP_NAME}) || heroku create ${HEROKU_APP_NAME}

.PHONY: heroku-container-login
heroku-container-login: ## heroku container:login
	heroku container:login

.PHONY: heroku-build
heroku-build: ## heroku用のtagを付けたDocker imageをbuild
	docker build . --tag registry.heroku.com/${HEROKU_APP_NAME}/web

.PHONY: heroku-push
heroku-push: ## heroku用のtagを付けたDocker imageをpush
	docker push registry.heroku.com/${HEROKU_APP_NAME}/web

.PHONY: heroku-release
heroku-release: ## heroku で release
	heroku container:release web

.PHONY: docker-build
docker-build: ## docker-compose build
	docker-compose build

.PHONY: up
up: ## docker-compose up
	docker-compose up



.PHONY: bash
bash: ## docker-compose run --rm --service-ports app bash
	docker-compose run --rm --service-ports app bash

.PHONY: chown
chown: ## sudo chown -R ${USER}:${USER} ./
	sudo chown -R ${USER}:${USER} ./

.PHONY: down
down: ## docker-compose down
	docker-compose down

.PHONY: deploy
deploy: ## docker-compose down
	git subtree push --prefix build/ origin gh-pages


################################################################################
# マクロ
################################################################################
# Makefileの中身を抽出してhelpとして1行で出す
# $(1): Makefile名
define help
  grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(1) \
  | grep --invert-match "## non-help" \
  | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'
endef

################################################################################
# タスク
################################################################################
.PHONY: help
help: ## Make タスク一覧
	@echo '######################################################################'
	@echo '# Makeタスク一覧'
	@echo '# $$ make XXX'
	@echo '# or'
	@echo '# $$ make XXX --dry-run'
	@echo '######################################################################'
	@echo $(MAKEFILE_LIST) \
	| tr ' ' '\n' \
	| xargs -I {included-makefile} $(call help,{included-makefile})
