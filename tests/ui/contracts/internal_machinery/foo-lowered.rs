//@ run-pass

extern crate core;

// TODO check here if lowered program behaves as expected before doing the
// actual lowering

// checks that variable declarations are lowered properly, with the ability to
// access function parameters

/// dummy function mocking contracts_enabled compiler intrinsic
fn contracts_enabled() -> bool {
    true
}

fn contract_check_requires<C: Fn() -> bool + Copy>(cond: Option<C>) {
    match cond {
        Some(cond) => {
            if !cond() {
                panic!("failed requires check");
            }
        }
        None => {}
    }
}

pub const fn build_check_ensures<Ret, C>(cond: C) -> C
where
    C: Fn(&Ret) -> bool + Copy + 'static,
{
    cond
}

fn contract_check_ensures<C: Fn(&Ret) -> bool + Copy, Ret>(cond: Option<C>, ret: Ret) -> Ret {
    match cond {
        Some(cond) => {
            if !cond(&ret) {
                panic!("failed ensures check");
            }
        }
        None => {}
    }
    ret
}

// TODO test other combinations of presence of requires/ensure clauses to see
// test how they would be lowered

fn foo(x: u32) -> u32 {
    let (pre, post) = if contracts_enabled() {
        let y = 2 * x;
        let pre = Some(|| true);
        let post = Some(build_check_ensures({ move |ret| *ret == y }));
        (pre, post)
    } else {
        (None, None)
    };

    contract_check_requires(pre);
    contract_check_ensures(post, { 2 * x })
}

fn main() {
    foo(1);
}
