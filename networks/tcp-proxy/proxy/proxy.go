package proxy


import (
    "context"
    "log"
    "example/tcp-proxy/config"
)


func NewProxy(conf *config.Proxy) (*Proxy, error) {
    proxy := new(Proxy)
    err := proxy.init(conf)
    if err != nil {
        return nil, err
    }
    return proxy, err
}

type Proxy struct {
    conf *config.Proxy
    listeners map[string]*Listener
    done chan struct{}
}

// Runs proxy by spawning listeners for virtual servers.
// Monitors confUpdate for new configurations, if a new config is sent, recreates listeners.
// Exit if ctx is Done
func (p *Proxy) Run(ctx context.Context, confUpdate chan *config.Proxy) {
    // initial listener spawn
    p.spawnListeners(ctx, p.done)

    for {
        select {
        case <-ctx.Done():
            log.Println("Proxy ctx Done. Cleaning up")
            // this is kinda redudant
            // listeners would pickup after themselves on a closed ctx
            close(p.done)
            return
        case conf := <-confUpdate:
            log.Println("Proxy conf update received")
            p.init(conf)
            p.spawnListeners(ctx, p.done)
        }
    }
}

// (re)initialize proxy by attempting to create all listeners from a config.
// function may be called more than once. Recalling it implies that old listeners
// will be killed and new ones will be created.
func (p *Proxy) init(conf *config.Proxy) error {
    // since proxy is reinitialized after a conf update, 
    // closing `done` before creating listeners guarantees that
    // the listeners goroutines won't leak.
    if p.done != nil {
        close(p.done)
    }

    p.done = make(chan struct{})
    p.conf = conf
    return p.createListeners()
}

// create listeners for Proxy's current config, return error
// exit on any listener creation error
func (p *Proxy) createListeners() error {
    p.listeners = make(map[string]*Listener, len(p.conf.Apps))

    for _, app := range p.conf.Apps {
        listener, err := NewListener(&app)
        if err != nil {
            log.Println("Failed to create Listener for app %v: %v", app, err)
            return err
        }

        p.listeners[app.Name] = listener
    }
    return nil
}

// spawn a goroutine to setup each listener as a server socket
func (p *Proxy) spawnListeners(ctx context.Context, done chan struct{}) {
    for app, listener := range p.listeners {
        log.Printf("Spawning listener: %v %v", app, listener)
        go listener.Listen(ctx, done)
    }
}
