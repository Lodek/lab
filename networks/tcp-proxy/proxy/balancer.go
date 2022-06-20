package proxy


import (
    "net"
    "context"
)


type LoadBalancer interface {
    // Define upstream options for LoadBalancer instance.
    SetUpstreams(upstreams []string)

    // Return a Conenction to a load balanced upstream.
    // If no upstream is available or a connection error happens, return error
    DialUpstream(ctx context.Context) (net.Conn, error)
}

type RandomLB struct {
    upstreams []string
}

func (lb *RandomLB) SetUpstreams(upstreams []string) {
    lb.upstreams = upstreams
}

func (lb *RandomLB) DialUpstream(ctx context.Context) (net.Conn, error) {
    //TODO implment something proper
    dialer := new(net.Dialer)
    conn, err := dialer.DialContext(ctx, "tcp", lb.upstreams[0])
    return conn, err
}
