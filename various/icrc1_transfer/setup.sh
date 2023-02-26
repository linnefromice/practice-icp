#!/usr/bin/env bash

deploy_icrc1_ledger () {
  canister_name=$1
  owner=$2
  controller=$3
  token_name=$4
  token_symbol=$5
  echo "canister_name: $canister_name"
  echo "owner: $owner"
  echo "controller: $controller"
  echo "token_name: $token_name"
  echo "token_symbol: $token_symbol"

  dfx deploy $canister_name --argument "(record {
    token_name = \"$token_name\";
    token_symbol = \"$token_symbol\";
    minting_account = record { owner = principal \"$owner\" };
    initial_balances = vec {};
    metadata = vec {};
    transfer_fee = 10;
    archive_options = record {
      num_blocks_to_archive = 1000;
      trigger_threshold = 2000;
      controller_id = principal \"$controller\";
    };
  })"
}

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

deploy_icrc1_ledger icrc1-ledger1 \
  $DEFAULT_PRINCIPAL_ID $DEFAULT_PRINCIPAL_ID "Polygon MATIC" "MATIC"

deploy_icrc1_ledger icrc1-ledger2 \
  $DEFAULT_PRINCIPAL_ID $DEFAULT_PRINCIPAL_ID "Cosmos ATOM" "ATOM"
