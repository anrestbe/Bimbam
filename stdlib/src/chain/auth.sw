library auth;
use ::ops::*;

// this can be a generic option when options land
enum Caller {
  Some: b256,
  None: (),
}

/// Returns `true` if the caller is external.
pub fn caller_is_external() -> bool {
  asm(r1) {
    gm r1 i1;
    r1: bool
  }
}

pub fn caller() -> Caller {
    Caller::Some(asm(r1) {
      gm r1 i2;
      r1: b256
    })
}

// expose a pub fn msg_sender()
// wrap  auth methods 1 & 3
pub fn msg_sender() -> Caller {
    if caller_is_external() {
        try_get_coin_owners()
    } else {
        caller()
    }
}

fn get_coin_owner() -> b256 {
    let inputs : Input[] = ?;
    let owner_candidate: b256;
    for input in inputs {
        if input.type = Coin {
            if candidate = zero {
                candidate = coin.owner;
            } else {
                if coin.owner == candidate {
                    continue;
                } else {
                    return Caller::None
                }
            }
        }
    }
    Caller::Some(owner_candidate)
}




