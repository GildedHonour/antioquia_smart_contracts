# Escrow smart contract

---
## methods

call `init()` method to initialize escrow on the global level; use it only once

```rust
fn init(
  owner_id: Option<AccountId>
  base_fee_percentage: Option<u128>
)

/*
wherein:
  * owner_id - an account of an owner/admin; if the value isn't not provided, the caller will become the one.
  * base_fee_percentage - the percentage of a fee that the owner/admin will receive off each deal
*/
```

to create a new deal, call `new()`; and attach `agreed_amount` of coins to it

```rust
#[payable]
fn new(
  escrow_id: EscrowId,
  funder_id: AccountId,
  beneficiary_id: AccountId,
  agreed_amount: Balance,
  current_fee_percentage: Option<u128>
)


/*
wherein:
  * escrow_id - random string that'll be ID of escrow
  * funder_id - account address of a funder/client
  * beneficiary_id - account address of the other party
  * agreed_amount - how much to deposit in escrow
  * current_fee_percentage - fee percentage in this escrow; if not provided, the base_fee_percentage will get used instead
*/
```

then either release escrow, if a deal has been finished successfully

```rust
fn release_deposit(
  escrow_id: EscrowId
)

/*
wherein:
  * escrow_id - escrow ID
*/
```

or reimburse the other party otherwise

```rust
fn reimburse_funder(
  escrow_id: EscrowId
)

/*
wherein:
  * escrow_id - escrow ID 
*/
```

other methods

```rust
fn get_base_fee_percentage() -> Balance
fn get_owner_id() -> AccountId
fn get_item(escrow_id: EscrowId) -> Option<EscrowItem>
fn set_base_fee_percentage(new_fee: Balance)

//withdraw all the coins of an escrow to the admin's/owner's account
//use it only when there's an urgent need to do it
fn emergency_withdraw(escrow_id: EscrowId)
```
