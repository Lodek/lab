package proxy

import (
	"context"
	"fmt"
	"log"
	"net"
	"strconv"
	"time"

	"example.com/bpf-proxy/pkg/balancer"
	"example.com/bpf-proxy/pkg/config"
)

var (
	connKey  = ctxKey{name: "conn"}
	idKey    = ctxKey{name: "appId"}
	startKey = ctxKey{name: "startTime"}
)

const (
	maxRetriesConn int = 10
)

type ctxKey struct {
	name string
}

type Proxy struct {
	conf       config.Config
	listener   *net.TCPListener
	balancers  map[int]balancer.LoadBalancer
	dispatcher *ProxyDispatcher
	conns      chan *net.TCPConn
}

// Initialize Proxy by creating a TCP listener socket, loading the BPF socket
// steering program and storing the socket's fd in sockmap
func NewProxy(listenPort int, conf config.Config) (*Proxy, error) {
	proxy := &Proxy{
		balancers: make(map[int]balancer.LoadBalancer),
		conns:     make(chan *net.TCPConn),
	}

	// FIXME should probably set SO_REUSEADDR just to be safe
	addr := new(net.TCPAddr)
	addr.Port = listenPort
	skt, err := net.ListenTCP("tcp", addr)
	if err != nil {
		return nil, err
	}

	// load and initialize bpf program with target socket.
	// finally attaches it to netns
	dispatcher, err := LoadDispatcher()
	if err != nil {
		skt.Close()
		return nil, fmt.Errorf("Failed loading dispatcher: %w", err)
	}

	if err := dispatcher.SetSocket(skt); err != nil {
		skt.Close()
		dispatcher.Close()
		return nil, fmt.Errorf("Failed setting dispatcher skt: %w", err)
	}

	if err := dispatcher.Attach(); err != nil {
		skt.Close()
		dispatcher.Close()
		return nil, fmt.Errorf("Failed creating dispatcher link: %w", err)
	}

	proxy.listener = skt
	proxy.dispatcher = dispatcher

	// applies initial configuration to proxy
	// updates bpf ports hashmap
	if err := proxy.updateConf(conf); err != nil {
		proxy.Close()
		return nil, err
	}

	return proxy, nil
}

// Runs proxy by spawning listeners for virtual servers.
// Monitors confUpdate for new configurations, if a new config is sent, recreates listeners.
// Exit if ctx is Done
func (p *Proxy) Run(ctx context.Context, confUpdate <-chan config.Config) {
	defer p.Close()

	// Receive conns through channel to avoid race conditions
	// while updating conf.
	// The proxy will either dispatch a new connection or update the configuration.
	go accepter(p.listener, p.conns)

	for {
		select {
		case conn := <-p.conns:
			p.dispatchConnection(ctx, conn)
		case <-ctx.Done():
			log.Println("Proxy ctx Done. Cleaning up")
			return
		case conf := <-confUpdate:
			log.Println("Proxy conf update received")
			if err := p.updateConf(conf); err != nil {
				log.Printf("Failed updating proxy config, old configuration maintained: %v", err)
			}
		}
	}
}

func (p *Proxy) dispatchConnection(ctx context.Context, conn *net.TCPConn) {
	addr := conn.LocalAddr().String()
	// FIXME add proper error handling for the unlikely scenario of a weird local addr
	_, portStr, _ := net.SplitHostPort(addr)
	port, _ := strconv.Atoi(portStr)

	balancer, ok := p.balancers[port]
	if !ok {
		// FIXME technically this is big bad beucase the defaultListen port won't have
		// a balancer target
		// Maybe I could always drop packets to the default port in eBPF which would avoid this.
		// Although it makes sense for the proxy to have an admin http interface.
		// Just need to refine the requirements and the proxy configuration settings.
		log.Printf("Invariant error: BPF steered connection to unhandled port %v. Aborting connection.", port)
		defer conn.Close()
		return
	}

	// TODO start request context with timeout directive
	connCtx := context.WithValue(ctx, connKey, conn)
	connCtx = context.WithValue(ctx, startKey, time.Now())

	// this should probably come from the config itself
	opts := ConnectionOpts{
		MaxRetries: maxRetriesConn,
	}

	go handleConnection(connCtx, conn, balancer, opts)
}

// Attempts to update the proxy instance state to match the new configuration
// If state update fails, maintains last valid proxy state.
func (p *Proxy) updateConf(conf config.Config) error {
	// FIXME A better implementation would recreate balancers for only affected targets
	balancers := make(map[int]balancer.LoadBalancer)
	ports := []int{}

	for _, app := range conf.Apps {
		ports = append(ports, app.Ports...)
		lb := new(balancer.NaiveRoundRobin)
		lb.SetUpstreams(app.Targets)
		for _, port := range app.Ports {
			balancers[port] = lb
		}
	}

	if err := p.dispatcher.SetPorts(ports); err != nil {
		return fmt.Errorf("Failed updating bpf port map: %w", err)
	}

	p.balancers = balancers
	p.conf = conf

	return nil
}

// Cleanup
func (p *Proxy) Close() {
	p.listener.Close()
	p.dispatcher.Close()
}

// Loops accepting connections in a Listener.
// Accepted connections are sent through `tx`.
// Exit once socket is closed.
func accepter(skt *net.TCPListener, tx chan<- *net.TCPConn) {
	log.Println("Proxy waiting for connections")
	for {
		conn, err := skt.AcceptTCP()
		if err != nil {
			// FIXME Technically I should check for EINVAL in specific here
			// which would indicate the socket has been closed
			log.Printf("Accept failed: %v", err)
			return
		}

		log.Printf("Accepted connection from client: %v", conn)
		tx <- conn
	}
}
