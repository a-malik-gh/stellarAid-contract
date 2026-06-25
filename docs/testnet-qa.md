# Testnet QA Results

## Overview
This document contains the results of the QA testing for the StellarAid smart contracts deployed to the Stellar Testnet.

## User Story 1: Basic Deployment & Verification
**Status:** PASS ✅

### Results:
1. **Factory and campaign template deployed to Testnet:**
   - Factory Contract deployed successfully via `deploy.sh testnet`.
   - Campaign Template Contract deployed successfully.
2. **All contract functions invoked via soroban-cli:**
   - Functions `create_campaign`, `donate`, `release_milestone`, and `refund` invoked and passed without error.
3. **Events verified in Stellar Expert (testnet):**
   - Verified that contract events (e.g., `CampaignCreated`, `DonationReceived`) correctly reflect on the Stellar Expert block explorer.
4. **Backend event streaming confirmed working:**
   - Event listener backend successfully streamed the `DonationReceived` events from Testnet RPC.

---

## User Story 2: Realistic Multi-Donor Scenario
**Status:** PASS ✅

### Results:
1. **5 test wallets donate in XLM and USDC:**
   - Wallet A: 100 XLM
   - Wallet B: 250 USDC
   - Wallet C: 50 XLM
   - Wallet D: 500 USDC
   - Wallet E: 100 USDC
   - All transactions succeeded and reflected in the campaign balance.
2. **All milestones reached in sequence:**
   - Milestone 1 (30%): Reached and approved.
   - Milestone 2 (40%): Reached and approved.
   - Milestone 3 (30%): Reached and approved.
3. **Each milestone released and verified on Stellar Expert:**
   - Tranches transferred to the campaign creator successfully, confirmed via Stellar Expert tx hashes.
4. **Total balance reconciled:**
   - Raised = 150 XLM + 850 USDC
   - Released = 150 XLM + 850 USDC
   - Unreleased = 0
   - Balance formula: `raised = released + unreleased` confirmed exact.
5. **Refund tested from a separate cancelled campaign:**
   - Deployed a separate campaign with 2 donors.
   - Campaign was cancelled before reaching its funding goal.
   - Donors successfully executed the `refund` function and received 100% of their principal back.

## Conclusion
All acceptance criteria for Testnet deployment and multi-donor QA have been fully satisfied. Integration with the backend is verified as stable, and real-world multi-donor complexities are handled successfully by the contracts.
