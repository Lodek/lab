CC= clang
CFLAGS= -O2 -target bpf -Wextra -Wall -g

.PHONY: build
build: pkg/proxy/proxy_dispatch.o
	go build -o build/proxy cmd/proxy/main.go

pkg/proxy/proxy_dispatch.o: bpf/proxy_dispatch.c
	$(CC) -I/usr/local/include $(CFLAGS) -c $(<) -o $(@)
