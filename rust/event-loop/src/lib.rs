// TODO Finish reading about panic handling
// Not sure what should be the expected behavior and how to implement a hook.
// I'd like to keep the context about the event being handle, but maybe that's unecessary.
// Does calling panic breaks the flow of execution?
// Does panicking return from the current function?
// How can a function panic without crashing the event loop?

// TODO Refactor code accordingly

// TODO finish Runner

//! Implementation of an Event Loop
use std::collections::{VecDeque, BTreeMap};
use std::time::{Duration, Instant};
use std::panic::{catch_unwind, set_hook};



pub struct Opts {
    max_retries: u8
}

pub struct Event {
    callback: Box<dyn FnMut (&mut EventLoop) -> Result<(), String>>,
    attempt: u8,
}

trait Timer {
    fn add_timer(timeout: Duration, event: Event) -> Result<(), String>;
    //fn pop_older_than(&mut self, instant: Instant) -> impl Iterator<Item=Event>;
    //fn pop_expired(&mut self) -> impl Iterator<Item=Event> {
        //self.pop_older_than(Instant::now())
    //}
}

type Queue = VecDeque<Event>;

pub struct EventLoop {
    user_events: Queue,
    timer_events: Queue,
    io_events: Queue,
    opts: Opts
}

impl EventLoop {

    pub fn new(opts: Opts) -> Self {
        EventLoop {
            user_events: VecDeque::new(),
            io_events: VecDeque::new(),
            timer_events: VecDeque::new(),
            opts
        }
    }

    pub fn push_event<'a, T>(&mut self, callback: T) -> ()
        where T: FnMut(&mut EventLoop) -> Result<(), String> + 'static
    {
        let event = Event {
            callback: Box::new(callback),
            attempt: 0,
        };
        self.user_events.push_back(event)
    }

    pub fn run(&mut self) {
        loop {
            self.handle_events()
        }
    }

    fn handle_event(&mut self, mut event: Event) {
        if event.attempt <= self.opts.max_retries {
            catch_unwind(|| (event.callback)(self))
                .and_then(|callback_result| {
                    callback_result.or_else(|err| {
                        eprintln!("error handling event: event={:?}: err={}", event, err);
                        event.attempt += 1;
                        self.user_events.push_back(event);
                        Ok(())
                    })
                }).or_else(|panic_cause| {
                    eprintln!("panic handling event: event={:?}: {:?}", event, panic_cause);
                    Ok(())
                });
        }
        else {
            eprintln!("max retry for handling event reached: {:?}", event);
        }
    }

    pub fn handle_events(&mut self) {
        loop {
            match self.user_events.pop_front() {
                Some(event) => {
                    self.handle_event(event);
                },
                None => break
            }
        }
    }

    fn tear_down() {}

    fn add_timer() {}

    fn monitor_fd() {}
}

// TODO  impl Drop for EventLoop {


#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn callback_updates_ctx() {
        struct Ctx(u8);

        let ctx = Rc::new(Ctx(42));
        //let ctx_ref = ctx.clone();
        let update_ctx = |a| {
            //ctx_ref.0;
            Ok(())
        };
        let mut event_loop = EventLoop::new();
        event_loop.push_event(update_ctx);

        // when i handle the current events
        event_loop.handle_events();
    }
}
