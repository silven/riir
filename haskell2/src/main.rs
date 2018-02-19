#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate curryrs;

use curryrs::hsrt::{start,stop};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

struct Haskell();

impl Haskell {
    fn start() -> Self {
        start("Haskell, ju".to_owned());
        return Self{};
    }
}

impl Drop for Haskell {
    fn drop(&mut self) {
        stop();
    }
}

fn main() {
    let _h = Haskell::start();

    println!("Calling Haskell from rust, 2^6 is {}.", unsafe { sixth(2) });
}
