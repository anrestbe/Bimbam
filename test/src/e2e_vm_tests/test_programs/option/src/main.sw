script;

use std::option::*;

fn returns_option(b: bool) -> Option<u8> {
    if b {
        Option::Some(42)
    } else {
        Option::None()
    }
}

fn main() -> bool {
    let res1 = returns_option(true);
    let res2 = returns_option(false);

    assert(~Option::is_some(res1));
    assert(~Option::is_none(res2));

    true
}
