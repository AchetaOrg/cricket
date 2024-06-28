use crate::metrics::ServiceCheck;
use std::sync::Mutex;
use lazy_static::lazy_static;
use std::collections::VecDeque;

lazy_static! {
    static ref SERVICE_CHECKS_QUEUE: Mutex<VecDeque<ServiceCheck>> = Mutex::new(VecDeque::new());
}

pub fn collect_service_checks() -> Vec<ServiceCheck> {
    let mut checks = Vec::new();
    let mut queue = SERVICE_CHECKS_QUEUE.lock().unwrap();
    while let Some(check) = queue.pop_front() {
        checks.push(check);
    }
    checks
}

pub fn push_service_check(check: ServiceCheck) {
    let mut queue = SERVICE_CHECKS_QUEUE.lock().unwrap();
    queue.push_back(check);
}
