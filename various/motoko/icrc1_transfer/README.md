# icrc1_transfer

- memo
  - <https://forum.dfinity.org/t/inter-canister-query-calls-community-consideration/6754>
  - <https://forum.dfinity.org/t/simplest-best-way-to-make-inter-canister-calls-in-motoko/18058>

## commands

```bash
# Preparations
dfx identity new user-a
dfx identity new user-b

# Executions
./setup.sh
DEFAULT_PRINCIPAL_ID=$(dfx identity get-principal --identity default)
USERA_PRINCIPAL_ID=$(dfx identity get-principal --identity user-a)
USERB_PRINCIPAL_ID=$(dfx identity get-principal --identity user-b)

## check minted
dfx canister call icrc1-ledger1 --identity default \
  icrc1_balance_of "(record { owner=principal \"${DEFAULT_PRINCIPAL_ID}\" })"
dfx canister call icrc1-ledger2 --identity default \
  icrc1_balance_of "(record { owner=principal \"${DEFAULT_PRINCIPAL_ID}\" })"

## transfer: default -> a (mint?)
dfx canister call icrc1-ledger1 --identity default \
  icrc1_transfer "(record {
    to = record {owner = principal \"${USERA_PRINCIPAL_ID}\" };
    amount=1_000_000
  })"
dfx canister call icrc1-ledger2 --identity default \
  icrc1_transfer "(record {
    to = record {owner = principal \"${USERA_PRINCIPAL_ID}\" };
    amount=2_000_000
  })"
### check
dfx canister call icrc1-ledger1 --identity default \
  icrc1_balance_of "(record { owner=principal \"${DEFAULT_PRINCIPAL_ID}\" })"
dfx canister call icrc1-ledger2 --identity default \
  icrc1_balance_of "(record { owner=principal \"${DEFAULT_PRINCIPAL_ID}\" })"
dfx canister call icrc1-ledger1 --identity default \
  icrc1_balance_of "(record { owner=principal \"${USERA_PRINCIPAL_ID}\" })"
dfx canister call icrc1-ledger2 --identity default \
  icrc1_balance_of "(record { owner=principal \"${USERA_PRINCIPAL_ID}\" })"

## transfer: a -> b
dfx canister call icrc1-ledger1 --identity user-a \
  icrc1_transfer "(record {
    to = record {owner = principal \"${USERB_PRINCIPAL_ID}\" };
    amount=250_000
  })"
dfx canister call icrc1-ledger2 --identity user-a \
  icrc1_transfer "(record {
    to = record {owner = principal \"${USERB_PRINCIPAL_ID}\" };
    amount=500_000
  })"
### check
dfx canister call icrc1-ledger1 --identity default \
  icrc1_balance_of "(record { owner=principal \"${USERA_PRINCIPAL_ID}\" })"
dfx canister call icrc1-ledger2 --identity default \
  icrc1_balance_of "(record { owner=principal \"${USERA_PRINCIPAL_ID}\" })"
dfx canister call icrc1-ledger1 --identity default \
  icrc1_balance_of "(record { owner=principal \"${USERB_PRINCIPAL_ID}\" })"
dfx canister call icrc1-ledger2 --identity default \
  icrc1_balance_of "(record { owner=principal \"${USERB_PRINCIPAL_ID}\" })"
```

```bash
./setup.sh
dfx deploy icrc1_transfer_backend
dfx canister call icrc1_transfer_backend --identity default \
  token_info "principle_id for ledger canister"
```

examples

```bash
URLs:
  Backend canister via Candid interface:
    icrc1-ledger1: http://127.0.0.1:4943/?canisterId=ryjl3-tyaaa-aaaaa-aaaba-cai&id=rrkah-fqaaa-aaaaa-aaaaq-cai
    icrc1-ledger2: http://127.0.0.1:4943/?canisterId=ryjl3-tyaaa-aaaaa-aaaba-cai&id=r7inp-6aaaa-aaaaa-aaabq-cai
    icrc1_transfer_backend: http://127.0.0.1:4943/?canisterId=ryjl3-tyaaa-aaaaa-aaaba-cai&id=rkp4c-7iaaa-aaaaa-aaaca-cai
% dfx canister call icrc1_transfer_backend --identity default \
  token_info "rrkah-fqaaa-aaaaa-aaaaq-cai"
("Polygon MATIC", "MATIC", 8 : nat8)
% dfx canister call icrc1_transfer_backend --identity default \
  token_info "r7inp-6aaaa-aaaaa-aaabq-cai"
("Cosmos ATOM", "ATOM", 8 : nat8)
```

```bash
% dfx canister call icrc1_transfer_backend --identity user-a \
  transfer "(\"rrkah-fqaaa-aaaaa-aaaaq-cai\", \"${USERB_PRINCIPAL_ID}\", 250000)"
```

## memo

- icrc-1
  - <https://internetcomputer.org/docs/current/developer-docs/integrations/icrc-1/>
    - <https://github.com/dfinity/ICRC-1/tree/main/standards/ICRC-1>
    - how to deploy
      - <https://github.com/dfinity/ic/tree/master/rs/rosetta-api/icrc1/ledger#step-2-start-the-replica>
      - <https://internetcomputer.org/docs/current/developer-docs/integrations/icrc-1/deploy-new-token>
    - example (full implementation): <https://github.com/NatLabs/icrc1>
- inter-canister calls
  - <https://forum.dfinity.org/t/simplest-best-way-to-make-inter-canister-calls-in-motoko/18058>
  - <https://forum.dfinity.org/t/inter-canister-query-calls-community-consideration/6754>
- flow: transfer
  - <https://internetcomputer.org/docs/current/developer-docs/integrations/ledger/interact-with-ledger/>
  - currently cannot transfer, so use notifacton
    - <https://drive.google.com/drive/u/0/folders/1TlaDISjZpAKpqJdXzYMw4hhuKj5YxZ3J>
    - ICRC-2: Approve and Transfer From
      - <https://github.com/dfinity/ICRC-1/tree/main/standards/ICRC-2>
    - ICRC-3: A Standard for Accessing the Transaction Log
      - <https://github.com/dfinity/ICRC-1/tree/roman-icrc3/standards/ICRC-3>

### tokens

- [ICP (NNS Ledger)](https://dashboard.internetcomputer.org/canister/ryjl3-tyaaa-aaaaa-aaaba-cai)
- [SNS-1 (SNS-1 Ledger)](https://dashboard.internetcomputer.org/canister/zfcdd-tqaaa-aaaaq-aaaga-cai)
- [ckBTC](https://dashboard.internetcomputer.org/canister/mxzaz-hqaaa-aaaar-qaada-cai)
