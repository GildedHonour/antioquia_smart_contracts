# Donation smart contract

---
## methods

call `init()` method to initialize escrow on the global level; use it only once

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

to send a donation:

```rust
#[payable]
send(
    donation_id: DonationId,
    receiver_account_id: AccountId,
    agreed_amount: Balance,
)

/*
wherein:
  * donation_id - random string that'll be ID of a donation
  * receiver_account_id - account address of the other party
  * amount - how much to send to the receiver
  * current_fee_percentage - fee percentage in this donation; if not provided, the base_fee_percentage will get used instead
*/
```

to get information about a donation:
```rust
get_donation(
  donation_id: DonationId
)
```