// TODO Finish reading about panic handling
// Not sure what should be the expected behavior and how to implement a hook.
// I'd like to keep the context about the event being handle, but maybe that's unecessary.
// Does calling panic breaks the flow of execution?
// Does panicking return from the current function?
// How can a function panic without crashing the event loop?

// TODO How am I going to handle the event type stuff? Any, dynamic dispatching, boxing or generics?

// TODO Refactor code accordingly

// TODO finish Runner

//! Implementation of a Event Loop
use std::collections::{VecDeque, BTreeMap};
use std::time::{Duration, Instant};


pub struct Opts {
    max_retries: u8
}

pub struct Event {
    // Is there any way to store a closure without a box?
    callback: Box<dyn FnMut () -> Result<(), String>>,
    attempt: u8,
}

trait Timer {
    fn add_timer(timeout: Duration, event: Event) -> Result<(), String>;
    //fn pop_older_than(&mut self, instant: Instant) -> impl Iterator<Item=Event>;
    //fn pop_expired(&mut self) -> impl Iterator<Item=Event> {
        //self.pop_older_than(Instant::now())
    //}
}

pub struct EventLoop {
    user_events: VecDeque<Event>
}

impl EventLoop {

    pub fn new() -> Self {
        EventLoop {
            user_events: VecDeque::new()
        }
    }

    pub fn push_event<T>(&mut self, callback: T) -> ()
        where T: FnMut() -> Result<(), String> + 'static
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

    pub fn handle_events(&mut self) {
        loop {
            // TODO refactor using combinators
            match self.user_events.pop_front() {
                // FIXME Add retry policy based on event result
                Some(mut event) => {
                    (event.callback)();
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
        let ctx_ref = ctx.clone();
        let update_ctx = move || {
            ctx_ref.0;
            Ok(())
        };
        let mut event_loop = EventLoop::new();
        event_loop.push_event(update_ctx);

        // when i handle the current events
        event_loop.handle_events();
    }
}
