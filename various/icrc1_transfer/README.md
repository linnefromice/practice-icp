# icrc1_transfer

```bash
./setup.sh
PRINCIPAL_ID=$(dfx identity get-principal --identity default)
dfx canister call icrc1-ledger1 --identity default \
  icrc1_balance_of "(record { owner=principal \"${PRINCIPAL_ID}\" })"
dfx canister call icrc1-ledger2 --identity default \
  icrc1_balance_of "(record { owner=principal \"${PRINCIPAL_ID}\" })"
dfx canister call icrc1-ledger1 icrc1_transfer "(record {
  to = record {owner = principal \"${PRINCIPAL_ID}\" };
  amount=1_000_000
})"
dfx canister call icrc1-ledger2 icrc1_transfer "(record {
  to = record {owner = principal \"${PRINCIPAL_ID}\" };
  amount=2_000_000
})"
```
