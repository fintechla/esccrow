#!/usr/bin/env bash

echo "deleting subaccount 04"
near delete esccrow.testaccount15.testnet testaccount15.testnet || true
echo "re creating subaccount 04"
near create-account esccrow.testaccount15.testnet --masterAccount testaccount15.testnet --initialBalance 50
echo "building and deploying"
./build.sh
./deploy.sh esccrow.testaccount15.testnet