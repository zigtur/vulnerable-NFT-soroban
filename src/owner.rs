use crate::balance::read_supply;
use crate::errors::Error;
use crate::storage_types::{DataKey, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD};
use soroban_sdk::{panic_with_error, Address, Env, Vec};

pub fn read_owner(env: &Env, id: i128) -> Address {
    let key = DataKey::Owner(id);
    match env.storage().persistent().get::<DataKey, Address>(&key) {
        Some(balance) => {
            env.storage().persistent().extend_ttl(
                &key,
                BALANCE_LIFETIME_THRESHOLD,
                BALANCE_BUMP_AMOUNT,
            );
            balance
        }
        None => panic_with_error!(env, Error::NotFound),
    }
}

pub fn write_owner(env: &Env, id: i128, owner: Option<Address>) {
    let key = DataKey::Owner(id);
    env.storage().persistent().set(&key, &owner);
    env.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}

pub fn check_owner(env: &Env, auth: &Address, id: i128) {
    if auth != &read_owner(env, id) {
        panic_with_error!(env, Error::NotOwned)
    }
}

pub fn read_all_owned(env: &Env, address: Address) -> Vec<i128> {
    let mut ids = Vec::new(&env);
    let supply = read_supply(&env);
    if supply > 0 {
        for n in 0..supply {
            let owner = read_owner(&env, n);
            if owner == address {
                ids.push_back(n);
            }
        }
    }
    ids
}
