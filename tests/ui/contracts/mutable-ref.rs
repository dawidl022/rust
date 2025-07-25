//@ run-pass
//@ compile-flags: -Zcontract-checks=yes
#![feature(contracts)]
//~^ WARN the feature `contracts` is incomplete and may not be safe to use and/or cause compiler crashes [incomplete_features]

extern crate core;
use core::contracts::{ensures, requires};

#[requires(let init_x = *x; true)]
#[ensures(move |_| *x == 2 * init_x)]
fn double_in_place(x: &mut i32) {
    *x *= 2;
}

fn main() {
    let mut x = 1;
    double_in_place(&mut x);
}
