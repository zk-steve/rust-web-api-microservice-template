PKG_NAME=rust-api-server
BUILD_VERSION=$(shell git describe --long)
BUILD_RELEASE=$(shell git describe --tags --abbrev=0)


lint:
	cargo clippy --fix --allow-dirty --allow-staged

build:
	PKG_NAME=rust-api-server
	BUILD_VERSION=$(shell git describe --long)
	BUILD_RELEASE=$(shell git describe --tags --abbrev=0)
	BUILDKIT_PROGRESS=plain
	DOCKER_BUILDKIT=1
	docker build --ssh default -t $(PKG_NAME):$(BUILD_VERSION) --target=prod .

build-dev:
	BUILDKIT_PROGRESS=plain DOCKER_BUILDKIT=1 docker build --ssh default -t $(PKG_NAME):$(BUILD_VERSION) --target=dev .
