.PHONY: build all run

all: build run

build:
	cd lib-rs && cargo build --release

run:
	luajit main.lua
