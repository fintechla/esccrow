#!/usr/bin/env bash

echo "deleting subaccount"
near delete nft-v01-400-pre-5.escrow-ftl.testnet escrow-ftl.testnet || true
echo "re creating subaccount"
    near create-account nft-v01-400-pre-5.escrow-ftl.testnet --masterAccount escrow-ftl.testnet --initialBalance 6
echo "building and deploying"
./build.sh
./deploy.sh nft-v1-400-pre-5.escrow-ftl.testnet