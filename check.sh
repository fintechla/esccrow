#!/usr/bin/env bash

echo "deleting subaccount 04"
near delete subaccount04.testaccount14.testnet testaccount14.testnet
echo "re creating subaccount 04"
near create-account subaccount04.testaccount14.testnet --masterAccount testaccount14.testnet
echo "building and deploying"
./build.sh
./deploy.sh subaccount04.testaccount14.testnet