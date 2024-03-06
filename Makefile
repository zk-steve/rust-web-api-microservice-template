POSTGRES_DIR="./src/adapter/src/repositories/postgres"
DATABASE_URL="postgres://postgres:changeme@127.0.0.1:5432/postgres"

PKG_NAME=rust-api-server
BUILD_VERSION=$(shell git describe --long)
BUILD_RELEASE=$(shell git describe --tags --abbrev=0)


lint:
	cargo clippy --fix --allow-dirty --allow-staged

setup-db:
	diesel setup --database-url ${DATABASE_URL} \
     --migration-dir ${POSTGRES_DIR}/migrations \
     --config-file ${POSTGRES_DIR}/diesel.toml

migrate:
	diesel migration run --database-url ${DATABASE_URL} \
     --migration-dir ${POSTGRES_DIR}/migrations \
     --config-file ${POSTGRES_DIR}/diesel.toml

migrate-redo:
	diesel migration redo --database-url ${DATABASE_URL} \
     --migration-dir ${POSTGRES_DIR}/migrations \
     --config-file ${POSTGRES_DIR}/diesel.toml

build:
	PKG_NAME=rust-api-server
	BUILD_VERSION=$(shell git describe --long)
	BUILD_RELEASE=$(shell git describe --tags --abbrev=0)
	BUILDKIT_PROGRESS=plain
	DOCKER_BUILDKIT=1
	docker build --ssh default -t $(PKG_NAME):$(BUILD_VERSION) --target=prod .

build-dev:
	BUILDKIT_PROGRESS=plain DOCKER_BUILDKIT=1 docker build --ssh default -t $(PKG_NAME):$(BUILD_VERSION) --target=dev .

profiling-public:
	CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph --root -- -c ./src/public/config/* -c ./deploy/local/custom.toml