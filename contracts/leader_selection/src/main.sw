contract;

// @todo reduce imports to minimum needed
use std::contants::*;
use std::contants::*;
use std::context::*;
use std::address::Address;
use abi::LeaderSelection;
use std::hash::*

// import "./vendor/ds/ds-token.sol";

// @todo extract into stdlib/context
fn block_height() -> u64 {
    asm(height) {
        bhei height;
        height: u64
    }
}

// @todo setup abi DSTokenf
let DSToken = abi(ds_token_abi, TOKEN_ADDRESS);

// @review u64 usage...
    // there may be cases where uint256 was ported to u64, but a b256 may be required.

////////////////////////////////////////
// Immutables
////////////////////////////////////////

// @todo add getter functions for all public storage variables/conts/immutable values.
/// @dev The address of the deposit token
pub immutable TOKEN_ADDRESS: Address;

/// @dev The length of each round
pub immutable ROUND_LENGTH: u64;

/// @dev The length of the submission window
pub immutable SUBMISSION_WINDOW_LENGTH: u64;

/// @dev The ratio of "tickets" to deposited tokens
pub immutable TICKET_RATIO: u64;


////////////////////////////////////////
// Storage declarations
////////////////////////////////////////

storage {
    /// @dev Store the current leader, and the current candidate for next leader (entry with best hash)
    pub leader: Address;
    pub candidate: Address;

    /// @dev store the current target hash and the difference of the closest submission from the target
    pub closest_submission: u64;
    pub targetHash: b256;

    /// @dev store the time when next selection process starts (new target is calculated)
    pub submission_window_start: u64;

    /// @dev Store the time the current round ends (new leader is selected)
    pub round_end: u64;

    /// @dev store the total balance deposited in the contract
    pub total_deposit: u64;

    /// @dev Store whether the submission window is open (hence a new target hash has been generated)
    pub submission_window_open: bool;

    /// @dev Store the deposited balances of each address
    balances: HashMap<Address, u64>;
}

////////////////////////////////////////
// Events
////////////////////////////////////////

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

    // @review how to set 'immutable' vars without a constructor? (immutables must be know at deployment time...)
    fn init(params: InitParams) {
        // @todo rename storage vars, remove 's_' from names
        TOKEN_ADDRESS = params.token_address;
        ROUND_LENGTH = params.round_length;
        SUBMISSION_WINDOW_LENGTH = params.submission_window_length;
        TICKET_RATIO = params.ticket_ratio;

        storage.target_hash.write() = params.genesis_seed;
        storage.closest_submission.write() = MAX_B256;

        // @todo replace usage of block.timestamp
        storage.round_end.write() = block.timestamp + params.round_length;
        storage.submission_window_start.write() = storage.round_end.read() - params.submission_window_length;
        storage.submission_window_open.write() = false;
    }

    /// @notice Deposit tokens in the contract
    /// @param amount: The amount of tokens to deposit
    /// @dev Requires this contract to be approved for at leas 'amount' on TOKEN_ADDRESS
    /// @dev Deposits are frozen during the submission window to avoid pre-calculation of the
    /// @dev minimum ticket number required to approach the revealed target hash
    fn deposit(amount: u64) {
        // @todo find a way to pass custom error type? panic() takes a u64, but need to pass that through from the assert()
        assert(!submission_window_open); // "Not allowed in submission window"
        assert(amount % TICKET_RATIO == 0); // "Not multiple of ticket ratio"
        storage.balances(msg_sender()).write() += amount;
        storage.total_deposit.write() += amount;

        // @review transfer of tokens !
        DSToken.transferFrom(msg.sender, address(this), amount);

        // @review event logging syntax
        log Deposit(msg_sender(), amount);

        // or:
        // log(Deposit {
        //     withdrawer: msg_sender(),
        //     withdrawal: amount
        // });


    }

    /// @notice Withdraw tokens from the contract
    /// @param amount: The amount of tokens to withdraw
    /// @dev amount must be a multiple of the ticket ratio
    fn withdraw(amount: u64) {
        assert(amount <= storage.balances(msg_sender())); // "Balance too low"
        assert(amount % TICKET_RATIO == 0); // "Not multiple of ticket ratio"

        storage.balances(msg_sender).write() -= amount;
        // @review transfer of tokens !
        DSToken.transfer(msg.sender, amount);
        log Withdrawal(msg.sender, amount);
    }

    /// @notice Open submission window to allow selection entries.
    /// @dev This is where the target hash is generated, as a function of the old target hash and the total deposit in the contract
    fn openSubmissionWindow() {
        assert(!s_submissionWindowOpen); // "Submission window already open"
        // @todo replace usage of block.timestamp
        assert(block.timestamp > s_submissionWindowStart); // "Too early to open"

        storage.targetHash.write() = hash(abi.encodePacked(storage.targetHash.read(), storage.totalDeposit.read()));
        storage.submission_window_open.write() = true;
    }

    /// @notice Submit an entry in the current lottery
    /// @param s: The 'ticket number' being entered
    /// @dev Requires the submission window to be open and a target hash to have been generated
    fn submit(s: u64) {
        assert(storage.submission_window_open.read()); // "submission window not open"
        // @todo replace usage of block.timestamp
        assert(block.timestamp < storage.round_end.read()); // "Round finished"

        // Check user has a high enough balance for submitted integer
        let max_allowed_ticket: u64 = storage.balances(msg.sender).read() / TICKET_RATIO;
        assert(s < max_allowed_ticket); // "Invalid ticket"

        let hash_value: b256 = hash(abi.encodePacked(msg_sender(), s));

        // Check that entry is closer to the target than the current best
        let difference: u64;
        if (hash_value > storage.targetHash.read()) {
            // @todo check to see if we have a 'from' implemented for these.
            difference = u64.from(hash_value) - u64.from(storage.target_hash.read());
        } else {
            difference = u64.from(storage.target_hash.read()) - u64.from(hash_value);
        }
        assert(difference < storage.closest_submission.read()); // "Hash not better"

        // Set new best entry and candidate
        storage.closest_submission.write() = difference;
        storage.candidate.write() = msg_sender();

        log Submission(msg_sender(), difference);
    }

    /// @notice Start a new round: End submission window, set new leader and reset lottery state
    fn newRound() {
        // @todo replace usage of block.timestamp
        assert(block.timestamp >= storage.round_end.read()); // "Current round not finished"
        // @todo replace usage of block.timestamp
        storage.round_end.write() = block.timestamp + ROUND_LENGTH;
        storage.submission_window_start.write() = storage.round_end.read() - SUBMISSION_WINDOW_LENGTH;
        storage.closest_submission.write() = MAX_U64;
        storage.leader.write() = storage.candidate.read();
        storage.candidate.write() = Address::from(0); // try with `constants::ZERO`
        storage.submission_window_open.write() = false;

        log(NewRound(storage.leader.read(), storage.round_end.read()));
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
        submission_window_length: b256, // uint256 ... => u64?
        ticket_ratio: b256, // uint256 ... => u64?
        genesis_seed: b256
    }
