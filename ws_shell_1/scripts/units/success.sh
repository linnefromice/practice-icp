#!/bin/bash

echo "This is success.sh!!!"

dfx canister call backend get --query
dfx canister call backend set '(50)'

dfx canister call backend greet '("Succeeded!!!")' --query
