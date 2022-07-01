package main

import (
	"context"
	"log"
	"os"
	"os/signal"

	"example.com/bpf-proxy/pkg/config"
	"example.com/bpf-proxy/pkg/proxy"
)

func main() {
	var basePort = 10000
	ctx := newCancelableContext()

	cfgStore := config.NewConfigStore("./config.json")

	// watch for changes to the config
	ch, err := cfgStore.StartWatcher()
	if err != nil {
		log.Fatalln(err)
	}
	defer cfgStore.Close()

	initialConf, err := cfgStore.Read()
	if err != nil {
		log.Fatalf("Could not read initial conf: %v", err)
	}

	p, err := proxy.NewProxy(basePort, initialConf)
	if err != nil {
		log.Fatalf("Failed to initialize proxy: %v", err)
	}

	go p.Run(ctx, ch)

	<-ctx.Done()
}

// newCancelableContext returns a context that gets canceled by a SIGINT
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
