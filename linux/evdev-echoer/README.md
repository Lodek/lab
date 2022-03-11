# evdev-echoer
Simple libevedev event echoer in Rust.
Reads all events from the specified device and echo it them to stdout.
Uses naive file handling approach, with busy loop.

Event devices are usually under `/dev/input/event*`.


## Building and running
Pure Rust package, use `cargo build` to build.
Build target will be under `target/debug`.

Run as sudo to open event device.
Event device must be specified as first command line argument.


## References
- https://www.freedesktop.org/software/libevdev/doc/latest/syn_dropped.html
- https://www.freedesktop.org/software/libevdev/doc/latest/group__events.html
- https://docs.rs/evdev-rs/0.5.0/evdev_rs/index.html
