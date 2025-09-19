use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

pub fn get_next_id() -> usize {
    NEXT_ID.fetch_add(1, Ordering::SeqCst)
}

pub trait Element {
    fn out_act(&mut self);
    fn in_act(&mut self);
    fn get_summary(&mut self) -> String;
}