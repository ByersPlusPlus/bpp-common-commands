# ByersPlusPlus Common Commands

These are a default set of commands for ByersPlusPlus!

## How to compile this?

Compiling a command library itself is pretty straight forward.

If you compiled [commandservice](https://github.com/ByersPlusPlus/commandservice) on your machine without Docker, you can simply run `make`.

If you intend to use the Docker container for commandservice, please run `make docker-build`, else the commandservice might not be able to load your command library due to differing glibc versions.
