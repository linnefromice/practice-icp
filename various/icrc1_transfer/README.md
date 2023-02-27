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
