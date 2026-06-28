#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Symbol, Address, Env, String};
use shared::types::{Campaign, CampaignStatus};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin = 0,
    Campaign(u64) = 1,
}

#[contracttype]
#[derive(Clone)]
pub struct CampaignCreatedEvent {
    pub campaign_id: u64,
    pub owner: Address,
    pub goal: i128,
    pub deadline: u64,
}

#[contract]
pub struct CampaignContract;

#[contractimpl]
impl CampaignContract {
    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    pub fn create_campaign(env: Env, owner: Address, goal: i128, deadline: u64) -> u64 {
        owner.require_auth();
        let id = Self::next_campaign_id(&env);
        let campaign = Campaign {
            id,
            owner: owner.clone(),
            goal,
            raised: 0,
            status: CampaignStatus::Active,
            deadline,
        };
        env.storage().persistent().set(&DataKey::Campaign(id), &campaign);
        env.events().publish((Symbol::new(&env, "campaign_created"),), CampaignCreatedEvent {
            campaign_id: id,
            owner,
            goal,
            deadline,
        });
        id
    }

    pub fn get_campaign(env: Env, campaign_id: u64) -> Option<Campaign> {
        env.storage().persistent().get(&DataKey::Campaign(campaign_id))
    }

    pub fn update_raised(env: Env, campaign_id: u64, amount: i128) {
        let mut campaign = env
            .storage()
            .persistent()
            .get::<DataKey, Campaign>(&DataKey::Campaign(campaign_id))
            .unwrap();
        campaign.raised += amount;
        env.storage().persistent().set(&DataKey::Campaign(campaign_id), &campaign);
    }

    pub fn approve_campaign(env: Env, admin: Address, campaign_id: u64) {
        admin.require_auth();
        Self::ensure_admin(&env, &admin);
        let mut campaign = Self::get_campaign(env.clone(), campaign_id).unwrap();
        campaign.status = CampaignStatus::Active;
        env.storage().persistent().set(&DataKey::Campaign(campaign_id), &campaign);
    }

    pub fn reject_campaign(env: Env, admin: Address, campaign_id: u64, reason: String) {
        admin.require_auth();
        Self::ensure_admin(&env, &admin);
        let mut campaign = Self::get_campaign(env.clone(), campaign_id).unwrap();
        campaign.status = CampaignStatus::Rejected;
        env.storage().persistent().set(&DataKey::Campaign(campaign_id), &campaign);
        let _ = reason;
    }

    pub fn suspend_campaign(env: Env, admin: Address, campaign_id: u64) {
        admin.require_auth();
        Self::ensure_admin(&env, &admin);
        let mut campaign = Self::get_campaign(env.clone(), campaign_id).unwrap();
        campaign.status = CampaignStatus::Suspended;
        env.storage().persistent().set(&DataKey::Campaign(campaign_id), &campaign);
    }

    pub fn transfer_admin(env: Env, current_admin: Address, new_admin: Address) {
        current_admin.require_auth();
        Self::ensure_admin(&env, &current_admin);
        env.storage().instance().set(&DataKey::Admin, &new_admin);
    }

    fn ensure_admin(env: &Env, admin: &Address) {
        let stored_admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        if stored_admin != *admin {
            panic!("unauthorized");
        }
    }

    fn next_campaign_id(env: &Env) -> u64 {
        let mut next_id: u64 = env.storage().instance().get(&Symbol::new(env, "next_campaign_id")).unwrap_or(1);
        env.storage().instance().set(&Symbol::new(env, "next_campaign_id"), &(next_id + 1));
        next_id
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn campaign_admin_and_status_flow() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, CampaignContract);
        let client = CampaignContractClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        let owner = Address::generate(&env);

        client.initialize(&admin);
        let campaign_id = client.create_campaign(&owner, &1_000_i128, &2_000_u64);
        let campaign = client.get_campaign(&campaign_id).unwrap();

        assert_eq!(campaign.owner, owner);
        assert_eq!(campaign.goal, 1_000_i128);
        assert_eq!(campaign.status, CampaignStatus::Active);

        client.suspend_campaign(&admin, &campaign_id);
        let suspended = client.get_campaign(&campaign_id).unwrap();
        assert_eq!(suspended.status, CampaignStatus::Suspended);

        client.reject_campaign(&admin, &campaign_id, &String::from_str(&env, "spam"));
        let rejected = client.get_campaign(&campaign_id).unwrap();
        assert_eq!(rejected.status, CampaignStatus::Rejected);
    }
}
