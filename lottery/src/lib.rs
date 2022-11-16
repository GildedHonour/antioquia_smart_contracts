use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::TreeMap;
use near_sdk::{env, log, near_bindgen, require, AccountId, Balance, PanicOnDefault, Promise};
use std::collections::BTreeMap;

//TODO replace with the proper type - UUID
type LotteryId = String;

#[derive(BorshDeserialize, BorshSerialize, PartialEq, Debug)]
pub enum Status {
    Draft,
    Canceled,
    New,
    // NOTE
    // when and if a need to open and close
    // registration at certain time arises,
    // uncomment this:
    //
    // RegistrationOpen,
    // RegistrationClosed,
    //
    Active,
    Over,
    Closed,
}

#[derive(BorshDeserialize, BorshSerialize, PartialEq, Debug)]
pub enum PrizeStatus {
    DepositPending,
    DepositFunded,
    WinnerPayedOff,
    OwnerReimbursed,
    //PartiallyPayedOffAndReimbursed,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub enum ParticipantStatus {
    Active,
    OptedOut,
    Suspended,
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct LotteryItem {
    pub lottery_id: LotteryId,
    pub status: Status,
    pub organiser_account_id: AccountId,

    // use the standard BTreeMap in order to avoid generating 'prefix'
    // pub participants: TreeMap<AccountId, Participant>,
    pub participants: BTreeMap<AccountId, Participant>,

    pub winner: Option<AccountId>,
    pub agreed_prize_amount: Balance,
    pub current_prize_amount: Balance,
    pub prize_status: PrizeStatus,
    pub current_fee_percentage: u128,
    // pub amount_paid_off: bool,
    // pub started_at: u128
    // pub ended_at: u128
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Participant {
    pub status: ParticipantStatus,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Lottery {
    pub owner_account_id: AccountId,
    pub base_fee_percentage: u128,
    pub items: TreeMap<LotteryId, LotteryItem>,
}

#[near_bindgen]
impl Lottery {
    const MIN_FEE_PERCENTAGE: u128 = 0;
    const MAX_FEE_PERCENTAGE: u128 = 100;
    const HUNDRED_PERCENT: u128 = 100;

    #[init]
    pub fn init(_owner_account_id: Option<AccountId>, base_fee_percentage: Option<u128>) -> Self {
        require!(!env::state_exists(), "Already initialized");
        let owner_account_id = _owner_account_id.unwrap_or(env::signer_account_id());
        let base_fee_percentage2 = base_fee_percentage.unwrap_or(Self::MIN_FEE_PERCENTAGE);
        require!(
            (base_fee_percentage2 >= Self::MIN_FEE_PERCENTAGE)
                && (base_fee_percentage2 <= Self::MAX_FEE_PERCENTAGE),
            format!(
                "base_fee_percentage must be between {}..{}",
                &Self::MIN_FEE_PERCENTAGE,
                &Self::MAX_FEE_PERCENTAGE
            )
        );

        let items: TreeMap<LotteryId, LotteryItem> = TreeMap::new(b"t");
        Self {
            owner_account_id,
            base_fee_percentage: base_fee_percentage2,
            items,
        }
    }

    ///creates a new lottery
    ///and deposits funds
    #[payable]
    pub fn new(
        &mut self,
        lottery_id: LotteryId,
        organiser_account_id: AccountId,
        agreed_prize_amount: Balance,
        current_fee_percentage: Option<u128>,
    ) -> Option<LotteryId> {
        require!(
            agreed_prize_amount > 0,
            "agreed_prize_amount must be greater than 0"
        );
        let attached_deposit_amount: Balance = env::attached_deposit();
        require!(
            attached_deposit_amount > 0,
            format!("attached_deposit_amount must be greater than 0")
        );

        let cond = (self.owner_account_id == env::predecessor_account_id())
            || (organiser_account_id == env::predecessor_account_id());
        require!(
            cond,
            "only funder or owner of this lottery may call this method"
        );

        if !self.items.contains_key(&lottery_id) {
            require!(
                agreed_prize_amount == attached_deposit_amount,
                format!(
                    "agreed_prize_amount and attached_deposit_amount must be equal: {} and {}",
                    agreed_prize_amount, attached_deposit_amount
                )
            );

            //FIXME: generate prefixes dynamically
            // let pts: TreeMap<AccountId, Participant> = TreeMap::new(lottery_id.as_bytes());
            let pts: BTreeMap<AccountId, Participant> = BTreeMap::new();

            let new_item = LotteryItem {
                lottery_id: lottery_id.clone(),
                agreed_prize_amount,
                status: Status::New,
                prize_status: PrizeStatus::DepositFunded,
                organiser_account_id,
                current_fee_percentage: current_fee_percentage.unwrap_or(self.base_fee_percentage),
                current_prize_amount: attached_deposit_amount,
                participants: pts,
                winner: None,
            };

            self.items.insert(&lottery_id.clone(), &new_item);
            Some(lottery_id)
        } else {
            log!(
                "lottery_id '{}' already exists; generate a new one",
                lottery_id
            );
            None
        }
    }

    pub fn add_participant(
        &mut self,
        lottery_id: LotteryId,
        participant_account_id: AccountId,
    ) -> Option<AccountId> {
        let mut lottery = self.items.get(&lottery_id).unwrap();
        if lottery.participants.contains_key(&participant_account_id) {
            log!(
                "participant with account_id '{}' already exists",
                participant_account_id
            );
            None
        } else {
            let new_pt = Participant {
                status: ParticipantStatus::Active,
            };

            lottery
                .participants
                .insert(participant_account_id.clone(), new_pt);

            //re-insert the current lottery item
            //this is required in order make the collection update itself
            self.items.insert(&lottery_id, &lottery);

            Some(participant_account_id.clone())
        }
    }

    //TODO: can be improved
    pub fn pick_random_winner(&mut self, lottery_id: LotteryId) -> AccountId {
        const MIDDLE: usize = 16;

        let mut lottery = self.items.get(&lottery_id).unwrap();
        let wn = lottery.winner;
        require!(wn.is_none(), format!("winner had already been chosen before: {}", wn.clone().unwrap()));

        let account_ids: Vec<AccountId> = lottery
            .participants
            .iter()
            .map(|(k, _)| (*k).clone())
            .collect();



        let rnd1 = self.random_in_range(MIDDLE, account_ids.len());
        let rnd_account_id = account_ids.get(rnd1 as usize).unwrap();
        lottery.winner = Some(rnd_account_id.clone());
        log!(
            "lottery_id: {}, random number: {}, the winner: {}",
            lottery_id,
            rnd1,
            rnd_account_id
        );

        //re-insert the current lottery item
        //this is required in order make the collection update itself
        self.items.insert(&lottery_id, &lottery);
        rnd_account_id.clone()
    }

    // returns random u8 number (0-254)
    fn random_u8(&self, index: usize) -> u8 {
        *env::random_seed().get(index).unwrap()
    }

    // returns random number from 0 to max
    fn random_in_range(&self, index: usize, max: usize) -> u32 {
        let rand_divider = 256 as f64 / (max + 1) as f64;
        let result = self.random_u8(index) as f64 / rand_divider;
        result as u32
    }

    fn get_lottery(&self, lottery_id: LotteryId) -> TreeMap<String, String> {
        let mut tree: TreeMap<String, String> = TreeMap::new(b"i");
        let lottery = self.items.get(&lottery_id).unwrap();
        tree.insert(
            &String::from("lottery_id"),
            &String::from(lottery_id.clone()),
        );

        tree.insert(
            &String::from("organiser_account_id"),
            &String::from(lottery_id),
        );

        tree.insert(
            &String::from("status"),
            &String::from(format!("{:?}", lottery.status)),
        );

        tree.insert(
            &String::from("agreed_prize_amount"),
            &String::from(lottery.agreed_prize_amount.to_string()),
        );

        tree.insert(
            &String::from("prize_status"),
            &String::from(format!("{:?}", lottery.prize_status)),
        );

        let winner_key = String::from("winner_account_id");
        let winner_val = match lottery.winner {
            Some(acc_id) => String::from(acc_id),
            None => String::from("none"),
        };
        tree.insert(&winner_key, &winner_val);

        tree.insert(
            &String::from("fee_percentage"),
            &String::from(lottery.current_fee_percentage.to_string()),
        );

        tree.insert(
            &String::from("participants_count"),
            &String::from(format!("{:?}", lottery.participants.len())),
        );

        tree
    }

    pub fn get_winner(&self, lottery_id: LotteryId) -> Option<AccountId> {
        self.items.get(&lottery_id).unwrap().winner
    }

    //releases the prize to the winner
    pub fn release_prize_to_winner(&mut self, lottery_id: LotteryId) {
        let mut lottery_item = self.items.get(&lottery_id).unwrap();
        let amount_for_winner = lottery_item.agreed_prize_amount / Self::HUNDRED_PERCENT
            * (Self::HUNDRED_PERCENT - lottery_item.current_fee_percentage);
        let amount_for_owner = lottery_item.agreed_prize_amount - amount_for_winner;

        //due to a potential rounding error,
        //verify that there'll be enough of the funds
        let amounts_sum = amount_for_winner + amount_for_owner;
        let calc_cond = lottery_item.current_prize_amount >= amounts_sum;
        require!(calc_cond, format!("current_prize_amount ({}) must be equal to or greater than the sum of the amounts to be released ({});", lottery_item.current_prize_amount, amounts_sum));

        let winner_account_id = lottery_item.winner.unwrap();
        //send funds to the winner
        let p1 = Promise::new(winner_account_id.clone()).transfer(amount_for_winner);
        lottery_item.current_prize_amount -= amount_for_winner;
        log!(
            "releasing '{}' to winner '{}'; lottery_id '{}'",
            amount_for_winner,
            winner_account_id,
            lottery_id
        );

        //send the fees to the owner
        let p2 = Promise::new(self.owner_account_id.clone()).transfer(amount_for_owner);
        p1.then(p2);
        //FIXME verify that _p1 has returned successfully
        lottery_item.current_prize_amount -= amount_for_owner;
        log!(
            "[lottery_id '{}'] sending commission of '{}' ({}%) to owner_account_id '{}'",
            lottery_id,
            amount_for_owner,
            lottery_item.current_fee_percentage,
            self.owner_account_id
        );

        lottery_item.prize_status = PrizeStatus::WinnerPayedOff;
    }

    /// returns the balance or prize of a LotteryItem
    pub fn get_balance(&self, lottery_id: LotteryId) -> Balance {
        let item = self.items.get(&lottery_id).unwrap();
        require!(item.agreed_prize_amount == item.current_prize_amount);
        item.agreed_prize_amount
    }
}
