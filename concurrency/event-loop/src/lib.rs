#![feature(btree_drain_filter)]
//! Implementation of an Event Loop


use std::collections::{VecDeque, BTreeMap};
use std::time::{Duration, Instant};
use std::panic::{catch_unwind, set_hook};
use std::vec::IntoIter;


pub struct Timer {
    tree: BTreeMap<Instant, Event>
}


impl Timer {

    fn add_timer(&mut self, timeout: Duration, event: Event) -> Result<(), String> {
        let timeout_instant = Instant::now() + timeout;
        // This is iffy because it overwrites an event if the timeout happens
        // to be at the same time, but oh well
        self.insert(timeout_instant, event);
    }

    fn pop_older_than(&mut self, instant: Instant) -> Vec<Event> {
        self.tree.drain_filter(|timeout| timeout < instant).collect()
    }

    fn pop_expired(&mut self) -> Vec<Event> {
        self.pop_older_than(Instant::now())
    }
}


pub struct Opts {
    max_retries: u8
}

#[derive(Debug, Copy, PartialEq)]
pub enum EventType {
    Timer(Duration),
    IO(i8)
}

pub struct Event {
    callback: Box<dyn FnMut (&mut EventLoop) -> Result<(), String>>,
    attempt: u8,
    event_type: EventType,
}

enum CallbackError {
    Panic(String),
    Error(String)
}

type CallbackResult = Result<(), CallbackError>


struct EventLoopIterator<'a> {
    expired_timers: IntoIter
}

impl<'a> EventLoopIterator<'a> {

    pub fn new(timer_events: &'a mut Timer, io_events: &'a mut VecDeque<Event>) -> Self {
        EventLoopIterator {
            expired_timers: timer_events.pop_expired().into_iter()
        }
    }

}

impl<'a> Iterator for EventLoopIterator<'a> {

    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.expired_timers.next()
    }

}

// Should I separate the event loop into an event loop and a runner?
// The event loop could be a dumb event queue aggregator with a run method,
// that handles all events and return a list of (Event, CallbackResult) pairs.
//
// The runtime would make use of the event loop to manage things and set options,
// add different behaviors and what not.
pub struct EventLoop {
    timer_events: Timer,
    io_events: Queue,
    opts: Opts
}

impl EventLoop {

    pub fn new(opts: Opts) -> Self {
        EventLoop {
            io_events: VecDeque::new(),
            timer_events: Timer::new(),
            opts
        }
    }

    pub fn add_timer<'a, T>(&mut self, callback: T, Duration: delay) -> ()
        where T: FnMut(&mut EventLoop) -> Result<(), String> + 'static
    {
        let event = Event {
            callback: Box::new(callback),
            attempt: 0,
            event_type: EventType::Timer(delay)
        };
        self.push_event(event)
    }

    pub fn monitor_fd() {}

    pub fn run(&mut self) {
        loop {
            // TODO how to avoid this busy looping?
            // Would be nice to be notified until there's somethign else in the queues
            self.handle_events()
        }
    }

    fn perform_callback(&mut self, &mut event: Event) -> CallbackResult {
        catch_unwind(|| (event.callback)(self))
            .map(|callback_return| {
                callback_return
                    .map(|_| Ok(()))
                    .unwrap_or_else(|msg| Err(CallbackError::Error(msg)))
            })
            .unwrap_or_else(|panic_reason| Err(CallbackError::Panic(format!("{:?}", panic_reason))))
    }

    fn handle_callback_result(&mut self, event: Event, callback_result: CallbackResult) {
        if let Err(CallbackError::Panic(reason)) = callback_result {
            eprintln!("aborting handling event (): callback panicked: {}", reason);
        }

        else if let Err(CallbackError::Error(msg)) = callback_result {
            eprintln!("error handling event (): callback returned error: {}", msg);
            if event.attempt < self.opts.max_retries {
                event.attempt += 1;
                self.push_event(event);
            }
        }
    }

    /// Handle events in event loop until all event queues are empty
    pub fn handle_events(&mut self) {
        for event in EventLoopIterator::new(&mut self.timer_events, &mut self.io_events) {
            let callback_result = self.perform_callback();
            self.handle_callback_result(event, callback_result);
        }
    }

    fn push_event(&mut self, event: Event) {
        match event.event_type {
            EventType::Timer(duration) => self.timer.add_timer(duration, event),
            _ => (),
        }
    }

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
