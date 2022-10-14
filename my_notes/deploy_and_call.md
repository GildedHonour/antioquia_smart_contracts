#How to compile and call smart contracts

compile
```bash
env 'RUSTFLAGS=-C link-arg=-s' 
cargo build --target wasm32-unknown-unknown --release
```

install
```bash
npm i near-cli
```

log in
```bash
near login
```
your account will be created in `/home/{user123}/.near-credentials/{testnet|mainnet}/`

deploy it
```bash
//1
near deploy --wasmFile target/wasm32-unknown-unknown/release/antioquia_escrow.wasm --accountId antioquia.testnet

//or, from a different account
near deploy --wasmFile target/wasm32-unknown-unknown/release/antioquia_escrow.wasm --accountId escrow.antioquia.testnet

//2 deploy and call the 'init' method (special method) at once
//near deploy --wasmFile out/example.wasm --accountId example-contract.testnet --initFunction new --initArgs '{"owner_id": "example-contract.testnet", "total_supply": "10000000"}'
```

call a method
```bash
near call <CONTRACT_ID> <METHOD1> '{"arg1": "val1", "arg2" : 666}' --accountId <ACCOUNT_ID>

//for example
//create a new escrow
near call escrow.antioquia.testnet new '{"escrow_id": "1aa", "funder_id": "11", "beneficiary_id": "22", "agreed_amount": 555}' --accountId antioquia.testnet

//set fees (you must be the owner/admin)
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
