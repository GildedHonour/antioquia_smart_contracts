# Escrow smart contract


## How to compile it and call its methods

#### prepare
compile the smart contract
```bash
env 'RUSTFLAGS=-C link-arg=-s' 
cargo build --target wasm32-unknown-unknown --release
```

install the NEAR CLI
```bash
npm i near-cli
```

log in; your account will be created in `/home/{user123}/.near-credentials/{testnet|mainnet}/`

```bash
near login
```

deploy the contract
```bash
# (1)
near deploy --wasmFile target/wasm32-unknown-unknown/release/antioquia_escrow.wasm --accountId antioquia.testnet

# (2) or, from a different account
near deploy --wasmFile target/wasm32-unknown-unknown/release/antioquia_escrow.wasm --accountId escrow.antioquia.testnet

# (3) or, from a dev account which will be created in the current directory
near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/antioquia_escrow.wasm

# (4) deploy and call the 'init' method (special method) at once
near deploy --wasmFile out/example.wasm --accountId example-contract.testnet --initFunction new --initArgs '{"owner_id": "example-contract.testnet", "total_supply": "10000000"}'
```

alternatively, build and deployt it by the scripts
```
./build.sh
./deploy.sh
```

### use
call a method
```bash
near call <CONTRACT_ID> <METHOD1> '{"arg1": "val1", "arg2" : 666}' --accountId <ACCOUNT_ID>

# for example
# create a new escrow
# notice '--amount <....>' parameter
# which means that some $coins are being sent to the contract
near call escrow.antioquia.testnet new '{"escrow_id": "1aa", "funder_id": "11", "beneficiary_id": "22", "agreed_amount": 555}' --amount 0.000000000000000000000010 --accountId antioquia.testnet 

# set fees (you must be the owner/admin)
near call escrow.antioquia.testnet set_base_fee_percentage '{"new_fee": 3}' --accountId escrow.antioquia.testnet
```

call a view method
```bash
near view escrow.antioquia.testnet get_base_fee_percentage --accountId antioquia.testnet
```

---
links:

* https://docs.near.org/tools/near-cli
* https://learn.figment.io/tutorials/write-and-deploy-a-smart-contract-on-near

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

whenever there's a need to create a new deal, call `new()` to create an escrow and attach the `agreed_amount` of coins to it

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
