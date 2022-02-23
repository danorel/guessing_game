use std::ops::Range;
use rand::Rng;

pub fn random () -> i32 {
    rand::thread_rng().gen_range::<i32, Range::<i32>>(1..101)
}