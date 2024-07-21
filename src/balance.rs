use crate::storage_types::{DataKey, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD};
use soroban_sdk::Env;

pub fn read_supply(env: &Env) -> i128 {
    let key = DataKey::Supply;
    match env.storage().persistent().get::<DataKey, i128>(&key) {
        Some(balance) => {
            env.storage().persistent().extend_ttl(
                &key,
                BALANCE_LIFETIME_THRESHOLD,
                BALANCE_BUMP_AMOUNT,
            );
            balance
        }
        None => 0,
    }
}

pub fn increment_supply(env: &Env) {
    let key = DataKey::Supply;
    env.storage()
        .persistent()
        .set(&key, &(read_supply(&env) + 1));
    env.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}
