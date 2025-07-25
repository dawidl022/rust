//@ run-pass
//@ compile-flags: -Zcontract-checks=yes
#![feature(contracts)]
//~^ WARN the feature `contracts` is incomplete and may not be safe to use and/or cause compiler crashes [incomplete_features]

extern crate core;
use core::contracts::requires;

#[requires(if x > 0 { return 0 }; true)]
fn foo(x: u32) -> i32 {
    42
}

fn main() {
    assert_eq!(foo(1), 42);
}
