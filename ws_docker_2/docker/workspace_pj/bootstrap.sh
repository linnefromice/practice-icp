#!/usr/bin/env bash
echo "> ls -l"
ls -l
echo "> ls -l artifacts"
ls -l artifacts
echo "> cd artifacts"
cd artifacts
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
echo "> cd .."
cd ..
echo "> yarn hardhat node --port 18545"
yarn hardhat node --port 18545
