SHELL := bash

.PHONY: setup format lint test build

setup:
	./scripts/setup.sh

format:
	./scripts/format.sh

lint:
	./scripts/lint.sh

test:
	./scripts/test.sh

build:
	./scripts/build.sh
