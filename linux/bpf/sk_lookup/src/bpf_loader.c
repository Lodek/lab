#include <stdio.h>
#include <stdlib.h>
#include <linux/bpf.h>
#include <linux/bpf.h>
#include <linux/types.h>
#include <bpf/libbpf.h>
#include <sys/syscall.h>
#include <errno.h>
#include <error.h>
#include <unistd.h>
#include <fcntl.h>

#include <sys/socket.h>


static inline int bpf(enum bpf_cmd cmd, union bpf_attr *attr, unsigned int size)
{
        return syscall(__NR_bpf, cmd, attr, size);
}

static inline int pidfd_open(pid_t target_pid, unsigned int flags)
{
	return syscall(__NR_pidfd_open, target_pid, flags);
}

static inline int pidfd_getfd(int pidfd, int targetfd, unsigned int flags)
{
	return syscall(__NR_pidfd_getfd, pidfd, targetfd, flags);
}


#define BPF_MAP_NAME "sock_map"
#define BPF_PROG_NAME "bpf_sk_lookup"

int main(int argc, char **argv) {
    if (argc != 4) {
        fprintf(stderr, "Usage: <netcat_pid> <bpf_obj_file> <socket_fd>");
        exit(EXIT_FAILURE);
    }

    int target_pid = atoi(argv[1]);
    char *bpf_obj_path = argv[2];
    int target_fd = atoi(argv[3]);

    /* Open netcat process pid and fetches copy of fd 
     * fd should be a socket which will be added to sockmap.
     */
    int pid_fd = pidfd_open(target_pid, 0);
    if (pid_fd < 0) {
        error(EXIT_FAILURE, errno, "pidfd_open failed pid=%d: ", target_pid);
    }

    int skt_fd = pidfd_getfd(pid_fd, target_fd, 0);
    if (skt_fd < 0) {
        error(EXIT_FAILURE, errno, "pidfd_getfd failed fd=%d: ", target_fd);
    }


    /* Open BPF obj file and load bpf prog with given name */
    struct bpf_object *bpf_elf_obj = bpf_object__open_file(bpf_obj_path, NULL);
    if (!bpf_elf_obj) {
        error(EXIT_FAILURE, errno, "failed opening bpf prog %s: ", bpf_obj_path);
    }

    struct bpf_program *prog = bpf_object__find_program_by_name(bpf_elf_obj, BPF_PROG_NAME);
    if (!prog) {
        error(EXIT_FAILURE, errno, "failed locating bpf prog %s: ", BPF_PROG_NAME);
    }

    if (bpf_object__load(bpf_elf_obj)) {
        error(EXIT_FAILURE, errno, "failed loading bpf prog %s: ", BPF_PROG_NAME);
    }

    /* get sockmap for loaded prog */
    struct bpf_map *sockmap = bpf_object__find_map_by_name(bpf_elf_obj, BPF_MAP_NAME);
    if (!sockmap) {
        error(EXIT_FAILURE, errno, "failed fetching map from bpf prog %s: ", BPF_MAP_NAME);
    }

    /* set skt into bpf sockmap */
    __u32 key = 0;
    int r = bpf_map__update_elem(sockmap, &key, sizeof(__u32), (void*) &skt_fd, sizeof(__u64), BPF_ANY);
    if (r) {
        error(EXIT_FAILURE, errno, "failed setting socket into bpf map: ");
    }

    int netns_fd = open("/proc/self/ns/net", 0);
    if (netns_fd < 0) {
        error(EXIT_FAILURE, errno, "failed opening netns: ");
    }

    /* attach bpf prog to self net ns */
    /* ie create a link */
    union bpf_attr attr;
    memset(&attr, 0, sizeof(attr));
    attr.link_create.prog_fd = bpf_program__fd(prog);
    attr.link_create.target_fd = netns_fd;
    attr.link_create.attach_type = BPF_SK_LOOKUP;
    attr.link_create.flags = 0;

    int attach_fd = bpf(BPF_LINK_CREATE, &attr, sizeof(attr));
    if (attach_fd < 0) {
        error(EXIT_FAILURE, errno, "failed attaching to netns: ");
    }

    // since the bpf program, map and link weren't pinned to the bpf fs
    // once the process exit these entities will be killed.
    for (;;) {
        getc(stdin);
    }
}
