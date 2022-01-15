contract;
storage {
    owner: b256,
    number: u64,
}


fn test() {
    // storage access means this function should be impure
    let test = storage.owner;
}
