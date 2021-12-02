use cosmwasm_std::{HumanAddr, ReadonlyStorage, StdError, StdResult, Storage, Uint128};
use schemars::JsonSchema;
use secret_toolkit::serialization::{Bincode2, Serde};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::any::type_name;

use crate::msg::Trait;

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, JsonSchema)]
pub struct SecretContract {
    pub address: HumanAddr,
    pub contract_hash: String,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, JsonSchema)]
pub struct Config {
    pub admin: HumanAddr,
    pub token_contract: SecretContract,
    pub nft_contract: Option<SecretContract>,
    pub prng_seed: Vec<u8>,
    pub mint_started: bool,
    pub mint_price: Uint128,
    pub mint_limit: u32,
    pub giveaways_to_send: Vec<HumanAddr>,
    pub utilities: Vec<Utilities>,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, JsonSchema)]
pub struct Utilities {
    pub traits: Vec<Trait>,
}

/// Returns StdResult<()> resulting from saving an item to storage
///
/// # Arguments
///
/// * `storage` - a mutable reference to the storage this item should go to
/// * `key` - a byte slice representing the key to access the stored item
/// * `value` - a reference to the item to store
pub fn save<T: Serialize, S: Storage>(storage: &mut S, key: &[u8], value: &T) -> StdResult<()> {
    storage.set(key, &Bincode2::serialize(value)?);
    Ok(())
}

/// Returns StdResult<T> from retrieving the item with the specified key.  Returns a
/// StdError::NotFound if there is no item with that key
///
/// # Arguments
///
/// * `storage` - a reference to the storage this item is in
/// * `key` - a byte slice representing the key that accesses the stored item
pub fn load<T: DeserializeOwned, S: ReadonlyStorage>(storage: &S, key: &[u8]) -> StdResult<T> {
    Bincode2::deserialize(
        &storage
            .get(key)
            .ok_or_else(|| StdError::not_found(type_name::<T>()))?,
    )
}

/// Removes an item from storage
///
/// # Arguments
///
/// * `storage` - a mutable reference to the storage this item is in
/// * `key` - a byte slice representing the key that accesses the stored item
pub fn remove<S: Storage>(storage: &mut S, key: &[u8]) {
    storage.remove(key);
}
