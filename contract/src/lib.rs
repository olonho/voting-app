use borsh::{BorshDeserialize, BorshSerialize};
use near_bindgen::{env, near_bindgen};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize, Deserialize)]
pub struct VotingOptions {
    user: String,
    question: String,
    variants: HashMap<String, String>,
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Voting {
    records: HashMap<String, String>,
}

#[near_bindgen]
impl Voting {
    pub fn vote(&mut self, message: String) {
        let account_id = env::signer_account_id();
        self.records.insert(account_id, message);
    }

    pub fn show_options(&self, account_id: String) -> VotingOptions {
        match self.records.get(&account_id) {
            None => {
                env::log(b"Using default message.");
                return VotingOptions {
                    user: account_id,
                    question: "Default question".to_string(),
                    variants: [
                        ("question1".to_string(), "Question 1".to_string()),
                        ("question2".to_string(), "Question 2".to_string()),
                    ]
                    .iter()
                    .cloned()
                    .collect(),
                };
            }
            _ => {
                env::log(b"Using custom question.");
                return VotingOptions {
                    user: account_id,
                    question: "Custom question".to_string(),
                    variants: [
                        ("question3".to_string(), "Question 3".to_string()),
                        ("question4".to_string(), "Question 4".to_string()),
                    ]
                    .iter()
                    .cloned()
                    .collect(),
                };
            }
        }
    }
}

/*
#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_bindgen::MockedBlockchain;
    use near_bindgen::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
        }
    }

    #[test]
    fn set_get_message() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Voting::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(
            "howdy bob_near".to_string(),
            contract.welcome("bob_near".to_string()).text
        );
    }

    #[test]
    fn get_nonexistent_message() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = Welcome::default();
        assert_eq!(
            "Hello francis.near".to_string(),
            contract.welcome("francis.near".to_string()).text
        );
    }
}
*/
