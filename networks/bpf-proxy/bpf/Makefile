CC= clang
CFLAGS= -O2 -target bpf -Wextra -Wall -g

proxy_dispatch.o: proxy_dispatch.c
	$(CC) -I/usr/local/include $(CFLAGS) -c $(<) -o $(@)

.PHONY: clean
clean: 
	rm *.o
