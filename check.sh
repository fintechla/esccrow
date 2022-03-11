#!/usr/bin/env bash

echo "deleting subaccount 04"
near delete esccrow.testaccount14.testnet testaccount14.testnet
echo "re creating subaccount 04"
near create-account esccrow.testaccount14.testnet --masterAccount testaccount14.testnet
echo "building and deploying"
./build.sh
./deploy.sh esccrow.testaccount14.testnet