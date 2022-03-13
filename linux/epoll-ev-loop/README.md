# Epoll Event Loop
Files monitor using `epoll`.
Files given as arguments are monitored for a readable state.

The code sets up an epoll instance, opens each file and monitors them until any file is readable.
The implementation is a simple event loop that blocks with an `epoll_wait` call until a file is ready.
The event loop simply reads the file data into a buffer and logs the file and how many bytes were read.


## Build and run
Rust based project using `cargo` as the package manager.
Install cargo, run `cargo build` to build or `cargo run` to run.

The executable receives list of files as arguments to monitor.

An easy way to see the event loop in action is to use event files (`/dev/input/event*` files), such as the keyboard or mouse files.
To identify the devices, try [evtest](https://man.archlinux.org/man/evtest.1).
Note: input files can only be read as root root or if the user is in the `input` group.

FIFOs can also be setup to work, it requires some more intricate setup though.


## References
- https://man7.org/linux/man-pages/man7/epoll.7.html
- [The Linux Programming Interface](https://man7.org/tlpi/), 63.4

