use std::env::args;
use std::io::Read;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::ffi::{CString, CStr};
use std::time::{SystemTime};
use std::os::unix::io::{AsRawFd, RawFd};

use libc::{epoll_create1, epoll_ctl, epoll_wait, EPOLL_CTL_ADD, EPOLLIN, c_int, epoll_event, __errno_location, strerror};


fn main() {
    let mut file_names = args();
    file_names.next();

    let mut file_map = HashMap::new();
    for file_name in file_names {
        eprintln!("Monitoring file: {}", &file_name);
        let file = OpenOptions::new().read(true).write(false).open(&file_name).unwrap();
        file_map.insert(file.as_raw_fd(), (file_name, file));
    }
    assert!(file_map.len() > 0, "At least one file to be monitored must be specified as an argument");


    let epoll_fd = unsafe {
        // 0 indicates that no flag is set
        let fd = epoll_create1(0);
        if fd > 1 {
            Ok(fd)
        }
        else {
            Err(get_err())
        }
    }.unwrap();


    for (fd, (path, _)) in file_map.iter() {
        monitor_file(epoll_fd, *fd).unwrap();
    }


    let mut buff: [u8; 10000] = [0; 10000];

    let buff_size: i32 = 10;
    let ev_template = epoll_event { events: 0, u64: 0} ; // dummy event for array init
    let mut event_buffer: [epoll_event; 10] = [ev_template; 10];

    loop {
        let event_count = unsafe {
            let fds = epoll_wait(epoll_fd, &mut event_buffer as *mut _ , buff_size, -1);
            if fds == -1 {
                Err(get_err())
            }
            else {
                Ok(fds)
            }
        }.unwrap();
        eprintln!("Events found: {}", event_count);

        for event in event_buffer.iter().take(event_count as usize) {
            let fd = event.u64 as i32;
            let (path, file) = file_map.get_mut(&fd).unwrap();
            let bytes = file.read(&mut buff).unwrap();
            println!("{:?}: file={} bytes={}", SystemTime::now(), path, bytes);
        }
    }
}

/// Add `fd` as a new file to be monitored in the given epoll instance.
/// The configuration is only for read events, ie epoll will wake when 
/// the file can be read.
fn monitor_file(epoll_fd: c_int, fd: RawFd) -> Result<(), String> {
    let mut event = epoll_event {
        events: EPOLLIN as u32,
        u64: fd as u64
    };

    unsafe {
        let result = epoll_ctl(epoll_fd, EPOLL_CTL_ADD, fd as i32, &mut event as *mut epoll_event);
        if result == 0 {
            Ok(())
        }
        else {
            Err(get_err())
        }
    }
}

/// Return current errno explenation string
/// Technically it should return Option<String> as this
/// may fail. 
fn get_err() -> String {
    unsafe {
        let errno_ptr = __errno_location();
        let ptr = strerror(*errno_ptr);
        CString::new(CStr::from_ptr(ptr).to_bytes()).unwrap().into_string().unwrap()
    }
}
