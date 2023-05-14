use std::borrow::Borrow;
use std::cell::RefCell;
use rand::{rngs::SmallRng, SeedableRng};
use rand::Rng;


thread_local! {
    static RNG: RefCell<SmallRng> = RefCell::new(SmallRng::from_entropy());
}

/// exclusive range
pub fn range(lower: f64, upper: f64) -> f64 {
    RNG.with(|gen| {
        let mut gen = gen.borrow_mut();
        gen.gen_range(lower..upper)
    })
}
