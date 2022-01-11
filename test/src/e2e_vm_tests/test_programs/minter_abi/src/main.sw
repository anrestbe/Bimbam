library minter_abi;

abi Minter {
    fn mint_coins(gas: u64, coins: u64, asset_id: b256, amount: u64);
}
