use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::TreeMap;
use near_sdk::{env, log, near_bindgen, require, AccountId, Balance, PanicOnDefault, Promise};

type EscrowId = String;

#[derive(BorshDeserialize, BorshSerialize, PartialEq, Debug)]
pub enum Status {
    New,
    Active,
    PayedOff,
    Reimbursed,
}

//TODO add a separate status for 'EscrowFundsStatus'
//

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct EscrowItem {
    pub escrow_id: EscrowId,
    pub status: Status,
    pub funder_account_id: AccountId,
    pub beneficiary_account_id: AccountId,
    pub agreed_amount: Balance,
    pub current_amount: Balance,
    pub current_fee_percentage: u128,

    // pub inserted_at: u64,
    // pub funded_at: u64,
    // pub finished_at: u64,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Escrow {
    pub owner_id: AccountId,
    pub base_fee_percentage: u128,
    pub items: TreeMap<EscrowId, EscrowItem>,
}

#[near_bindgen]
impl Escrow {
    const MIN_FEE_PERCENTAGE: u128 = 0;
    const MAX_FEE_PERCENTAGE: u128 = 100;
    const HUNDRED_PERCENT: u128 = 100;

    /// initialize Escrow globally;
    /// it has to be called only once;
    /// * `base_fee_percentage` - percentage; has to be in between MIN_FEE_PERCENTAGE and MAX_FEE_PERCENTAGE
    #[init]
    pub fn init(_owner_id: Option<AccountId>, base_fee_percentage: Option<u128>) -> Self {
        require!(!env::state_exists(), "Already initialized");
        let owner_id = _owner_id.unwrap_or(env::signer_account_id());

        let base_fee_percentage2 = base_fee_percentage.unwrap_or(Self::MIN_FEE_PERCENTAGE);
        require!(
            (base_fee_percentage2 >= Self::MIN_FEE_PERCENTAGE) && (base_fee_percentage2 <= Self::MAX_FEE_PERCENTAGE),
            format!(
                "base_fee_percentage must be between {}..{}",
                &Self::MIN_FEE_PERCENTAGE,
                &Self::MAX_FEE_PERCENTAGE
            )
        );

        let items: TreeMap<EscrowId, EscrowItem> = TreeMap::new(b"t");
        Self {
            owner_id,
            base_fee_percentage: base_fee_percentage2,
            items,
        }
    }

    /// returns base_fee as percentage
    pub fn get_base_fee_percentage(&self) -> Balance {
        self.base_fee_percentage
    }

    /// returns the Id of the owner
    pub fn get_owner_id(&self) -> AccountId {
        self.owner_id.clone()
    }

    /// set base_fee as percentage
    /// it has to be in between MIN_FEE_PERCENTAGE and MAX_FEE_PERCENTAGE
    pub fn set_base_fee_percentage(&mut self, new_fee: Balance) -> Balance {
        require!(
            self.owner_id == env::predecessor_account_id(),
            "only owner may call this method"
        );
        let cond = (new_fee >= Self::MIN_FEE_PERCENTAGE) && (new_fee <= Self::MAX_FEE_PERCENTAGE);
        require!(
            cond,
            format!(
                "fee_percentage must to be between {} and {}",
                Self::MIN_FEE_PERCENTAGE,
                Self::MAX_FEE_PERCENTAGE
            )
        );
        self.base_fee_percentage = new_fee;
        self.base_fee_percentage
    }

    ///creates and activates a new escrow
    ///requires a payment
    #[payable]
    pub fn new(
        &mut self,
        escrow_id: EscrowId,
        funder_account_id: AccountId,
        beneficiary_account_id: AccountId,
        agreed_amount: Balance,
        current_fee_percentage: Option<u128>,
    ) -> Option<EscrowId> {
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

        let cond = (self.owner_id == env::predecessor_account_id()) || (funder_account_id == env::predecessor_account_id());
        require!(cond, "only funder or owner of this escrow may call this method");

        //FIXME:
        //in case of any error, return the funds to the funder
        //Promise::new(env::predecessor_account_id()).transfer(env::attached_deposit());

        if !self.items.contains_key(&escrow_id) {
            let new_item = EscrowItem {
                escrow_id: escrow_id.clone(),
                agreed_amount,
                current_amount: agreed_amount,
                status: Status::Active,
                funder_account_id,
                beneficiary_account_id,
                current_fee_percentage: current_fee_percentage.unwrap_or(self.base_fee_percentage),
            };

            self.items.insert(&escrow_id.clone(), &new_item);
            Some(escrow_id)
        } else {
            log!("escrow_id '{}' already exists; generate a new one", escrow_id);
            None
        }
    }

