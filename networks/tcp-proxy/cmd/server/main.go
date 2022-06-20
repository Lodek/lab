package main

import (
	"context"
	"log"
	"os"
	"os/signal"
        "example/tcp-proxy/config"
        "example/tcp-proxy/proxy"
)

func main() {
    conf := defaultConf()
    ctx := newCancelableContext()
    confCh := make(chan *config.Proxy)

    proxy, err := proxy.NewProxy(&conf)
    if err != nil {
        log.Fatalln("error creating main proxy: %v", err)
    }

    proxy.Run(ctx, confCh)

    <-ctx.Done()
}


func newCancelableContext() context.Context {
	doneCh := make(chan os.Signal, 1)
	signal.Notify(doneCh, os.Interrupt)

	ctx := context.Background()
	ctx, cancel := context.WithCancel(ctx)

	go func() {
		<-doneCh
		log.Println("signal recieved")
		cancel()
	}()

	return ctx
}

func defaultConf() config.Proxy {
    server5k := config.Application {
        Name: "5k fly",
        Ports: []int{5000},
        Origins: []string {"fly.io:443"},
    }

    server4k := config.Application {
        Name: "4k",
        Ports: []int{4000},
        Origins: []string {"httpbin.org:80"},
    }

    return config.Proxy {
        Apps: []config.Application {server5k, server4k},
    }
}
