use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::TreeMap;
use near_sdk::{env, log, near_bindgen, require, AccountId, Balance, PanicOnDefault, Promise};

type LotteryId = String;

#[derive(BorshDeserialize, BorshSerialize, PartialEq, Debug)]
pub enum Status {
    Draft,
    New,
    Canceled,
    Active,
    Over,
    Closed
}

#[derive(BorshDeserialize, BorshSerialize, PartialEq, Debug)]
pub enum PrizeStatus {
    DepositPending,
    PartiallyFunded,
    Funded,
    WinnerPayedOff,
    OwnerReimbursed,
    PartiallyPayedOffAndReimbursed,
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct LotteryItem {
    pub lottery_id: LotteryId,
    pub status: Status,
    pub organiser_account_id: AccountId,
    pub participants: Vector<LotteryParticipant>,
    pub agreed_prize_amount: Balance,
    pub current_prize_amount: Balance,
    pub prize_status: PrizeStatus,
    pub current_fee_percentage: u128,
    pub winner: LotteryParticipant,
    // pub amount_paid_off: bool,
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct LotteryParticipant {
    pub account_id: AccountId,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Lottery {
    pub owner_id: AccountId,
    pub base_fee_percentage: u128,
    pub items: TreeMap<EscrowId, LotteryItem>,
}

#[near_bindgen]
impl Lottery {
    const MIN_FEE_PERCENTAGE: u128 = 0;
    const MAX_FEE_PERCENTAGE: u128 = 100;
    const HUNDRED_PERCENT: u128 = 100;

    #[init]
    pub fn init(owner_account_id: Option<AccountId>, base_fee_percentage: Option<u128>) -> Self {
        require!(!env::state_exists(), "Already initialized");
        let owner_id = owner_id.unwrap_or(env::signer_account_id());

        let base_fee_percentage2 = base_fee_percentage.unwrap_or(Self::MIN_FEE_PERCENTAGE);
        require!(
            (base_fee_percentage2 >= Self::MIN_FEE_PERCENTAGE) && (base_fee_percentage2 <= Self::MAX_FEE_PERCENTAGE),
            format!(
                "base_fee_percentage must be between {}..{}",
                &Self::MIN_FEE_PERCENTAGE,
                &Self::MAX_FEE_PERCENTAGE
            )
        );
    }

    ///creates a new lottery
    pub fn new(
        &mut self,
        lottery_id: LotteryId,
        organiser_account_id: AccountId,
        agreed_prize_amount: Balance,
        current_fee_percentage: Option<u128>,
    ) -> Option<LotteryId> {
        let cond = (self.owner_id == env::predecessor_account_id()) || (organiser_account_id == env::predecessor_account_id());
        require!(cond, "only funder or owner of this escrow may call this method");

        if !self.items.contains_key(&lottery_id) {
            let new_item = LotteryItem {
                lottery_id: lottery_id.clone(),
                agreed_prize_amount,
                status: Status::Draft,
                prize_status: PrizeStatus::DepositPending,
                organiser_account_id,
                current_fee_percentage: current_fee_percentage.unwrap_or(self.base_fee_percentage),
            };

            self.items.insert(&lottery_id.clone(), &new_item);
            Some(lottery_id)
        } else {
            log!("lottery_id '{}' already exists; generate a new one", lottery_id);
            None
        }
    }

    #[payable]
    pub deposit_funds(&self, lotter_id: LotteryId) -> Balance {
        require!(agreed_amount > 0, "agreed_amount must be greater than 0");

        let actual_amount: Balance = env::attached_deposit();
        require!(
            actual_amount > 0,
            format!("expected deposit: {}; actual one: {}", agreed_amount, actual_amount)
        );
        require!(
            agreed_amount == actual_amount,
            format!(
                "agreed_amount and actual_amount must be equal: {} and {}",
                agreed_amount, actual_amount
            )
        );

        if self.items.contains_key(&lottery_id) {
            //TODO update 'current_prize_amount'
        } else {
            log!("lottery_id '{}' doens't exists");
            0
        }
    }

    //TODO
    pub fn pick_random_winner(&self, lotter_id: LotteryId) -> AccountId {

    }

    // Generate random u8 number (0-254)
    fn random_u8(&self, index: usize) -> u8 {
        *env::random_seed().get(index).unwrap()
    }

    // Get random number from 0 to max
    fn random_in_range(&self, index: usize, max: usize) -> u32 {
        let rand_divider = 256 as f64 / (max + 1) as f64;
        let result = self.random_u8(index) as f64 / rand_divider;
        return result as u32;
    }

    //TODO
    fn get_info(&self, lotter_id: LotteryId) {
        //lottery_id:
        //organiser_account_id:
        //status
        //participants_count:
        //prize:
        //prize_status
        //fees percentage
    }

    //TODO
    fn is_participant_in(&self, participant_account_id: AccountId) {
    }
}
