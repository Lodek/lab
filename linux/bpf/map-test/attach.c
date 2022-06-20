
#include <stdio.h>
#include <stdlib.h>
#include <linux/bpf.h>
#include <linux/types.h>
#include <fcntl.h>
#include <sys/syscall.h>
#include <unistd.h>



int bpf(enum bpf_cmd cmd, union bpf_attr *attr, unsigned int size)
{
    return syscall(__NR_bpf, cmd, attr, size);
}


char *bpf_prog = "/sys/fs/bpf/sk-helloo";
char *netns_path = "/var/run/docker/373e3f17eb5f";

int main() {

    /* Open Get loaded BPF program FD */
    union bpf_attr attr = {

        .pathname = (__aligned_u64) bpf_prog,
        .bpf_fd = 0,
        .file_flags = 0,
    };
    int bfd = bpf(BPF_OBJ_GET, &attr, sizeof(attr));
    if (bfd < 0) {
        printf("failed to get object");
        exit(1);
    }


    /* Open NetNs file */
    int nsfd = open("/var/run/docker/netns/373e3f17eb5f", 0);
    if (nsfd < 0) {
        printf("failed opening netns");
        exit(1);
    }

    /* Attach BPF prog */
    union bpf_attr attach_attr = {
        .link_create = {
            .prog_fd = bfd,
            .target_fd = nsfd,
            .attach_type = BPF_SK_LOOKUP,
            .flags = 0
        }
    };
    int attach_fd = bpf(BPF_LINK_CREATE, &attach_attr, sizeof(attach_attr));
    if (attach_fd < 0) {
        printf("failed attaching");
        exit(1);
    }

    for (;;) {
        sleep(1);
    }

}
