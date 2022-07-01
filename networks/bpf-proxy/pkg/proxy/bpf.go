package proxy

import (
	"bytes"
	_ "embed"
	"fmt"
	"log"
	"net"
	"os"

	"github.com/cilium/ebpf"
	"github.com/cilium/ebpf/link"
)

//go:embed proxy_dispatch.o
var dispatcher_elf []byte

// Model entities contained within BPF elf file.
type ProxyDispatcherSpec struct {
	Dispatcher *ebpf.Program `ebpf:"dispatch"`
	Sockmap    *ebpf.Map     `ebpf:"sock_map"`
	Ports      *ebpf.Map     `ebpf:"ports_map"`
}

// Cleanups BPF program by closing links and associated maps / programs.
// This approach has the advantage of always avoiding bpf orphans left through
// bpf fs pins.
func (spec *ProxyDispatcherSpec) Close() {
	spec.Sockmap.Close()
	spec.Ports.Close()
	spec.Dispatcher.Close()
}

// Encapsulates operations to manage lifecycle of the BPF proxy dispatch program,
// including updating port set and target socket
type ProxyDispatcher struct {
	spec    ProxyDispatcherSpec
	link    *link.NetNsLink
	sktFile *os.File
}

// Loads dispatcher bpf program in the kernel.
func LoadDispatcher() (*ProxyDispatcher, error) {
	dispatcherSpec := ProxyDispatcherSpec{}

	reader := bytes.NewReader(dispatcher_elf[:])
	spec, err := ebpf.LoadCollectionSpecFromReader(reader)
	if err != nil {
		return nil, fmt.Errorf("could not load ebpf program: %w", err)
	}

	if err := spec.LoadAndAssign(&dispatcherSpec, nil); err != nil {
		return nil, fmt.Errorf("can't extract maps and prog from elf: %w", err)
	}

	dispatcher := &ProxyDispatcher{spec: dispatcherSpec}

	return dispatcher, nil
}

// Attach ProxyDispatcher to the current processe's network namespace
// with attach type BPF_SK_LOOKUP
func (pd *ProxyDispatcher) Attach() error {
	netns, err := os.Open("/proc/self/ns/net")
	if err != nil {
		return err
	}
	defer netns.Close()

	link, err := link.AttachNetNs(int(netns.Fd()), pd.spec.Dispatcher)
	if err != nil {
		return fmt.Errorf("could not crate dispatcher bpf link: %w", err)
	}

	pd.link = link
	return nil
}

// Set ProxyDispatcher map of port numbers with `ports`.
// Performs a batch update to minimize syscalls
func (pd *ProxyDispatcher) SetPorts(ports []int) error {
	opts := &ebpf.BatchOptions{
		ElemFlags: 0,
		Flags:     uint64(ebpf.UpdateAny),
	}

	// NOTE it's important that these types are compatible with the map definitinos
	// in the bpf program, otherwise BatchUpdate errors out.
	count := len(ports)
	zeroes := make([]uint8, count)
	castPorts := make([]uint32, count)

	for i, port := range ports {
		castPorts[i] = uint32(port)
	}

	// TODO should perform a batch delete call to remove the ports which should no longer be listened to.

	res, err := pd.spec.Ports.BatchUpdate(castPorts, zeroes, opts)
	if res < count {
		log.Printf("Failed to update all ports in map. expected %v result %v", count, res)
	}

	return err
}

// Retrives conn's file descriptor and stores it in ProxyDispatcher
// internal SOCKMAP ebpf map. Connection will be forward to the given socket
func (pd *ProxyDispatcher) SetSocket(conn *net.TCPListener) error {
	file, err := conn.File()
	if err != nil {
		return err
	}

	pd.sktFile = file

	var key uint32 = 0
	fd := uint64(file.Fd())

	if err = pd.spec.Sockmap.Update(key, fd, ebpf.UpdateAny); err != nil {
		return fmt.Errorf("could not update dispatcher sockmap: %w", err)
	}

	return nil
}

// Cleanups BPF program by closing links and associated maps / programs.
// This approach has the advantage of always avoiding bpf orphans left through
// bpf fs pins.
func (pd *ProxyDispatcher) Close() {
	if pd.link != nil {
		pd.link.Close()
	}

	pd.spec.Close()

	if pd.sktFile != nil {
		pd.sktFile.Close()
	}
}
