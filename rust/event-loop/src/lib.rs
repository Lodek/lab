use std::collections::VecDeque;

// TODO document structs


pub struct EventLoop<EventEnum> {
    user_events: VecDeque<Event<EventEnum, EventEnum>>
}

type Callback<T, EventEnum> = fn (T, &mut EventLoop<EventEnum>) -> ();

pub struct Event<T, EventEnum> {
    callback: Callback<T, EventEnum>,
    data: T
}


impl<EventEnum> EventLoop<EventEnum> {

    pub fn new() -> Self {
        EventLoop {
            user_events: VecDeque::new()
        }
    }

    pub fn push_event(&mut self, callback: Callback<EventEnum, EventEnum>, data: EventEnum) {
        let event = Event {
            callback,
            data
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
            match self.user_events.pop_front() {
                Some(event) => (event.callback)(event.data, self),
                None => break
            }
        }

    }

    fn tear_down() {}

    fn add_timer() {}

    fn monitor_fd() {}
}

// TODO impl Drop trait


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn callback_updates_ctx() {

        // Given shared ctx type
        pub struct Ctx(i8);
        // given event data with ref to context
        pub struct CustomData<'a>(&'a mut Ctx);
        // given callback that mutates value in context
        fn update_ctx<'a>(data: CustomData<'a>, ev_loop: &mut EventLoop<CustomData<'a>>) -> () {
            data.0.0 = 69;
        }
        // given event loop with event to update context through the callback
        let mut ctx = Ctx(42);
        let mut event_loop = EventLoop::new();
        event_loop.push_event(update_ctx, CustomData(&mut ctx));

        // when i handle the current events
        event_loop.handle_events();

        // then ctx was updated
        assert_eq!(ctx.0, 69);
    }
}
