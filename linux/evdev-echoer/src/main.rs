/// Simple libevedev event echoer.
/// Reads all events from the specified device and echo it them to stdout.
/// Uses naive file handling approach, with busy loop.

use std::env::args;
use std::fs::File;

use evdev_rs::{Device, DeviceWrapper, ReadFlag};


fn main() {
    let mut args_iter = args();
    args_iter.next();
    let dev_file = args_iter.next().expect("Supply event device as arg");
    let file = File::open(dev_file).unwrap();
    let dev = Device::new_from_file(file).unwrap();

    loop {
        if dev.has_event_pending() {
            let (_, event) = dev.next_event(ReadFlag::NORMAL).unwrap();
            println!("Event: {:?}", event);
        }
    }
}
