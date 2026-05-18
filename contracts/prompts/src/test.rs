#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_register_and_get() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(PromptRegistryContract, ());
    let client = PromptRegistryContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let category = String::from_str(&env, "Midjourney");
    let content = String::from_str(&env, "cinematic portrait, golden hour, 8k --ar 16:9");

    client.register_prompt(&owner, &category, &content);

    let prompts = client.get_prompts();
    assert_eq!(prompts.len(), 1);
    assert_eq!(prompts.get(0).unwrap().owner, owner);
}

#[test]
fn test_update_by_owner() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(PromptRegistryContract, ());
    let client = PromptRegistryContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    client.register_prompt(
        &owner,
        &String::from_str(&env, "ChatGPT"),
        &String::from_str(&env, "prompt lama"),
    );

    let id = client.get_prompts().get(0).unwrap().id;

    client.update_prompt(
        &owner,
        &id,
        &String::from_str(&env, "ChatGPT"),
        &String::from_str(&env, "prompt baru yang sudah diedit"),
    );

    let updated = client.get_prompts().get(0).unwrap();
    assert_eq!(updated.content, String::from_str(&env, "prompt baru yang sudah diedit"));
}

#[test]
fn test_delete_by_owner() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(PromptRegistryContract, ());
    let client = PromptRegistryContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    client.register_prompt(
        &owner,
        &String::from_str(&env, "ChatGPT"),
        &String::from_str(&env, "act as a senior engineer"),
    );

    let id = client.get_prompts().get(0).unwrap().id;
    client.delete_prompt(&owner, &id);

    assert_eq!(client.get_prompts().len(), 0);
}

#[test]
#[should_panic]
fn test_update_by_non_owner_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(PromptRegistryContract, ());
    let client = PromptRegistryContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let attacker = Address::generate(&env);

    client.register_prompt(
        &owner,
        &String::from_str(&env, "Midjourney"),
        &String::from_str(&env, "valuable prompt"),
    );

    let id = client.get_prompts().get(0).unwrap().id;

    client.update_prompt(
        &attacker,
        &id,
        &String::from_str(&env, "Midjourney"),
        &String::from_str(&env, "prompt dicuri"),
    );
}

#[test]
#[should_panic]
fn test_delete_by_non_owner_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(PromptRegistryContract, ());
    let client = PromptRegistryContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let attacker = Address::generate(&env);

    client.register_prompt(
        &owner,
        &String::from_str(&env, "ChatGPT"),
        &String::from_str(&env, "some valuable prompt"),
    );

    let id = client.get_prompts().get(0).unwrap().id;
    client.delete_prompt(&attacker, &id);
}