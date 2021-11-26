contract;

use std::contants::*;
use abi::LeaderSelection;

// @review u64 usage...
    // there may be cases where uint256 was ported to u64, but a b256 may be required.

    ////////////////
    // Immutables //
    ////////////////

    // @todo add getter functions for all public storage variables/conts/immutable values.
    /// @dev The address of the deposit token
    pub immutable TOKEN_ADDRESS: Address;

    /// @dev The length of each round
    pub immutable ROUND_LENGTH: u64;

    /// @dev The length of the submission window
    pub immutable SUBMISSION_WINDOW_LENGTH: u64;

    /// @dev The ratio of "tickets" to deposited tokens
    pub immutable TICKET_RATIO: u64;

    /////////////
    // Storage //
    /////////////

    /// @dev Store the current leader, and the current candidate for next leader (entry with best hash)
    pub s_leader: Address;
    pub s_candidate: Address;

    /// @dev store the current target hash and the difference of the closest submission from the target
    pub s_closest_submission: u64;
    pub s_targetHash: b256;

    /// @dev store the time when next selection process starts (new target is calculated)
    pub s_submission_window_start: u64;

    /// @dev Store the time the current round ends (new leader is selected)
    pub s_round_end: u64;

    /// @dev store the total balance deposited in the contract
    pub s_total_deposit: u64;

    /// @dev Store whether the submission window is open (hence a new target hash has been generated)
    pub s_submission_window_open: bool;

    /// @dev Store the deposited balances of each address
    // @todo implement generic storage-backed mappings
    mapping(address => uint) public s_balances;

    ////////////
    // Events //
    ////////////

    // @review will there be "indexed" values in Fuel event logs?

    /// @dev Log a new deposit: the depositing address and the deposit amount
    struct Deposit {
        withdrawer: address, // indexed
        withdrawal: u64      // indexed
    }

    /// @dev Log a new withdrawal: the withdrawing address and the withdrawal amount
    struct Withdrawal {
        withdrawer: address, // indexed
        withdrawal: u64,     // indexed
    }

    /// @dev Log a submission : the submitting address and the resulting hash
    struct Submission {
        submitter: address,  // indexed
        submittedHash: u64,
    }

    /// @dev Log the start of a new round : the new leader and round end time
    struct NewRound {
        leader:address,     // indexed
        endTime: u64,
    }

abi LeaderSelection {
    fn init(params: InitParams);
    fn deposit(amount: u64);
    fn withdraw(amount: u64);
    fn open_submission_window();
    fn submit(s: u64);
    fn new_round();
}

impl LeaderSelection for Contract {

    fn init(params: InitParams) {
        TOKEN_ADDRESS = params.token_address;
        ROUND_LENGTH = params.round_length;
        SUBMISSION_WINDOW_LENGTH = params.submission_window_length;
        TICKET_RATIO = params.ticket_ratio;
        s_target_hash = params.genesis_seed;

        s_closest_submission = MAX_B256;

        // solhint-disable-next-line not-rely-on-time
        // @audit how to deal with timestamps in sway?
        fn block_height() -> u64 {
            asm(height) {
                bhei height;
                height: u64
            }
        }
        s_round_end = block.timestamp + params.round_length;
        s_submission_window_start = s_roundEnd - submission_window_length;
        s_submission_window_open = false;
    }


}


/// @notice Constructor.
    /// @param tokenAddress: The address of the deposit token used for weighted selection
    /// @param roundLength: The amount of time for after which a new leader can be instated
    /// @param submissionWindowLength: The period of time before the end of a round where deposits/withdrawals are blocked, the target hash is revealed, and submisssions are allowed
    /// @param ticketRatio: The number of deposited tokens per "ticket".
    /// @param genesisSeed: Used to initialize the target hash for the first round
    /// @dev On deployment, there is no leader. The contract accepts deposits for a fixed period. Then a submission window is entered, after which the first leader is instated
    struct InitParams {
        token_address: Address,
        round_length: b256, // uint256 ... => u64?
        submissionWindowLength: b256, // uint256 ... => u64?
        ticket_ratio: b256, // uint256 ... => u64?
        genesis_seed: b256
    }
