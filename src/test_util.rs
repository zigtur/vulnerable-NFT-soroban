#![cfg(any(test, feature = "testutils"))]

use crate::contract::{NFToken, NFTokenClient};
use soroban_sdk::{Address, Env};

pub fn setup_test_token<'a>(
    env: &Env,
    admin: &Address,
) -> NFTokenClient<'a> {
    let contract_id = env.register_contract(None, NFToken);
    let client = NFTokenClient::new(env, &contract_id);



    client.initialize(admin);
    client
}
