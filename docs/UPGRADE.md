# Contract Upgrade Guide

## Overview

The campaign, donation, and withdrawal contracts now support a guarded initialization flow and an admin-only upgrade entry point.

## Upgrade process

1. Build a new WASM artifact for the contract.
2. Obtain the new WASM hash from the build output.
3. Call the contract's `upgrade(env, admin, new_wasm_hash)` entry point using an authorized admin address.
4. Verify the deployment on testnet by invoking a read-only entry point after the upgrade.

## Notes

- Initialization can only be performed once per contract instance.
- The admin address is stored in instance storage and is required for upgrade operations.
- Avoid logging secrets or private keys in application logs.
