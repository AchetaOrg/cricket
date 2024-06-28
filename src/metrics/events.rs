use crate::metrics::Event;
use std::sync::Mutex;
use lazy_static::lazy_static;
use std::collections::VecDeque;

lazy_static! {
    static ref EVENTS_QUEUE: Mutex<VecDeque<Event>> = Mutex::new(VecDeque::new());
}

pub fn collect_events() -> Vec<Event> {
    let mut events = Vec::new();
    let mut queue = EVENTS_QUEUE.lock().unwrap();
    while let Some(event) = queue.pop_front() {
        events.push(event);
    }
    events
}

pub fn push_event(event: Event) {
    let mut queue = EVENTS_QUEUE.lock().unwrap();
    queue.push_back(event);
}
