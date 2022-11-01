use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::TreeMap;
use near_sdk::{env, log, near_bindgen, require, AccountId, Balance, PanicOnDefault, Promise};

type LotteryId = String;

#[derive(BorshDeserialize, BorshSerialize, PartialEq, Debug)]
pub enum Status {
    New,
    Active,
    Closed,
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct LotteryItem {
    pub lottery_id: LotteryId,
    pub status: Status,
    pub funder_account_id: AccountId,
    pub participants: Vector<AccountId>,
    pub prise_amount: Balance,
    pub current_fee_percentage: u128,
    pub current_amount: Balance,
    pub winner_account_id: AccountId,
    pub is_prise_paid_off: bool,
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

    //TODO
    pub fn pick_random_winner(lotter_id: LotteryId) -> AccountId {
        
    }
}