    /// releases funds to beneficiary:
    ///     (100 - fee %)    --> to beneficiary
    ///     (fee %)          --> to owner
    ///
    /// who may call this method
    ///     * owner
    ///     * funder
    pub fn release_deposit(&mut self, escrow_id: EscrowId) {
        match self.items.get(&escrow_id) {
            Some(mut escrow_item) => {
                require!(escrow_item.status == Status::Active, "this escrow isn't active");
                let authoriz_cond = (self.owner_id == env::predecessor_account_id())
                    || (escrow_item.funder_account_id == env::predecessor_account_id());
                require!(
                    authoriz_cond,
                    "only funder or owner of this escrow may call this method"
                );
                require!(
                    escrow_item.current_amount >= escrow_item.agreed_amount,
                    format!(
                        "the amount of the actual deposit ({}) must be equal of greater than the agreed amount ({})",
                        escrow_item.current_amount, escrow_item.agreed_amount
                    )
                );

                let amount_for_beneficiary = escrow_item.agreed_amount / Self::HUNDRED_PERCENT
                    * (Self::HUNDRED_PERCENT - escrow_item.current_fee_percentage);
                let amount_for_owner = escrow_item.agreed_amount - amount_for_beneficiary;

                //due to a potential rounding error,
                //verify that there'll be enough of the funds
                let amounts_sum = amount_for_beneficiary + amount_for_owner;
                let calc_cond = escrow_item.current_amount >= amounts_sum;
                require!(calc_cond, format!("current_amount ({}) must be equal to or greater than the sum of the amounts to be released ({});", escrow_item.current_amount, amounts_sum));

                //send funds to the beneficiary
                let p1 = Promise::new(escrow_item.beneficiary_account_id.clone()).transfer(amount_for_beneficiary);
                escrow_item.current_amount -= amount_for_beneficiary;
                log!(
                    "releasing '{}' to beneficiary '{}'; escrow_id '{}'",
                    amount_for_beneficiary,
                    escrow_item.beneficiary_account_id,
                    escrow_id
                );

                //send the fees to the owner
                let p2 = Promise::new(self.owner_id.clone()).transfer(amount_for_owner);
                p1.then(p2);
                //FIXME verify that _p1 has returned successfully
                escrow_item.current_amount -= amount_for_owner;
                log!(
                    "sending commission of '{}' ({}%) to owner_id '{}'; escrow_id '{}'",
                    amount_for_owner,
                    escrow_item.current_fee_percentage,
                    self.owner_id,
                    escrow_id
                );

                escrow_item.status = Status::PayedOff;
            }
            None => {
                //FIXME return None or Error
                log!("escrow_id '{}' not found", escrow_id)
            }
        }
    }

    /// reimburse the funder the funds
    /// who may call this method:
    ///     * owner
    ///     * beneficiary
    pub fn reimburse_funder(&self, escrow_id: EscrowId) {
        match self.items.get(&escrow_id) {
            Some(mut escrow_item) => {
                require!(escrow_item.status == Status::Active, "this escrow isn't active");

                let cond = (self.owner_id == env::predecessor_account_id())
                    || (escrow_item.beneficiary_account_id == env::predecessor_account_id());
                require!(cond, "only beneficiary or owner may call this method");

                //verify that there'll be enough of the funds
                let calc_cond = escrow_item.current_amount >= escrow_item.agreed_amount;
                require!(
                    calc_cond,
                    format!(
                        "current_amount ({}) must be equal to or greater than agreed_amount ({});",
                        escrow_item.current_amount, escrow_item.agreed_amount
                    )
                );

                let _p1 = Promise::new(escrow_item.funder_account_id.clone()).transfer(escrow_item.agreed_amount);
                //FIXME verify that _p1 has returned successfully
                escrow_item.status = Status::Reimbursed;
                escrow_item.current_amount = 0;
            }
            None => {
                //FIXME return None or Error
                //todo!()
                log!("escrow_id '{}' not found", escrow_id);
            }
        }
    }

    pub fn remove_item(&mut self, escrow_id: EscrowId) {
        match self.items.get(&escrow_id) {
            Some(escrow_item) => {
                require!(
                    escrow_item.status != Status::Active,
                    format!("escrow id {} is active, therefore it may not be removed", escrow_id)
                );

                require!(self.owner_id == env::predecessor_account_id(), "owner's only method");
                self.items.remove(&escrow_id);
            }
            None => {
                log!("escrow_id '{}' not found", escrow_id)
            }
        }
    }

    pub fn emergency_withdraw(&mut self, escrow_id: EscrowId) {
        match self.items.get(&escrow_id) {
            Some(mut escrow_item) => {
                require!(self.owner_id == env::predecessor_account_id(), "owner's only method");
                let _p1 = Promise::new(self.owner_id.clone()).transfer(escrow_item.current_amount);
                log!(
                    "emergency withdrawal from an escrow by owner; amount: {}, escrow_id: {}, owner_id: {}",
                    escrow_item.current_amount,
                    escrow_id,
                    self.owner_id
                );

                //FIXME
                //verify that the _p1 has finished successfully
                escrow_item.current_amount = 0;
            }
            None => {
                log!("escrow_id '{}' not found", escrow_id)
            }
        }
    }

    /// returns the balance of an EscrowItem
    pub fn get_balance(&self, escrow_id: EscrowId) -> Option<Balance> {
        match self.items.get(&escrow_id) {
            Some(item) => {
                require!(item.agreed_amount == item.current_amount);
                Some(item.agreed_amount)
            }
            None => None,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, MockedBlockchain};

    fn get_context(is_view: bool) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        let account0: AccountId = "0.near".parse().unwrap();
        builder
            .current_account_id(account0)
            .signer_account_id(accounts(0))
            .predecessor_account_id(accounts(0))
            .is_view(is_view);

        builder
    }

    #[test]
    fn test_init_contract() {
        let context = get_context(false);
        //TODO
        assert_eq!(true, true);
    }
}
