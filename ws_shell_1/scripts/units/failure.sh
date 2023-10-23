#!/bin/bash

echo "This is failure.sh..."

dfx canister call backend get --query
dfx canister call backend set '(-50)'

dfx canister call backend greet '("Failed...")' --query
