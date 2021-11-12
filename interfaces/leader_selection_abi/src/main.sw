library leader_selection_abi;

abi LeaderSelection {
    fn init(gas: u64, coins: u64, color: b256, input: ());
    fn my_func(gas: u64, coins: u64, color: b256, input: ()) -> bool;
    fn deposit(gas: u64, coins: u64, color: b256, input: ());
    fn withdraw(gas: u64, coins: u64, color: b256, input: ());
    fn open_submission_window(gas: u64, coins: u64, color: b256, input: ());
    fn submit(gas: u64, coins: u64, color: b256, input: ());
    fn new_round(gas: u64, coins: u64, color: b256, input: ());
}