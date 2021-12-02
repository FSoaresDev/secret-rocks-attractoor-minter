use cosmwasm_std::{HumanAddr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, JsonSchema)]
pub struct SecretContract {
    pub address: HumanAddr,
    pub contract_hash: String,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, JsonSchema)]
pub struct Config {
    pub admin: HumanAddr,
    pub token_contract: SecretContract,
    pub prng_seed: Vec<u8>,
    pub mint_started: bool,
    pub mint_price: Uint128,
    pub mint_limit: u32,
}
