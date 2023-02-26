#!/usr/bin/env bash

echo "##### SETUP #####"

echo "# pre-process"
dfx stop

echo "# download"
export IC_VERSION=b43543ce7365acd1720294e701e8e8361fa30c8f
mkdir -p resource-icrc1 && cd resource-icrc1
test -f ic-icrc1-ledger.wasm.gz || curl -o ic-icrc1-ledger.wasm.gz "https://download.dfinity.systems/ic/$IC_VERSION/canisters/ic-icrc1-ledger.wasm.gz"
test -f ic-icrc1-ledger.wasm || gunzip ic-icrc1-ledger.wasm.gz
test -f icrc1.did || curl -o icrc1.did "https://raw.githubusercontent.com/dfinity/ic/$IC_VERSION/rs/rosetta-api/icrc1/ledger/icrc1.did"
cd ..

echo "# dfx"
dfx start --background --clean

export DEFAULT_PRINCIPAL_ID=$(dfx identity get-principal --identity default)
echo "principal-id to use: $DEFAULT_PRINCIPAL_ID"

export TOKEN_NAME="Polygon MATIC"
export TOKEN_SYMBOL="MATIC"
dfx deploy icrc1-ledger1 --argument "(record {
  token_name = \"${TOKEN_NAME}\";
  token_symbol = \"${TOKEN_SYMBOL}\";
  minting_account = record { owner = principal \"${DEFAULT_PRINCIPAL_ID}\" };
  initial_balances = vec {};
  metadata = vec {};
  transfer_fee = 10;
  archive_options = record {
    num_blocks_to_archive = 1000;
    trigger_threshold = 2000;
    controller_id = principal \"${DEFAULT_PRINCIPAL_ID}\";
  };
})"

export TOKEN_NAME="Cosmos ATOM"
export TOKEN_SYMBOL="ATOM"
dfx deploy icrc1-ledger2 --argument "(record {
  token_name = \"${TOKEN_NAME}\";
  token_symbol = \"${TOKEN_SYMBOL}\";
  minting_account = record { owner = principal \"${DEFAULT_PRINCIPAL_ID}\" };
  initial_balances = vec {};
  metadata = vec {};
  transfer_fee = 10;
  archive_options = record {
    num_blocks_to_archive = 1000;
    trigger_threshold = 2000;
    controller_id = principal \"${DEFAULT_PRINCIPAL_ID}\";
  };
})"
