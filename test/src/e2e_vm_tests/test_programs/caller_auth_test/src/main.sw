script;
use std::chain::revert;
use auth_testing_abi::AuthTesting;
use std::constants::ETH_COLOR;

fn main() -> bool {
  let caller = abi(AuthTesting, 0x8ca92c2a448e86f374657604a3d62f3d83226f86acfed38e8124cce826926f7f);

  // should be false in the case of a script
  let is_external = caller.returns_gm_one(1000, 0, ETH_COLOR, ());
  let coin_sender: b256 = caller.get_coin_sender(1000, 1, ETH_COLOR, ());
  // @todo refactor to use std::context::id
  // coin_sender currently should be 0x00..
  // t1 should be false
  let t1 = coin_sender == 0x8ca92c2a448e86f374657604a3d62f3d83226f86acfed38e8124cce826926f7f;

  !is_external && t1
}
