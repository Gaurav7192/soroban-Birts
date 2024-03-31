use soroban_sdk::{Address, Env};

use crate::storage_types::DataKey;

pub fn has_administrator(e: &Env) -> bool { //chech admin
    let key = DataKey::Admin;
    e.storage().instance().has(&key)
}

pub fn read_administrator(e: &Env) -> Address { //read administrator
    let key = DataKey::Admin;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_administrator(e: &Env, id: &Address) {  //check token owner
    let key = DataKey::Admin;
    e.storage().instance().set(&key, id);
}
