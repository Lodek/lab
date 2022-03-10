# evdev-emitter

Rust binary to spawn keyboard events from user space.
Uses Linux's `uinput` module thorugh `libevdev` to create a virtual event device.
After crating the virtual device, it loops infinetely pressing then releasing space bar.


## Running
Use Rust's `cargo` to build the project.

```
cargo build
cargo run
```

Sometimes it might be necessary to run the binary as sudo.
The executable is located in `./target/debug/evdev-emitter`.


## References
- https://docs.rs/evdev-rs/0.5.0/evdev_rs/
- https://www.freedesktop.org/software/libevdev/doc/latest/index.html
- https://www.kernel.org/doc/html/v4.12/input/uinput.html
- https://www.kernel.org/doc/html/v4.12/input/input.html
- https://www.kernel.org/doc/html/v4.12/input/event-codes.html
