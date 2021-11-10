contract;
use std::chain::auth::*;
use auth_testing_abi::AuthTesting;

impl AuthTesting for Contract {
    // @todo refactor this to use msg_sender()
  fn returns_gm_one(gas: u64, coins: u64, color: b256, input: ()) -> bool {
     caller_is_external()
  }

  fn get_coin_sender(gas: u64, coins: u64, color: b256, input: ()) -> Caller {
      msg_sender()
  }
}
