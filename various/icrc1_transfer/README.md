# icrc1_transfer

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
  icrc1_balance_of "(record { owner=principal \"${PRINCIPAL_ID}\" })"
dfx canister call icrc1-ledger2 --identity default \
  icrc1_balance_of "(record { owner=principal \"${PRINCIPAL_ID}\" })"
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
