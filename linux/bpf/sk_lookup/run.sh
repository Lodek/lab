#! /usr/bin/sh

# start netcat
# get netcat pid
# run c program in background
# cat kernel pipe for debug info
echo "starting netcat listener"
nc -kl 0.0.0.0 9999 &
echo "netcat pid: $!"

echo spawning C loader
build/bpfloader $! build/sk_lookup_kern.o &

echo "tapping on kernel trace pipe"
cat /sys/kernel/debug/tracing/trace_pipe
