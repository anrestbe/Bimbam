contract;

storage {
    owner: ContractOwner =
        ContractOwner {
            data:
                OwnerInner {
                    value:  0x0000000000000000000000000000000000000000000000000000000000000000
                }
        },
    number: u64 = 0,
}

struct ContractOwner {
    data: OwnerInner,
}

struct OwnerInner {
    value: b256,
}


impure fn returns_owner() -> b256 {
    (storage.owner).data.value
}