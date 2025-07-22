//@ run-pass
#![feature(contracts, cfg_contract_checks, contracts_internals, core_intrinsics)]
//~^ WARN the feature `contracts` is incomplete and may not be safe to use and/or cause compiler crashes [incomplete_features]

extern crate core;

fn foo(x: u32) -> u32 {
    let post = if core::intrinsics::contract_checks() {
        let y = 2 * x;
        core::intrinsics::contract_check_requires(|| y > 0);
        Some(core::contracts::build_check_ensures({ move |ret| *ret == y }))
    } else {
        None
    };

    // outer contract_check_ensures is unreachable, but this should not
    // trigger a warning!
    core::intrinsics::contract_check_ensures(post, {
        loop {
            return core::intrinsics::contract_check_ensures(post, { 2 * x });
        }
    })
}

fn main() {
    foo(1);
}
