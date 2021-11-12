library leader_selection_abi;

abi LeaderSelection {
    fn init(gas: u64, coins: u64, color: b256, input: ());
    fn my_func(gas: u64, coins: u64, color: b256, input: ()) -> bool;
}

// abi AuthTesting {
//   fn returns_gm_one(gas: u64, coins: u64, color: b256, input: ()) -> bool;
// }