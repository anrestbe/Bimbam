contract;

// TODO: declaring ContractOwner after `storage` breaks the node dependencies calculation.
// Not handling that right now but it needs to be handled before this PR goes in.
// If you're code reviewing this and this comment is still here don't approve it.

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

abi TestAbi {
  fn test_deposit(unused: u64, unused: u64, unused: b256, val: u64) -> b256;
}

impl TestAbi for Contract {
  impure fn test_deposit(unused: u64, unused: u64, unused: b256, val: u64) -> b256 {
    returns_owner()
  }
}
