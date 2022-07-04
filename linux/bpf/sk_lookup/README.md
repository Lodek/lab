# About

A test `sk_lookup` eBPF program and C runner.
This simple example toys with `sk_lookup` functionality to forward connections in the port range [3000, 5000) to a running process.

The connection steering is added to the running procress by storing the process' socket file descriptor in a BPF SOCKMAP.
The file descriptor is fetched from the running process using the `pidfd_getfd` syscall.


# Build

To build simply run:

```
make 
```

Build requires clang / llvm and a Linux version which suports `sk_lookup` eBPF programs.


# Use

To see it in action, start a process which listens in a port.
A simple example is netcat:

```
nc -kl 0.0.0.0 7777
```

Then execute the `bpfloader` passing the following arguments:
- bpf elf file with program to load
- pid of listener process
- socket fd number from running process.

For the netcat example, the socket file descriptor is 3.
As such, the command would be:

```
sudo build/bpfloader $NETCAT_PID sk_lookup_kern.o 3
```

Note that sudo is required in order to create bpf objects, unless your user has the correct permissions.

In order to test it, open a connection with the localhost on the open ports [3000-5000).

eg:

```
echo "hello" | nc -N -4 localhost 3500
```

Note that the connection was established and the result was received, even though netcat process was opened at port 7777.


# References
- https://www.kernel.org/doc/html/latest/bpf/prog_sk_lookup.html
