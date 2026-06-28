#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Symbol, Vec};
use shared::types::Withdrawal;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Withdrawal(u64) = 0,
    WithdrawalsByCampaign(u64) = 1,
    Admin = 2,
}

#[contracttype]
#[derive(Clone)]
pub struct WithdrawalRequestedEvent {
    pub campaign_id: u64,
    pub recipient: Address,
    pub amount: i128,
}

#[contracttype]
#[derive(Clone)]
pub struct WithdrawalApprovedEvent {
    pub withdrawal_id: u64,
}

#[contracttype]
#[derive(Clone)]
pub struct WithdrawalRejectedEvent {
    pub withdrawal_id: u64,
    pub reason: String,
}

#[contract]
pub struct WithdrawalContract;

#[contractimpl]
impl WithdrawalContract {
    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    pub fn request_withdrawal(env: Env, campaign_id: u64, owner: Address, amount: i128, recipient: Address) -> u64 {
        owner.require_auth();
        let id = Self::next_withdrawal_id(&env);
        let withdrawal = Withdrawal {
            campaign_id,
            recipient: recipient.clone(),
            amount,
            approved: false,
        };
        env.storage().persistent().set(&DataKey::Withdrawal(id), &withdrawal);
        let mut withdrawals = env.storage().persistent().get(&DataKey::WithdrawalsByCampaign(campaign_id)).unwrap_or(Vec::new(&env));
        withdrawals.push_back(withdrawal.clone());
        env.storage().persistent().set(&DataKey::WithdrawalsByCampaign(campaign_id), &withdrawals);
        env.events().publish((Symbol::new(&env, "withdrawal_requested"),), WithdrawalRequestedEvent {
            campaign_id,
            recipient,
            amount,
        });
        id
    }

    pub fn approve_withdrawal(env: Env, withdrawal_id: u64, admin: Address) {
        admin.require_auth();
        Self::ensure_admin(&env, &admin);
        let mut withdrawal = env.storage().persistent().get::<DataKey, Withdrawal>(&DataKey::Withdrawal(withdrawal_id)).unwrap();
        withdrawal.approved = true;
        env.storage().persistent().set(&DataKey::Withdrawal(withdrawal_id), &withdrawal);
        env.events().publish((Symbol::new(&env, "withdrawal_approved"),), WithdrawalApprovedEvent { withdrawal_id });
    }

    pub fn reject_withdrawal(env: Env, withdrawal_id: u64, admin: Address, reason: String) {
        admin.require_auth();
        Self::ensure_admin(&env, &admin);
        let withdrawal = env.storage().persistent().get::<DataKey, Withdrawal>(&DataKey::Withdrawal(withdrawal_id)).unwrap();
        let _ = withdrawal;
        env.events().publish((Symbol::new(&env, "withdrawal_rejected"),), WithdrawalRejectedEvent { withdrawal_id, reason });
    }

    pub fn get_withdrawal(env: Env, withdrawal_id: u64) -> Option<Withdrawal> {
        env.storage().persistent().get(&DataKey::Withdrawal(withdrawal_id))
    }

    pub fn get_withdrawals_by_campaign(env: Env, campaign_id: u64) -> Vec<Withdrawal> {
        env.storage().persistent().get(&DataKey::WithdrawalsByCampaign(campaign_id)).unwrap_or(Vec::new(&env))
    }

    fn ensure_admin(env: &Env, admin: &Address) {
        let stored_admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        if stored_admin != *admin {
            panic!("unauthorized");
        }
    }

    fn next_withdrawal_id(env: &Env) -> u64 {
        let mut next_id: u64 = env.storage().instance().get(&Symbol::new(env, "next_withdrawal_id")).unwrap_or(1);
        env.storage().instance().set(&Symbol::new(env, "next_withdrawal_id"), &(next_id + 1));
        next_id
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn withdrawal_requests_and_approval_flow() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, WithdrawalContract);
        let client = WithdrawalContractClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        let owner = Address::generate(&env);
        let recipient = Address::generate(&env);

        client.initialize(&admin);
        let withdrawal_id = client.request_withdrawal(&7_u64, &owner, &120_i128, &recipient);

        let withdrawal = client.get_withdrawal(&withdrawal_id).unwrap();
        assert_eq!(withdrawal.amount, 120_i128);
        assert!(!withdrawal.approved);

        client.approve_withdrawal(&withdrawal_id, &admin);
        let approved = client.get_withdrawal(&withdrawal_id).unwrap();
        assert!(approved.approved);
    }
}
