script;

use std::constants::ETH_ID;
use std::contract_id::ContractId;
use std::chain::assert;
use std::context::balance_of_contract;
use minter_abi::*;

fn main() -> bool {
    let minter_contract = abi(Minter, 0x9c7921ef960d2ee20f3c13c44eaf166a28e297a0d7b84b1b158c753b595751a3);

    let minter_id = ~ContractId::from(0x9c7921ef960d2ee20f3c13c44eaf166a28e297a0d7b84b1b158c753b595751a3);


    let mut balance = balance_of_contract(0x9c7921ef960d2ee20f3c13c44eaf166a28e297a0d7b84b1b158c753b595751a3, minter_id);
    assert(balance == 0);

    minter_contract.mint_coins(1000000000000, 0, ETH_ID, 1);

    balance = balance_of_contract(0x9c7921ef960d2ee20f3c13c44eaf166a28e297a0d7b84b1b158c753b595751a3, minter_id);

    assert(balance == 1);

    true
}
