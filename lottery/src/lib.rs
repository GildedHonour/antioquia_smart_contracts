use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::TreeMap;
use near_sdk::{env, log, near_bindgen, require, AccountId, Balance, PanicOnDefault, Promise};

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
    Closed
}

#[derive(BorshDeserialize, BorshSerialize, PartialEq, Debug)]
pub enum PrizeStatus {
    DepositPending,
    DepositPartiallyFunded,
    DepositFunded,
    WinnerPayedOff,
    OwnerReimbursed,
    PartiallyPayedOffAndReimbursed,
}

#[derive(BorshDeserialize, BorshSerialize, PartialEq, Debug)]
pub enum ParticipantStatus {
    Active,
    Suspended
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct LotteryItem {
    pub lottery_id: LotteryId,
    pub status: Status,
    pub organiser_account_id: AccountId,
    pub participants: TreeMap<AccountId, Participant>,
    pub winner: Option<Participant>,
    pub agreed_prize_amount: Balance,
    pub current_prize_amount: Balance,
    pub prize_status: PrizeStatus,
    pub current_fee_percentage: u128,
    // pub amount_paid_off: bool,
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
            (base_fee_percentage2 >= Self::MIN_FEE_PERCENTAGE) && (base_fee_percentage2 <= Self::MAX_FEE_PERCENTAGE),
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
    pub fn new(
        &mut self,
        lottery_id: LotteryId,
        organiser_account_id: AccountId,
        agreed_prize_amount: Balance,
        current_fee_percentage: Option<u128>,
    ) -> Option<LotteryId> {
        require!(agreed_prize_amount > 0, "agreed_prize_amount must be greater than 0");

        let cond = (self.owner_account_id == env::predecessor_account_id()) || (organiser_account_id == env::predecessor_account_id());
        require!(cond, "only funder or owner of this escrow may call this method");

        if !self.items.contains_key(&lottery_id) {
            let pts: TreeMap<AccountId, Participant> = TreeMap::new(b"p");
            let new_item = LotteryItem {
                lottery_id: lottery_id.clone(),
                agreed_prize_amount,
                status: Status::Draft,
                prize_status: PrizeStatus::DepositPending,
                organiser_account_id,
                current_fee_percentage: current_fee_percentage.unwrap_or(self.base_fee_percentage),
                current_prize_amount: 0,
                participants: pts,
                winner: None
            };

            self.items.insert(&lottery_id.clone(), &new_item);
            Some(lottery_id)
        } else {
            log!("lottery_id '{}' already exists; generate a new one", lottery_id);
            None
        }
    }

    #[payable]
    pub fn deposit_funds(&mut self, lottery_id: LotteryId) -> Balance {
        let attached_deposit_amount: Balance = env::attached_deposit();
        require!(
            attached_deposit_amount > 0,
            format!("attached_deposit_amount must be greater than 0")
        );

        match self.items.get(&lottery_id) {
            Some(mut lottery) => {
                require!(
                    lottery.agreed_prize_amount == attached_deposit_amount,
                    format!(
                        "agreed_prize_amount and attached_deposit_amount must be equal: {} and {}",
                        lottery.agreed_prize_amount, attached_deposit_amount
                    )
                );

                lottery.current_prize_amount = attached_deposit_amount;
                lottery.prize_status = PrizeStatus::DepositFunded;
                lottery.status = Status::New;
                attached_deposit_amount
            },
            None => {
                log!("lottery with id '{}' doesn't exist", lottery_id);
                0
            }
        }
    }

    pub fn add_participant(&self,
        lottery_id: LotteryId,
        participant_account_id: AccountId
    ) -> Option<AccountId> {
        match self.items.get(&lottery_id) {
            Some(mut lottery) => {
                if lottery.participants.contains_key(&participant_account_id) {
                    log!("participant with account_id '{}' already exists", participant_account_id);
                    None
                } else {
                    let new_pt = Participant{
                        status: ParticipantStatus::Active
                    };

                    lottery.participants.insert(
                        &participant_account_id,
                        &new_pt,
                    );

                    Some(participant_account_id)
                }
            },
            None => {
                log!("lottery with id '{}' doesn't exist", lottery_id);
                None
            }
        }
    }

    //TODO
    pub fn pick_random_winner(&self, lottery_id: LotteryId) -> AccountId {
        todo!()
    }

    // Generate random u8 number (0-254)
    fn random_u8(&self, index: usize) -> u8 {
        *env::random_seed().get(index).unwrap()
    }

    // Get random number from 0 to max
    fn random_in_range(&self, index: usize, max: usize) -> u32 {
        let rand_divider = 256 as f64 / (max + 1) as f64;
        let result = self.random_u8(index) as f64 / rand_divider;
        result as u32
    }

    fn get_lottery(&self, lottery_id: LotteryId) -> TreeMap<String, String> {
        let mut tree: TreeMap<String, String> = TreeMap::new(b"i");
        match self.items.get(&lottery_id) {
            Some(lottery) => {
                tree.insert(
                    &String::from("lottery_id"), 
                    &String::from(lottery_id.clone())
                );

                tree.insert(
                    &String::from("organiser_account_id"), 
                    &String::from(lottery_id)
                );

                tree.insert(
                    &String::from("status"), 
                    &String::from(format!("{:?}", lottery.status))
                );

                tree.insert(
                    &String::from("agreed_prize_amount"), 
                    &String::from(lottery.agreed_prize_amount.to_string())
                );

                tree.insert(
                    &String::from("prize_status"), 
                    &String::from(format!("{:?}", lottery.prize_status))
                );

                tree.insert(
                    &String::from("fee_percentage"), 
                    &String::from(lottery.current_fee_percentage.to_string())
                );

                tree.insert(
                    &String::from("participants_count"), 
                    &String::from(format!("{:?}", lottery.participants.len()))
                );

                tree
            },
            None => {
                log!("lottery with id '{}' doesn't exist", lottery_id);
                tree
            }
        }
    }

    //TODO
    fn get_participant(&self,
        lottery_id: LotteryId,
        account_id: AccountId
    ) -> Option<Participant> {
      None
    }

    fn get_winner(&self, lottery_id: LotteryId) -> Option<Participant> {
        None
    }
}
