package proxy

import (
	"context"
	"io"
	"log"
	"net"

	"example.com/bpf-proxy/pkg/balancer"
)

const (
	BUFF_SIZE = 10000
)

// Connection management options
type ConnectionOpts struct {
	// Max attempts to connect to an upstream server
	MaxRetries int
}

// Handle an incoming client connection
// Attempt to connect to an upstream using the given balancer and stream data
// between upstream and client sockets.
// Blocks until either connection ends or ctx is done
func handleConnection(ctx context.Context, clt *net.TCPConn, bl balancer.LoadBalancer, opts ConnectionOpts) {
	defer clt.Close()

	upstream, err := bl.DialUpstream(ctx, opts.MaxRetries)
	if err != nil {
		log.Printf("Failed connecting to upstream for %v: %v", ctx, err)
		return
	}
	defer upstream.Close()

	cltDone := make(chan struct{})
	go streamData(clt, upstream, cltDone)

	upDone := make(chan struct{})
	go streamData(upstream, clt, upDone)

	// block until a connection is closed or main ctx is done
	// closing connections will unblock reads and goroutines will exit
	select {
	case <-cltDone:
	case <-upDone:
	case <-ctx.Done():
	}
}

// Loops reading from Reader and writing to Writer
// When an error happens, close the sync channel and return
func streamData(reader io.Reader, writer io.Writer, done chan<- struct{}) {
	var nrd, nwr int
	var errRd, errWr error
	var buf, writeBuf []byte
	buf = make([]byte, BUFF_SIZE)

	for {
		nrd, errRd = reader.Read(buf)
		writeBuf = buf[0:nrd]

		if errRd != nil {
			log.Printf("error reading from %v: %v", reader, errRd)
		}

		if len(writeBuf) > 0 {
			nwr, errWr = writer.Write(writeBuf)
			writeBuf = writeBuf[nwr:]
			if errWr != nil {
				log.Printf("error writing to %v: %v", writer, errWr)
			}
		}

		if errRd != nil || errWr != nil {
			close(done)
			break
		}
	}
}
