contract;

use minter_abi::*;

impl Minter for Contract {
    fn mint_coins(gas: u64, coins: u64, asset_id: b256, amount: u64) {
        asm(r1: amount) {
            mint r1;
        };
    }
}
