package balancer

import (
	"context"
	"fmt"
	"log"
	"net"
	"sync"
)

// Interface for a Load Balancer which distributes connection
// requests to the set upstreams
// All method in LoadBalancer must be thread safe, as multiple
// connections will invoke it when they first are created.
type LoadBalancer interface {
	// Define upstream options for LoadBalancer instance.
	SetUpstreams(upstreams []string)

	// Return a Conenction to a load balanced upstream.
	// If no upstream is available or a connection error happens, return error
	DialUpstream(ctx context.Context, tries int) (net.Conn, error)
}

type NaiveRoundRobin struct {
	upstreams []string
	mut       sync.Mutex
	idx       int
}

func (b *NaiveRoundRobin) SetUpstreams(upstreams []string) {
	b.mut.Lock()
	defer b.mut.Unlock()

	b.upstreams = upstreams
	b.idx = 0
}

// Naive rounb robin balancing implementation.
// Simply iterates over upstreams and tries each in turn until
// the tries run out.
func (b *NaiveRoundRobin) DialUpstream(ctx context.Context, tries int) (net.Conn, error) {
	if tries < 1 {
		return nil, fmt.Errorf("num of trials must be at least one. Received %v", tries)
	}

	b.mut.Lock()
	defer b.mut.Unlock()

	errors := make([]error, tries)

	for i := 0; i < tries; i++ {
		b.idx = cycleIdx(b.idx, len(b.upstreams))
		upstream := b.upstreams[b.idx]

		dialer := new(net.Dialer)
		conn, err := dialer.DialContext(ctx, "tcp", upstream)
		if err != nil {
			log.Printf("Failed to establish connection for ctx=%v with upstream %v", ctx, upstream)
			errors = append(errors, err)
			continue
		}

		log.Printf("Establish connection for ctx=%v with upstream %v", ctx, upstream)
		return conn, nil
	}

	return nil, fmt.Errorf("could not connect to any upstreams after %v tries: %v", tries, errors)
}

func cycleIdx(idx, max int) int {
	idx++
	if idx == max {
		return 0
	} else {
		return idx
	}
}
