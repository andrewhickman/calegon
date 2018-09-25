use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Name(usize);

impl Default for Name {
    fn default() -> Self {
        static COUNT: AtomicUsize = AtomicUsize::new(0);

        Name(COUNT.fetch_add(1, Ordering::Relaxed))
    }
}
