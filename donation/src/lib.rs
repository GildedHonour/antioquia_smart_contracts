use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::TreeMap;
use near_sdk::{env, log, near_bindgen, require, AccountId, Balance, PanicOnDefault, Promise};
use std::collections::BTreeMap;
use uuid::Uuid;

// type DonationId = Uuid;
type DonationId = String;

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct DonationItem {
    pub donation_id: DonationId,
    pub from_account_id: AccountId,
    pub to_account_id: AccountId,
    pub agreed_amount: Balance,
    pub current_fee_percent: u128,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Donation {
    pub owner_account_id: AccountId,
    pub base_fee_percent: u128,
    pub items: TreeMap<DonationId, DonationItem>,
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

        let items: TreeMap<DonationId, DonationItem> = TreeMap::new(b"d");
        Self {
            owner_account_id,
            base_fee_percent: base_fee_percent2,
            items,
        }
    }

    ///makes a donation transaction
    #[payable]
    pub fn send(
        &mut self,
        donation_id: DonationId,
        from_account_id: AccountId,
        to_account_id: AccountId,
        agreed_amount: Balance,
        current_fee_percent: Option<u128>,
    ) -> (Balance, Balance) {
        require!(agreed_amount > 0, "agreed_amount must be greater than 0");

        let donation = DonationItem {
            donation_id: donation_id.clone(),
            from_account_id: env::predecessor_account_id(),
            to_account_id: to_account_id.clone(),
            agreed_amount: agreed_amount,
            current_fee_percent: current_fee_percent.unwrap_or(self.base_fee_percent),
        };

        self.items.insert(&donation_id.clone(), &donation);

        let amount_for_receiver = donation.agreed_amount / Self::HUNDRED_PERCENT
            * (Self::HUNDRED_PERCENT - donation.current_fee_percent);
        let amount_for_owner = donation.agreed_amount - amount_for_receiver;

        let p1 = Promise::new(to_account_id.clone()).transfer(amount_for_receiver);
        let p2 = Promise::new(self.owner_account_id.clone()).transfer(amount_for_owner);

        (amount_for_receiver, amount_for_owner)
    }

    pub fn get_donation(&self, donation_id: DonationId) -> BTreeMap<String, String> {
        let donation = self.items.get(&donation_id).unwrap();
        let mut tree: BTreeMap<String, String> = BTreeMap::new();
        tree.insert(
            String::from("donation_id"),
            String::from(donation_id.clone()),
        );

        tree.insert(
            String::from("from_account_id"),
            String::from(donation.from_account_id),
        );

        tree.insert(
            String::from("to_account_id"),
            String::from(donation.to_account_id),
        );

        tree.insert(
            String::from("amount"),
            String::from(donation.agreed_amount.to_string()),
        );

        tree.insert(
            String::from("fee_percent"),
            String::from(donation.current_fee_percent.to_string()),
        );

        tree
    }
}
