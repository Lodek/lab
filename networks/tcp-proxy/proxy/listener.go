package proxy

import (
    "net"
    "time"
    "context"
    "log"
    "example/tcp-proxy/config"
)

var (
    connKey = ctxKey { name: "conn" }
    idKey = ctxKey { name: "appId" }
    startKey = ctxKey { name: "startTime" }
)


type ctxKey struct {
    name string
}


func NewListener(c *config.Application) (*Listener, error) {
    skts := make([]net.Listener, 0, len(c.Ports))

    for _, port := range c.Ports {
        addr := new(net.TCPAddr)
        addr.Port = port
        // TODO should open the skt with SO_REUSEADDR
        skt, err := net.ListenTCP("tcp", addr)
        if err != nil {
            return nil, err
        }
        skts = append(skts, skt)
    }

    listener :=  new(Listener)
    listener.conf = c
    listener.skts = skts
    listener.conns = make(chan net.Conn)
    listener.balancer = new(RandomLB)
    listener.balancer.SetUpstreams(listener.conf.Origins)

    return listener, nil
}


type Listener struct {
    conns chan net.Conn
    conf *config.Application
    skts []net.Listener
    balancer LoadBalancer
}

// Loops listening for incoming connections in Listener.
// Responds to context cancelation events and a close on `done`
func (l *Listener) Listen(ctx context.Context, done chan struct{}) {
    for _, skt := range l.skts {
        go accepter(skt, l.conns)
    }

    for {
        select {
        case conn := <-l.conns:
            // TODO start request context with timeout directive
            connCtx := context.WithValue(ctx, connKey, conn)
            connCtx = context.WithValue(ctx, startKey, time.Now())
            go handleConnection(connCtx, conn, l.balancer, 10)
        case <-ctx.Done():
            l.cleanup()
            return
        case <-done:
            l.cleanup()
            return
        }
    }
}

func (l *Listener) cleanup() {
    for _, skt := range l.skts {
        skt.Close()
    }
}


// Loops accepting connections in a Listener.
// Accepted connections are sent through `tx`.
// Exit once Listener is closed.
func accepter(skt net.Listener, tx chan net.Conn) {
    for {
        conn, err := skt.Accept()
        if err != nil {
            log.Printf("Connection Accept failed: %v", err);
            // FIXME figure out how to check for EINVAL err
            if err != nil {
                log.Printf("Listening skt was closed. Exiting Listener: %v", err)
                return
            }
            continue
        }
        log.Printf("Accepted connection from client: %v", conn)
        tx <- conn
    }
}
