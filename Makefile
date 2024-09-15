include .env.database

# 🔧 環境準備
.PHONY: setup setup-migration

setup:
	@cp .env.api api/src/.env
	@cp .env.database database/.env
	@echo "DATABASE_URL=mysql://$(MYSQL_ROOT_USER):$(MYSQL_ROOT_PASSWORD)@$(MYSQL_HOST):$(MYSQL_PORT)/$(MYSQL_DATABASE)" > import/.env

setup-migration:
	@docker compose up migration --build

# 🐳 コンテナ操作
.PHONY: setup api mysql

api:
	@docker compose exec api cargo watch -x run

mysql:
	@docker compose exec mysql mysql -u root -p$(MYSQL_ROOT_PASSWORD) $(MYSQL_DATABASE)

# 📦 Migration
.PHONY: migrate redo redoall revert revertall

migrate:
	@docker compose run --rm migration diesel migration run

redo:
	@docker compose run --rm migration diesel migration redo

redoall:
	@docker compose run --rm migration diesel migration redo --all

revert:
	@docker compose run --rm migration diesel migration revert

revertall:
	@docker compose run --rm migration diesel migration revert --all
