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
echo "> dfx deploy"
dfx deploy
echo "> cd .."
cd ..
echo "> yarn hardhat node --port 18545"
yarn hardhat node --port 18545
