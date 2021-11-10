script;

use std::chain::auth::msg_sender;
use std::chain::auth::Caller;
use std::chain::revert;
use std::constants::ETH_COLOR;


fn main() -> bool {();
    let sender = msg_sender();

    let t1: bool = sender == Caller::Some(ETH_COLOR);

    t1
}
