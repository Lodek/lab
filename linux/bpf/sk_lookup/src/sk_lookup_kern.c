#include <linux/bpf.h>
#include <linux/types.h>
#include <bpf/bpf_helpers.h>
#include <errno.h>

#define PORT_MIN 3000
#define PORT_MAX 5000


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


SEC("sk_lookup")
int bpf_sk_lookup(struct bpf_sk_lookup *ctx)
{
    char call_fmt[] = "sk lookup received event";
    bpf_trace_printk(call_fmt, sizeof(call_fmt));

    __u32 skt_key = 0;
    struct bpf_sock *skt;
    long err;

    if (!(PORT_MIN <= ctx->local_port  && ctx->local_port < PORT_MAX)) {
        char fmt[] = "skipping: local port not in range: port=%d";
        bpf_trace_printk(fmt, sizeof(fmt), ctx->local_port);
        return SK_PASS;
    }

    skt = bpf_map_lookup_elem(&sock_map, &skt_key);
    if (!skt) {
        char fmt[] = "no socket in sockmap, skipping";
        bpf_trace_printk(fmt, sizeof(fmt));
        return SK_PASS;
    }

    err = bpf_sk_assign(ctx, skt, 0);
    if (err) {
        char fmt[] = "sk_assign failed: local_port=%d errno=%d";
        bpf_trace_printk(fmt, sizeof(fmt), ctx->local_port, err);
    }

    char fmt[] = "ok: assigned connection to skt port=%ld";
    bpf_trace_printk(fmt, sizeof(fmt), skt->dst_port, skt->protocol);

    bpf_sk_release(skt);

    return err ? SK_DROP : SK_PASS;
}

char __license[] SEC("license") = "GPL";
