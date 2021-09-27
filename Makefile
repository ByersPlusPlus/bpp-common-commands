build:
	rustup component add rustfmt
	cargo build --release

docker-build:
	docker run --rm -v "$(shell pwd)":/usr/src/bpp-common-commands rust:1.54.0-slim-buster /bin/bash -c "cd /usr/src/bpp-common-commands && apt update && apt install make -y && make"