# Lottery smart contract

---
## methods

call `init()` method to initialize lottery on the global level; use it only once

```rust
fn init(
  owner_account_id: Option<AccountId>
  base_fee_percent: Option<u128>
)

/*
wherein:
  * owner_id - an account of an owner/admin; if the value isn't not provided, the caller will become the one.
  * base_fee_percent - percent of a fee that the owner/admin will receive off each deal
*/
```

to create a new lottery:

```rust
#[payable]
fn new(
    lottery_id: LotteryId,
    organiser_account_id: AccountId,
    agreed_prize_amount: Balance,
    current_fee_percent: Option<u128>,
)
/*
wherein:
  * lottery_id - random string that'll be ID of lottery
  * organiser_account_id - account address of a organiser
  * agreed_prize_amount - how much is the prize
  * current_fee_percentage - fee percentage in this escrow; if not provided, the base_fee_percentage will get used instead
*/
```

to add a new participant, or opt out an existing one:

```rust
fn add_participant(
    lottery_id: LotteryId,
    participant_account_id: AccountId
)

fn opt_out_participant(
    lottery_id: LotteryId,
    participant_account_id: AccountId
)

/*
wherein:
  * lottery_id - random string that'll be ID of lottery
  * participant_account_id - account address of a participant
*/
```

to pick a winner, randomly; or to get one:
```rust
fn pick_random_winner(
    lottery_id: LotteryId
)

fn get_winner(
    lottery_id: LotteryId
)
```

to send the prize to the winner:

```rust
fn release_prize_to_winner(
    lottery_id: LotteryId
)
```

to get a lottery info:

```rust
fn get_lottery(
    lottery_id: LotteryId
)
```

to get the current balance/prize of lottery; if the whole prize has been sent to the winner, the balance becomes 0;
```rust
fn get_current_balance(
    lottery_id: LotteryId
)
```