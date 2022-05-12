#!/usr/bin/env bash

echo "deleting subaccount"
near delete nft-v00.escrow-ftl.testnet escrow-ftl.testnet || true
echo "re creating subaccount"
near create-account nft-v00.escrow-ftl.testnet --masterAccount escrow-ftl.testnet --initialBalance 50
echo "building and deploying"
./build.sh
./deploy.sh nft-v00.escrow-ftl.testnet