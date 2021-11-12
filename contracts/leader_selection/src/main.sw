contract;

use leader_selection_abi::LeaderSelection;

impl LeaderSelection for Contract {

    ////////////
    // Events //
    ////////////

    /// @dev Log a new deposit: the depositing address and the deposit amount
    struct Deposit {
        withdrawer: address, // indexed
        withdrawal: u64      // indexed
    }

    /// @dev Log a new withdrawal: the withdrawing address and the withdrawal amount
    struct Withdrawal {
        withdrawer: address, // indexed
        withdrawal: u64, // indexed
    }

    /// @dev Log a submission : the submitting address and the resulting hash
    struct Submission {
        submitter: address, // indexed
        submittedHash: u64,
    }

    /// @dev Log the start of a new round : the new leader and round end time
    struct NewRound {
        leader:address, // indexed
        endTime: u64,
    }

}
