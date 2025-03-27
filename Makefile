include .env
export $(shell sed 's/=.*//' .env)

export COMPOSE_DOCKER_CLI_BUILD=1
export DOCKER_BUILDKIT=1
export path=$(PWD)
EXPORT = export RUSTPATH=$(PWD)

test:
	$(EXPORT) && cargo test -- --test-threads 1 --nocapture

migration:
	$(EXPORT) && sqlx migrate add -r ${title}

migration-prepare:
	$(EXPORT) && cargo sqlx prepare

upgrade:
	$(EXPORT) && sqlx migrate run --database-url $(DATABASE_URL)

downgrade:
	$(EXPORT) && sqlx migrate revert --database-url $(DATABASE_URL)

checks:
	$(EXPORT) && cargo fmt
	$(EXPORT) && cargo clippy
	$(EXPORT) && cargo sqlx prepare --check

delete-git-branch-except-main:
	git branch | grep -v "main" | xargs git branch -D
