use crate::storage_types::DataKey;
use soroban_sdk::{Env, Address, panic_with_error};
use crate::errors::Error;

pub fn has_administrator(env: &Env) -> bool {
    let key = DataKey::Admin;
    env.storage().instance().has(&key)
}

pub fn read_administrator(env: &Env) -> Address {
    let key = DataKey::Admin;
    match env.storage().instance().get::<DataKey, Address>(&key) {
        Some(data) => data,
        None => panic_with_error!(env, Error::NotFound),
    }
}

pub fn write_administrator(env: &Env, id: &Address) {
    let key = DataKey::Admin;
    env.storage().instance().set(&key, id);
}