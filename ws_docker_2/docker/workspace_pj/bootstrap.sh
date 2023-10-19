#!/usr/bin/env bash
echo "> ls -l"
ls -l
## for dfx
echo "> ls -l artifacts"
ls -l artifacts
echo "> cp -rp artifacts ws_dfx"
cp -rp artifacts ws_dfx
echo "> cd ws_dfx"
cd ws_dfx
echo "> pwd"
pwd
echo "> dfx start --background --clean"
dfx start --background --clean
echo "> dfx canister create --all"
dfx canister create --all
echo "> dfx build"
dfx build
echo "> dfx canister install --all"
dfx canister install --all
echo "> check dashboard url (dfx deploy)"
dfx deploy
echo "> collect deliverables from deployments"
mkdir -p ../outputs/.dfx
cp -rp .env ../outputs
cp -rp .dfx/local ../outputs/.dfx
## for hardhat
echo "> cd .."
cd ..
echo "> yarn hardhat node --port 18545"
yarn hardhat node --port 18545