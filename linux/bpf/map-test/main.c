#include <linux/bpf.h>
#include <linux/types.h>
#include <bpf/bpf_helpers.h>


#ifndef __section
# define __section(NAME)                  \
   __attribute__((section(NAME), used))
#endif


__section("sk_lookup")
int bpf_sk_lookup(struct bpf_sk_lookup *ctx)
{
    char fmt[] = "sk_lookup hello: ip=%llu port=%llu";
    bpf_trace_printk(fmt, sizeof(fmt), ctx->local_ip4, ctx->local_port);
    return SK_PASS;
}

char __license[] __section("license") = "GPL";
