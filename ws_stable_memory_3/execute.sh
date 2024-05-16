#!/bin/bash

# for i in $(seq 1 500000000)
for i in $(seq 1 1000)
do
  echo "> Exec: $i"
  dfx canister --ic call backend_1 insert_snapshot_bulk "(100, 100000)"
  dfx canister --ic call backend_1 snapshots_len
  dfx canister --ic call backend_1 current_used_memory
done
