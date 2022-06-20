package proxy
/*
connectionStreamer
------------------
strimmer receives 2 connection objects and replays any data incoming from one connection into the other one

should respect cancelation requests
*/

// Reads from Connection until socket is exhausted, send read results through channel
func streamConn(ctx context.Context, ch chan error, readCon writeCon net.Conn) {
    buff := make([]byte, STREAM_BUFF_SIZE)
    for {
        // TODO prob should add a read timeout to this boy
        // TODO add ctx check in loop to check for ctx cancelation
        n, err := con.Read(buff)

        if err != nil && errors.Is(err, net.ErrClosed) {
            log.Println("reader is closed, closing pair: ctx=%v skt=%v", ctx, readCon)
            writeCon.Close()
            return
        }

        if err != nil {
            ch <- err
            return
        }

        if n == 0 {
            // FIXME actually not too sure how to go about this.
            // will a skt ever return 0 data read?
            log.Println("reading socket returned no data, closing it: ctx=%v skt=%v", ctx, readCon)
            readCon.Close()
            return
        }

        n, err := writeCon.Write(buff[0:n])

        if err != nil {
            ch <- err
            return
        }

        if err != nil && errors.Is(err, net.ErrClosed) {
            log.Println("writer is closed, closing self: ctx=%v skt=%v", ctx, readCon)
            readCon.Close()
            return
        }
        // NOTE Should I check written bytes and compare against read bytes?
    }
}
