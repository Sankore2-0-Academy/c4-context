#!/bin/bash 

source ./scripts/setting.conf

# Get Account Ids
# near call $SUB_ACCOUNT get_wallet '{}' --accountId <signer-account-id>

# Get Signer's public key
# near call $SUB_ACCOUNT get_pk '{}' --accountId <signer-account-id>

# Get Gas info
# near call $SUB_ACCOUNT get_gas '{}' --accountId <signer-account-id> --gas=300000000000000

# Get Deposit
# near call $SUB_ACCOUNT get_deposit '{}' --accountId <signer-account-id> --amount=2

# Storage info
# near call $SUB_ACCOUNT get_storage_info '{}' --accountId=<signer-account-id>

# Balance
# near call $SUB_ACCOUNT get_balance '{}' --accountId=<signer-account-id>

# Block info
# near call $SUB_ACCOUNT get_block_id '{}' --accountId=<signer-account-id>

# Account validity
near call $SUB_ACCOUNT is_valid_account '{"account_id": "jhtdgte.testnet"}' --accountId=<signer-account-id>