package proxy


import (
    "net"
    "context"
    "io"
    "log"
    "errors"
)

type ClosedConn int

const (
    BUFF_SIZE = 1000
)

const (
    CLIENT_CLOSED ClosedConn = iota
    SERVER_CLOSED ClosedConn = iota
)


// created by listener -> receives ctx -> dial some upstream -> read/write from both ends until someone closes a connection or timeout -> finalize connection
func handleConnection(ctx context.Context, conn net.Conn,  bl LoadBalancer, maxRetries int) {
    defer conn.Close()

    upConn, err := bl.DialUpstream(ctx)
    if err != nil {
        log.Printf("Failed connecting to upstream for %v: %v", ctx, err)
        return
    }

    var streamer SocketStreammer
    streamer.Stream(ctx, conn, upConn)
}


type SocketStreammer struct {
    clt, srv net.Conn
    closed chan ClosedConn
}

func (ss *SocketStreammer) Stream(ctx context.Context, clt, srv net.Conn) {
    ss.closed = make(chan ClosedConn, 1)
    ss.clt = clt
    ss.srv = srv
    go ss.sktTerminator(ctx)
    go ss.streamData(ctx, clt, srv, CLIENT_CLOSED, SERVER_CLOSED)
    go ss.streamData(ctx, srv, clt, SERVER_CLOSED, CLIENT_CLOSED)
}

func (ss *SocketStreammer) streamData(ctx context.Context, reader io.Reader, writer io.Writer, rdId, wrId ClosedConn) {
    var buf, writeBuf []byte
    buf = make([]byte, BUFF_SIZE)

    var nrd, nwr int
    var err error

    for {
        if len(writeBuf) > 0 {
            nwr, err = writer.Write(writeBuf)
            log.Printf("Written: %v", nwr)
            writeBuf = writeBuf[nwr:]
            if err != nil {
                // return maybe?
                return
            }
            continue
        }

        nrd, err = reader.Read(buf)

        if nrd > 0 {
            log.Printf("Read: %v", nrd)
            writeBuf = buf[0:nrd]
            continue
        }

        if err != nil {
            log.Printf("error reading from %v: %v", reader, err)
            if errors.Is(err, io.EOF) {
                ss.closed <- rdId
                return
            }
            // should i continue or just quit?
            return
        }
    }
}

func (ss *SocketStreammer) sktTerminator(ctx context.Context) {
    select {
    case id := <- ss.closed:
        log.Printf("terminator received: %v", id)
        switch id {
        case CLIENT_CLOSED: ss.srv.Close()
        case SERVER_CLOSED: ss.clt.Close()
        }
    }
}
