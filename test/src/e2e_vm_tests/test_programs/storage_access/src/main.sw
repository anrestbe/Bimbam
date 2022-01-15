contract;

storage {
    owner: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000,
    number: u64 = 0,
}


impure fn returns_owner() -> b256 {
    storage.owner
}