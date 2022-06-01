use std::os::unix::io::{RawFd, AsRawFd};
use std::fs::File;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::collections::btree_map::BTreeMap;
use std::collections::hash_set::HashSet;
use std::{thread, time};

use libc::{epoll_create1, epoll_event, epoll_ctl, EPOLL_CTL_ADD, EPOLLIN, EPOLLOUT, c_int};

const ADD_CTL_FLAGS: c_int = EPOLLIN | EPOLLOUT;


/// Abstraction for an Event Loop that monitors files until they readable.

struct MonitoredFile<'a> {
    file: &'a mut File,
    callback: Box<dyn FnMut(&mut File)>
}


/// Events should be a set such that it contains one event for each file
struct EPollEventLoop<'a> {
    epoll_fd: RawFd,
    files: BTreeMap<RawFd, MonitoredFile<'a>>,
    rx: Receiver<RawFd>,
    thread_handle: thread::JoinHandle<()>,
}

/// Runner repeatedly reads epoll fd until a file is readable.
/// Notifies event loop about readable files through a channel.
fn epoll_runner(sender: Sender<RawFd>, epoll_fd: RawFd) {
    //let struct_vec = Vec::new();

    loop {
        thread::sleep(time::Duration::from_millis(1000));
        //epoll_wait();
        // iterate over struct, write Fds to sender
        // check whether send is closed, if it is kill thread
    }
}

impl<'a> EPollEventLoop<'a> {

    pub fn new() -> Option<Self>  {
        let flags: c_int = 0;
        let epoll_fd: RawFd = epoll_create1(flags);
        if epoll_fd == -1 {
            // FIXME check errno
            return None;
        }

        let (tx, rx) = channel();
        let thread_handle = thread::spawn(move || epoll_runner(tx, epoll_fd));

        Some(Self {
            files: BTreeMap::new(),
            epoll_fd,
            rx,
            thread_handle,
        })
    }

    pub fn monitor_file(&mut self, file: &'a mut File, callback: Box<dyn FnMut(&mut File)>) -> Option<()>
    {
        let fd = file.as_raw_fd();
        let event_data = epoll_event {
            events: ADD_CTL_FLAGS as u32,
            u64: fd as u64
        };

        if epoll_ctl(self.epoll_fd, EPOLL_CTL_ADD, fd, &mut event_data as *mut epoll_event) == -1 {
            return None;
        }

        let monitored_file = MonitoredFile {
            file,
            callback: Box::new(callback)
        };
        self.files.insert(fd, monitored_file);

        Some(())
    }

    pub fn handle_events(&mut self) {
        let updated_fds = self.rx.try_iter().collect::<HashSet<_>>();
        for fd in updated_fds.iter() {
            // should be safe as all values are ocming from the worker thread
            let monitored_file = self.files.get_mut(fd).unwrap();
            (monitored_file.callback)(monitored_file.file)
        }
    }

}

impl<'a> Drop for EPollEventLoop<'a> {

    fn drop(&mut self) {
        // remove all files from epoll
        // close epoll
        // remove files so that external client is not affected, since finishing epoll closes
        // the associated file descriptors
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
