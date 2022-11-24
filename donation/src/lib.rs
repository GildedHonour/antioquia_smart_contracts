use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::TreeMap;
use near_sdk::{env, log, near_bindgen, require, AccountId, Balance, PanicOnDefault, Promise};
use std::collections::BTreeMap;

type DonationId = UUID;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Donation {
    pub owner_account_id: AccountId,
    pub base_fee_percent: u128,
    pub items: TreeMap<DonatioId, DonationItem>,
}

#[near_bindgen]
impl Donation {
    const MIN_FEE_PERCENT: u128 = 0;
    const MAX_FEE_PERCENT: u128 = 100;
    const HUNDRED_PERCENT: u128 = 100;

    #[init]
    pub fn init(_owner_account_id: Option<AccountId>, base_fee_percent: Option<u128>) -> Self {
        require!(!env::state_exists(), "Already initialized");
        let owner_account_id = _owner_account_id.unwrap_or(env::signer_account_id());
        let base_fee_percent2 = base_fee_percent.unwrap_or(Self::MIN_FEE_PERCENT);
        require!(
            (base_fee_percent2 >= Self::MIN_FEE_PERCENT)
                && (base_fee_percent2 <= Self::MAX_FEE_PERCENT),
            format!(
                "base_fee_percent must be between {}..{}",
                &Self::MIN_FEE_PERCENT,
                &Self::MAX_FEE_PERCENT
            )
        );

        let items: TreeMap<LotteryId, LotteryItem> = TreeMap::new(b"t");
        Self {
            owner_account_id,
            base_fee_percent: base_fee_percent2,
            items,
        }
    }

    ///creates a new lottery
    ///and deposits funds
    #[payable]
    pub fn send(
        &mut self,
        donation_id: DonationId,
        from_account_id: AccountId,
        to_account_id: AccountId,
        agreed_amount: Balance,
        current_fee_percent: Option<u128>,
    ) -> Option<DonationId> {
        require!(
            agreed_amount > 0,
            "agreed_amount must be greater than 0"
        );
        //TODO
    }

}
