
$(shell mkdir -p build || true)

.PHONY: all
all: bpfloader sk_lookup_kern.o

bpfloader: src/bpf_loader.c
	gcc src/bpf_loader.c -lbpf -o build/bpfloader

sk_lookup_kern.o: src/sk_lookup_kern.c
	clang -O2 -Wextra -Wall -target bpf -g -c src/sk_lookup_kern.c -o build/sk_lookup_kern.o

.PHONY: clean
clean:
	rm build/*
