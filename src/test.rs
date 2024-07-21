#![cfg(test)]
use crate::contract::{NFToken, NFTokenClient};

use crate::test_util::setup_test_token;
use soroban_sdk::{
    testutils::Address as _, Address,
    Env,
};

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register_contract(None, NFToken);
    let client = NFTokenClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);
    assert_eq!(admin, client.admin());
    // TODO: getters for other fields?
}

#[test]
fn test_mint_new() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let client = setup_test_token(&env, &admin);

    let to = Address::generate(&env);
    client.mint_new(&to);
    assert_eq!(to, client.owner(&0));
}
