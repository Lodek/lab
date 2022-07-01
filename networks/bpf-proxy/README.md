# About

A simple TCP proxy which uses a BPF `sk_lookup` program for connection steering


# Building and Running

The project provides a `Makefile` to build the proxy.

To build simply run 
```
make
```

The makefile builds the BPF program and places it inside the `proxy` go pkg.
The `pkg/proxy/bpf.go` file embeds the compiled elf object in itself through the `embed` package.

The generated `proxy` executable is placed in the `build` directory

To run do:
```
sudo build/proxy
```

Unforunately it needs sudo :(
I am unsure which files / permision are required in order to do bpf stuff as an unprivileged user.

# Notes

## A short summary of what you built, how it works, and how you landed on this design

This project implements a simple TCP proxy based on the given skeleton for configuration handling.

The design process was a bit intricate as this was my first time writing Go code.
I tried to balanced what I understood to be a good approach vs what the language readily offers.

The proxy uses go's native concurrency model to handle multiple connections, each connection spawns a goroutine.
My first intuition was to use an event loop / async model, which is well suited for the proxy's work load (IO bound).
This is the model used by nginx after all, a software I have some familiarity with.

It turns out Async isn't as prevalent on Go and upon researching a bit, even popular go web servers such as Caddy uses goroutines to handle connections.
As such, I dropped the async runtime idea and followed the well established approach.

The proxy works by spawning a single TCP socket in a code defined port number.
Connections to the proxy are forwarded to its socket through an eBPF program, which forwards the connection based on a set of known port numbers, the user "App" ports.

An incoming connection spawns a new goroutine and it streams data between an upstream server and the client.
The streamming continues until either end closes the connection.
TCP Keepalive probes were not implemented, which is something that should be reviewed.

The upstream sever is chosen through a load balancing interface, currently it implements a naive version of the round robin scheduling algorithm.
Round Robin was chosen due to its simplicity and familiarity, however the code was built around a `LoadBalancer` interface which would allow swapping strategies, even per user App if necessary.

Hot config reload is supported by making use of the given Watcher implementation.
Configuration updates are forwarded to the main Proxy through a channel.
The proxy tries to apply the configuration, in case of an error (such as bpf map update error), the old configuration is maintained.

Without extending I want to mentioned that I tried to be very mindful regarding the hot reload design.
I have experience of a scenario where nginx was used as the core of a edge computing platform, which is inheretly multi-tenant.
As the platform grew and the number of customers increased, the single biggest bottleneck for scailability was in Nginx's reload model.
Reloading ~40k server blocks - some of which container over 10k domains - was painful slow by the standards we aimed for, which was extremely wasteful given that only a handful of servers were updated.

I believe cloudflare ran into similar problems and their solution was to steer away from Nginx confs for a while and use more and more Lua code through OpenResty, but at the end they ended up rewriting their stuff.
To me that's a difficult problem to foresee and requires previous domain knowledge.

The current code is naive in that it recreates every config resource there is but by design, the config update function can be selective about what to update.


## How you might add hot config reloading that doesn't break existing connections if apps and targets change

Hot reloading currently works around the file watcher model.
A new instance of the configuration is sent through a Go channel to the proxy struct, where it performs a `select` between incoming connections or config updates.
The select should guarantee that no new connections are accepted while the configuration is updating.

As for existing connections, configuration updates do not mutate values.
Each goroutine will have a reference to the required resources it needs (eg a `LoadBalancer` reference), which will not be mutated.

After the connection is finished, Go's garbage collector should dispose of the orphan resources.


## What might break under a production load? What needs to happen before your proxy is production ready?

The following items are what I would consider standard engineering practices for production loads.
- implementing a minimal test suite
- metric scraping for platform monitoring
- limit test the ammount of concurrent connections it could handle

Issues specific for this project are the following.

TCP Keepalives must be reevaluated.
They are currently not implemented however would be extremely valuable to avoid half-open connections.
Managing various TCP connection patterns - each of which are particular to customer applications - while not compromising the integrity / performance of our platform seems like a tricky problem.
I frankly don't have an answer for that at the moment.
My best guess would be to have user defined TCP keep alive with a proxy hard cap.
That way there is flexibility while also maintaining an upper bound.
Definitely would need to brainstorm this with a team.

An important thing I would like to address is figuring out whether there's a way to make the proxy run without needing root permissions.
BPF seems to require it :/

Other than that, there are several FIXME and TODO comments in the code which wold be ideal to address before production.
Such as connection cancelation handling (timeouts / keepalive), some code literacy improvements and some other minor details.


## How would you make a global, clustered version of your proxy?

That's an interesting question.
In all honesty I think it would greatly depends on the envisioned features for this proxy.

It's my impression that an Anycast network should handle most of the heavy lifting with respect to high availability.

Assuming the proxy will only support TCP streaming (ie no need for proxy instances syncing or East-West traffic), the big unsolved problem left is configuration orchestration.
That is, how to propagate a config update to every edge server in a timely manner.

The first thought that comes to mind is a pub-sub messaging architecture, that would be one of the classic approach to this sort of problem.
The primary data storage would receive customer App updates and broadcast it to a queue where all edges would consume from it.

I can't help but toy with the thought of using Litestream for that as well.
But I don't know enough about it to know whether it would work very well :P
In theory it could be interesting, use its streaming functionalities to duplicate the configuration set to the various edge servers and consequently the proxy instances.
The point I am unsure about is whether it has a feature to notify about *which* changes it received.
This is important so the granular config update model can be enforced.


## What you did to add BPF steering

Bpf steering is done through a `sk_lookup` eBPF program.
The bpf prorgam design is quite similar to the example given in the bpf docs and the cloudflare presentation.

Basically a bpf program with two maps, one `SOCKMAP` for the target socket, another map which acts as a port number lookup table.
The logic is quite simple, the ebpf program responds to socket lookup events and checks on the lookup table whether that port number should be handled, if it should it uses a bpf helper to assign the connection to the socket.

In order to actually make use of the bpf program there are three elements:
- The Bpf program itself
- The maps
- The link / attach point

The BPF program is written in C and uses LLVM to compile to BPF's instruction set.
The BPF program declares the maps used in the `.maps` section.

The compiled bpf elf is loaded by the proxy using Go's ebpf lib.
The loading procress creates the BPF program and the maps.
During the proxy init process, the Go code opens the listening socket and sets a copy of its file descriptor into the `SOCKMAP` map and it attaches the bpf program to its own network namespace.

The port set is updated each time the proxy receives a new configuration.

Since there are no pins to the bpf file system, it means there is no possibility for bpf orphans to lie arround.
All file descriptors associated to bpf objects will be closed at proxy shutdown.
Avoiding pins means it's easier to avoid bugs such as the one in the [article](https://facebookmicrosites.github.io/bpf/blog/2018/08/31/object-lifetime.html).


## How you'd update the BPF maps when configuration changes

Port set updates can be done through the BPF call.
It will use the file descriptor for the BPF map and perform updates.

In line with my previous comment about making configuration updates fast and granular, I would keep a a copy of the set of monitored ports in the proxy process and perform a diff to get the changed ports.
This would allow updating the port set with only 2 syscalls:
- One `BPF_MAP_DELETE_BATCH` call to remove the ports which should no longer be handled
- One `BPF_BPF_MAP_UPDATE_BATCH` call to add all ports which should be handled.

Writing this I realized I forgot to do the first step of that operation in the proxy implementation :/
