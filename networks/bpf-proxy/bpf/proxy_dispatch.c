#include <linux/bpf.h>
#include <linux/types.h>
#include <bpf/bpf_helpers.h>

#define PORT_SET_SIZE 1000

/* Most documentation seems to use the "maps" section to declare bpf maps.
 * That approach has been deprecated and substituted by ".maps".
 * See: https://github.com/libbpf/libbpf/issues/272
 */
struct {
    __uint(type, BPF_MAP_TYPE_SOCKMAP);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u64);
} sock_map SEC(".maps");


struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, PORT_SET_SIZE);
    __type(key, __u32);
    __type(value, __u8);
} ports_map SEC(".maps");


/* for some reason the Go ebpf lib requires this to have a /
 * see https://github.com/cilium/ebpf/blob/master/elf_reader.go#L1151
 */
SEC("sk_lookup/")
int dispatch(struct bpf_sk_lookup *ctx)
{
    __u32 skt_key = 0;
    __u16 port;
    __u8 *contains;
    struct bpf_sock *skt;
    long err;

    port = ctx->local_port;
    contains = bpf_map_lookup_elem(&ports_map, &port);
    if (!contains) {
#if (DEBUG)
        char fmt[] = "skipping: local port not in port set: port=%d";
        bpf_trace_printk(fmt, sizeof(fmt), ctx->local_port);
#endif
        return SK_PASS;
    }

    skt = bpf_map_lookup_elem(&sock_map, &skt_key);
    if (!skt) {
        // is there a better way to report errors in BPF?
        char fmt[] = "error: no socket in sockmap. skipping";
        bpf_trace_printk(fmt, sizeof(fmt));
        return SK_PASS;
    }

    err = bpf_sk_assign(ctx, skt, 0);
    if (err) {
        char fmt[] = "error: sk_assign failed local_port=%d err=%d";
        bpf_trace_printk(fmt, sizeof(fmt), ctx->local_port, err);
    }

#if (DEBUG)
    char fmt[] = "ok: assigned connection to skt port=%ld";
    bpf_trace_printk(fmt, sizeof(fmt), skt->dst_port, skt->protocol);
#endif

    bpf_sk_release(skt);

    return err ? SK_DROP : SK_PASS;
}

char __license[] SEC("license") = "GPL";
