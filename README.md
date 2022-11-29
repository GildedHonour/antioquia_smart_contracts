# Antioquia smart contracts

## Escrow

on testnet: [escrow.antioquia.testnet](https://explorer.testnet.near.org/accounts/escrow.antioquia.testnet)

[description](escrow/README.md)


## Lottery

on testnet: [lottery.antioquia.testnet](https://explorer.testnet.near.org/accounts/lottery.antioquia.testnet)

[description](lottery/README.md)


## Donation

on testnet: [donation.antioquia.testnet](https://explorer.testnet.near.org/accounts/donation.antioquia.testnet)

[description](donation/README.md)

---

## How to compile a contract and call its methods

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

deploy a contract, with the **escrow** contract as an example:
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

alternatively, build and deployt it by script
```bash
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
