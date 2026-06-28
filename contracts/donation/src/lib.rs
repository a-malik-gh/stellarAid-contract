#![no_std]

use soroban_sdk::{contract, contractclient, contractimpl, contracttype, Address, Env, Symbol, Vec};
use shared::types::Donation;

#[contractclient(name = "CampaignContractClient")]
trait CampaignContractTrait {
    fn update_raised(env: Env, campaign_id: u64, amount: i128);
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    DonationHistory(Address) = 0,
    CampaignDonations(u64) = 1,
    CampaignRaised(u64) = 2,
    CampaignContract = 3,
}

#[contracttype]
#[derive(Clone)]
pub struct DonationMadeEvent {
    pub donor: Address,
    pub campaign_id: u64,
    pub amount: i128,
}

#[contract]
pub struct DonationContract;

#[contractimpl]
impl DonationContract {
    pub fn initialize(env: Env, campaign_contract: Address) {
        env.storage().instance().set(&DataKey::CampaignContract, &campaign_contract);
    }

    pub fn donate(env: Env, donor: Address, campaign_id: u64, amount: i128) {
        donor.require_auth();
        let mut donations = env.storage().persistent().get(&DataKey::CampaignDonations(campaign_id)).unwrap_or(Vec::new(&env));
        let timestamp = env.ledger().timestamp();
        let donation = Donation {
            donor: donor.clone(),
            campaign_id,
            amount,
            timestamp,
        };
        donations.push_back(donation.clone());
        env.storage().persistent().set(&DataKey::CampaignDonations(campaign_id), &donations);

        let donor_history = env.storage().persistent().get(&DataKey::DonationHistory(donor.clone())).unwrap_or(Vec::new(&env));
        let mut history = donor_history;
        history.push_back(donation.clone());
        env.storage().persistent().set(&DataKey::DonationHistory(donor), &history);

        let total = env.storage().persistent().get(&DataKey::CampaignRaised(campaign_id)).unwrap_or(0_i128);
        env.storage().persistent().set(&DataKey::CampaignRaised(campaign_id), &(total + amount));

        let campaign_contract: Address = env.storage().instance().get(&DataKey::CampaignContract).unwrap();
        let campaign_client = CampaignContractClient::new(&env, &campaign_contract);
        campaign_client.update_raised(&campaign_id, &amount);

        env.events().publish((Symbol::new(&env, "donation_made"),), DonationMadeEvent {
            donor: donation.donor,
            campaign_id,
            amount,
        });
    }

    pub fn get_donations_for_campaign(env: Env, campaign_id: u64) -> Vec<Donation> {
        env.storage().persistent().get(&DataKey::CampaignDonations(campaign_id)).unwrap_or(Vec::new(&env))
    }

    pub fn get_total_raised(env: Env, campaign_id: u64) -> i128 {
        env.storage().persistent().get(&DataKey::CampaignRaised(campaign_id)).unwrap_or(0_i128)
    }

    pub fn get_donor_history(env: Env, donor: Address) -> Vec<Donation> {
        env.storage().persistent().get(&DataKey::DonationHistory(donor)).unwrap_or(Vec::new(&env))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn donation_flow_records_history_and_total() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, DonationContract);
        let client = DonationContractClient::new(&env, &contract_id);
        let donor = Address::generate(&env);
        let campaign_contract = Address::generate(&env);

        client.initialize(&campaign_contract);
        client.donate(&donor, &7_u64, &100_i128);

        let donations = client.get_donations_for_campaign(&7_u64);
        assert_eq!(donations.len(), 1);
        assert_eq!(client.get_total_raised(&7_u64), 100_i128);

        let history = client.get_donor_history(&donor);
        assert_eq!(history.len(), 1);
        assert_eq!(history.get(0).unwrap().amount, 100_i128);
    }
}
