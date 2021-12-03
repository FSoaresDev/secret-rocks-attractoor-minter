use cosmwasm_std::{
    from_binary, to_binary, Api, Binary, Env, Extern, HandleResponse, HumanAddr, InitResponse,
    Querier, StdError, StdResult, Storage, Uint128,
};
use rand::prelude::SliceRandom;
use rand::Rng;
use secret_toolkit::snip20::{send_msg, transfer_msg};

use crate::msg::{
    Authentication, Extension, HandleAnswer, MediaFile, Metadata, Mint, NftsHandleMsg, QueryAnswer,
    ResponseStatus, Trait,
};
use crate::state::{load, save, SecretContract, Utilities};
use crate::{
    msg::{HandleMsg, InitMsg, QueryMsg},
    state::Config,
};
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::{ChaCha20Rng, ChaChaRng};
use secret_toolkit::storage::{TypedStore, TypedStoreMut};
use secret_toolkit::utils::{HandleCallback, InitCallback};
use secret_toolkit::{crypto::sha_256, snip20::register_receive_msg};
use sha2::{Digest, Sha256};

pub const BLOCK_SIZE: usize = 256;

pub const SECRET_NUMBERS: &[u8] = b"secret_numbers";
pub const ADDITIONAL_ENTROPY: &[u8] = b"additional_entropy";
pub const CONFIG_KEY: &[u8] = b"config";

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let mut config_store = TypedStoreMut::attach(&mut deps.storage);
    let prng_seed_hashed = sha_256(&msg.prng_seed.0);

    let mut admin = env.message.sender;

    if msg.admin.is_some() {
        admin = msg.admin.unwrap();
    }

    config_store.store(
        CONFIG_KEY,
        &Config {
            admin,
            token_contract: msg.token_contract.clone(),
            nft_contract: None,
            prng_seed: prng_seed_hashed.to_vec(),
            mint_started: false,
            mint_price: msg.mint_price.clone(),
            mint_limit: msg.mint_limit.clone(),
            giveaways_to_send: msg.giveaways.clone(),
            utilities: msg.utilities.clone(),
            mint_amount_cap_per_tx: msg.mint_amount_cap_per_tx.clone(),
            minted_current_utilities: vec![0; msg.utilities.len()],
        },
    )?;

    let additional_entropy = (0..1000).map(|_| "0").collect::<Vec<_>>().concat();
    save(
        &mut deps.storage,
        ADDITIONAL_ENTROPY,
        &additional_entropy.to_string(),
    )?;

    let mut secret_numbers: Vec<u32> = (1..msg.mint_limit + 1).map(|i: u32| i).collect();

    let mut hasher = Sha256::new();
    vec![
        additional_entropy.as_bytes(),
        &prng_seed_hashed,
        env.block.time.to_string().as_bytes(),
    ]
    .iter()
    .for_each(|el| hasher.update(el));
    let seed: [u8; 32] = hasher.finalize().into();
    let mut rng = ChaChaRng::from_seed(seed);
    secret_numbers.shuffle(&mut rng);

    save(&mut deps.storage, SECRET_NUMBERS, &secret_numbers)?;

    Ok(InitResponse {
        messages: vec![register_receive_msg(
            env.contract_code_hash.clone(),
            None,
            1,
            msg.token_contract.contract_hash.clone(),
            msg.token_contract.address.clone(),
        )?],
        log: vec![],
    })
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Receive {
            sender,
            from,
            amount,
            msg,
        } => try_receive(deps, env, sender, from, amount, msg),

        HandleMsg::AddNftContract { contract } => add_nft_contract(deps, env, contract),
        HandleMsg::MintGiveaways {} => mint_giveaways(deps, env),
        HandleMsg::StartMint {} => start_mint(deps, env),
        _ => Err(StdError::generic_err("action not found!")),
    }
}

pub fn add_nft_contract<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    contract: SecretContract,
) -> StdResult<HandleResponse> {
    let mut config_store = TypedStoreMut::attach(&mut deps.storage);
    let mut config: Config = config_store.load(CONFIG_KEY)?;

    if env.message.sender != config.admin {
        return Err(StdError::generic_err(format!(
            "Only admin can execute this action!"
        )));
    }

    if config.mint_started != false {
        return Err(StdError::generic_err(format!(
            "Mint should be stoped to perform this"
        )));
    }

    config.nft_contract = Some(contract);

    config_store.store(CONFIG_KEY, &config)?;

    return Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::StartMint {
            status: ResponseStatus::Success,
        })?),
    });
}

pub fn mint_giveaways<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    let config_store = TypedStore::attach(&deps.storage);
    let config: Config = config_store.load(CONFIG_KEY)?;
    let additional_entropy: String = load(&deps.storage, &ADDITIONAL_ENTROPY)?;
    let mut token_data_list: Vec<u32> = load(&deps.storage, &SECRET_NUMBERS)?;

    if env.message.sender != config.admin {
        return Err(StdError::generic_err(format!(
            "Only admin can execute this action!"
        )));
    }

    if config.nft_contract.is_none() {
        return Err(StdError::generic_err(format!(
            "A NFT contract should be added before this action"
        )));
    }

    let nft_contract = config.nft_contract.clone().unwrap();

    // query nft contract to check the current total supply
    let nft_current_count_response = secret_toolkit::snip721::num_tokens_query(
        &deps.querier,
        None,
        BLOCK_SIZE,
        nft_contract.contract_hash.clone(),
        nft_contract.address.clone(),
    )?;

    if nft_current_count_response.count == config.mint_limit {
        return Err(StdError::generic_err(format!("No more mints available!")));
    }

    let mints_available = config.mint_limit - nft_current_count_response.count;

    if mints_available < config.giveaways_to_send.len() as u32 {
        return Err(StdError::generic_err(format!(
            "Giveaway mints surpass the mint limit!"
        )));
    }

    let mut hasher = Sha256::new();
    vec![
        additional_entropy.as_bytes(),
        &config.prng_seed,
        env.block.time.to_string().as_bytes(),
    ]
    .iter()
    .for_each(|el| hasher.update(el));
    let seed: [u8; 32] = hasher.finalize().into();
    let mut rng = ChaChaRng::from_seed(seed);

    let mut mints: Vec<Mint> = vec![];
    let mut minted_current_utilities = config.minted_current_utilities.clone();

    for giveaway in config.giveaways_to_send {
        mints.push(_mint(
            deps,
            &giveaway,
            &mut rng,
            &mut token_data_list,
            &mut minted_current_utilities,
            &config.utilities,
        ));
    }

    let mut config_store = TypedStoreMut::attach(&mut deps.storage);
    let mut config: Config = config_store.load(CONFIG_KEY)?;

    config.giveaways_to_send = vec![];
    config.minted_current_utilities = minted_current_utilities;
    config_store.store(CONFIG_KEY, &config)?;

    let mints_msg = NftsHandleMsg::BatchMintNft {
        mints,
        padding: None,
    };

    let mints_cosmos_msg =
        mints_msg.to_cosmos_msg(nft_contract.contract_hash, nft_contract.address, None)?;

    return Ok(HandleResponse {
        messages: vec![mints_cosmos_msg],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::StartMint {
            status: ResponseStatus::Success,
        })?),
    });
}

pub fn start_mint<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    let mut config_store = TypedStoreMut::attach(&mut deps.storage);
    let mut config: Config = config_store.load(CONFIG_KEY)?;

    if env.message.sender != config.admin {
        return Err(StdError::generic_err(format!(
            "Only admin can execute this action!"
        )));
    }

    if config.nft_contract.is_none() {
        return Err(StdError::generic_err(format!(
            "A NFT contract should be added before the mint is started"
        )));
    }

    config.mint_started = true;

    config_store.store(CONFIG_KEY, &config)?;

    return Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::StartMint {
            status: ResponseStatus::Success,
        })?),
    });
}

pub fn stop_mint<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    let mut config_store = TypedStoreMut::attach(&mut deps.storage);
    let mut config: Config = config_store.load(CONFIG_KEY)?;

    if env.message.sender != config.admin {
        return Err(StdError::generic_err(format!(
            "Only admin can execute this action!"
        )));
    }

    config.mint_started = false;

    config_store.store(CONFIG_KEY, &config)?;

    return Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::StartMint {
            status: ResponseStatus::Success,
        })?),
    });
}

pub fn try_receive<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    _sender: HumanAddr,
    from: HumanAddr,
    amount: Uint128,
    msg: Binary,
) -> StdResult<HandleResponse> {
    let config = TypedStore::<Config, S>::attach(&deps.storage).load(CONFIG_KEY)?;
    let msg: HandleMsg = from_binary(&msg)?;
    if let HandleMsg::MintNfts { count, entropy } = msg.clone() {
        if env.message.sender != config.token_contract.address {
            return Err(StdError::generic_err(format!("Invalid token sent!")));
        } else {
            return mint_nfts(deps, env.clone(), amount, from, count, entropy);
        }
    } else {
        return Err(StdError::generic_err(format!("Receive handler not found!")));
    }
}

pub fn mint_nfts<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    amount: Uint128,
    from: HumanAddr,
    count: u32,
    entropy: String,
) -> StdResult<HandleResponse> {
    let config_store = TypedStore::attach(&deps.storage);
    let config: Config = config_store.load(CONFIG_KEY)?;
    let mut token_data_list: Vec<u32> = load(&deps.storage, &SECRET_NUMBERS)?;

    if count > config.mint_amount_cap_per_tx {
        return Err(StdError::generic_err(format!(
            "Only {:?} tokens can be minted at the same time!",
            config.mint_amount_cap_per_tx
        )));
    }

    if config.mint_started != true {
        return Err(StdError::generic_err(format!("Mint has not started yet!")));
    }

    if config.nft_contract.is_none() {
        return Err(StdError::generic_err(format!("Nft needs to be added!")));
    }

    let nft_contract = config.nft_contract.unwrap().clone();

    // Check if sent amount is correct
    let total_amount_expected = config.mint_price.u128() * (count as u128);

    if total_amount_expected != amount.u128() {
        return Err(StdError::generic_err(format!(
            "Incorrect amount of snip20 tokens received {:?} != {:?}",
            amount.u128(),
            total_amount_expected
        )));
    }

    // query nft contract to check the current total supply
    let nft_current_count_response = secret_toolkit::snip721::num_tokens_query(
        &deps.querier,
        None,
        BLOCK_SIZE,
        nft_contract.contract_hash.clone(),
        nft_contract.address.clone(),
    )?;

    if nft_current_count_response.count == config.mint_limit {
        return Err(StdError::generic_err(format!("No more mints available!")));
    }

    let mints_available = config.mint_limit - nft_current_count_response.count;

    if mints_available < count {
        return Err(StdError::generic_err(format!(
            "Not sufficient available mints to satisfy this request!"
        )));
    }

    // add entropy provided by the user to the additional entropy
    let mut new_entropy = entropy.clone();
    if new_entropy.len() > 20 {
        new_entropy = new_entropy[0..20].to_string();
    } else if new_entropy.len() == 0 {
        new_entropy = "0".to_string();
    }

    let mut additional_entropy: String = load(&deps.storage, &ADDITIONAL_ENTROPY)?;

    // Randomize the length that will be added with current and new entropy
    // this will prevent predicting results in a feasible way even with someone knowing the seed
    let mut hasher = Sha256::new();
    vec![
        additional_entropy.as_bytes(),
        new_entropy.as_bytes(),
        &config.prng_seed,
        env.block.time.to_string().as_bytes(),
    ]
    .iter()
    .for_each(|el| hasher.update(el));
    let seed: [u8; 32] = hasher.finalize().into();
    let mut rng = ChaChaRng::from_seed(seed);

    let new_entropy_len: usize = rng.gen_range(0, new_entropy.len());

    if additional_entropy.len() >= 1000 {
        additional_entropy = additional_entropy[new_entropy_len..additional_entropy.len()]
            .to_string()
            + &new_entropy[0..new_entropy_len];
    } else {
        additional_entropy = additional_entropy + &new_entropy[0..new_entropy_len];
    }

    save(&mut deps.storage, ADDITIONAL_ENTROPY, &additional_entropy)?;

    let mut mints: Vec<Mint> = vec![];
    let mut minted_current_utilities = config.minted_current_utilities.clone();

    for _ in 1..=count {
        mints.push(_mint(
            deps,
            &from,
            &mut rng,
            &mut token_data_list,
            &mut minted_current_utilities,
            &config.utilities,
        ));
    }

    // update minted_current_utilities
    let mut config_store = TypedStoreMut::attach(&mut deps.storage);
    let mut config: Config = config_store.load(CONFIG_KEY)?;

    config.minted_current_utilities = minted_current_utilities;
    config_store.store(CONFIG_KEY, &config)?;

    let mints_msg = NftsHandleMsg::BatchMintNft {
        mints,
        padding: None,
    };

    let mints_cosmos_msg =
        mints_msg.to_cosmos_msg(nft_contract.contract_hash, nft_contract.address, None)?;

    let mut messages = vec![mints_cosmos_msg];

    // mint revenue
    let wallet1_revenue = amount.multiply_ratio(Uint128(15 as u128), Uint128(1000));

    messages.push(transfer_msg(
        HumanAddr("secret1q7kkhvkwzlj0dv3p3nk2ewfgvxs22u6hav3a65".to_string()),
        wallet1_revenue,
        None,
        BLOCK_SIZE,
        config.token_contract.contract_hash.clone(),
        config.token_contract.address.clone(),
    )?);

    let wallet2_revenue = amount.multiply_ratio(Uint128(985 as u128), Uint128(1000));

    messages.push(transfer_msg(
        HumanAddr("secret1r4gka3q0zcner6vg629e887a6wejpy00djwlk6".to_string()),
        wallet2_revenue,
        None,
        BLOCK_SIZE,
        config.token_contract.contract_hash.clone(),
        config.token_contract.address.clone(),
    )?);

    return Ok(HandleResponse {
        messages,
        log: vec![],
        data: Some(to_binary(&HandleAnswer::MintNfts {
            status: ResponseStatus::Success,
        })?),
    });
}

fn _mint<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    owner: &HumanAddr,
    rng: &mut ChaCha20Rng,
    secret_numbers_list: &mut Vec<u32>,
    minted_current_utilities: &mut Vec<u32>,
    utilities: &Vec<Utilities>,
) -> Mint {
    let utilities_index: usize = rng.gen_range(0, utilities.len());
    let utility = utilities[utilities_index].clone();
    minted_current_utilities[utilities_index] = minted_current_utilities[utilities_index] + 1;

    let secret_number_index: usize = rng.gen_range(0, secret_numbers_list.len());
    let secret_number = secret_numbers_list.swap_remove(secret_number_index);

    return Mint {
        token_id: None,
        owner: Some(owner.clone()),
        public_metadata: Some(Metadata {
            extension: Some(Extension {
                image: None,
                image_data: None,
                external_url: None,
                description: None,
                name: Some("Secret Rocks Attractoor".to_string()),
                attributes: Some(utility.traits),
                background_color: None,
                animation_url: None,
                youtube_url: None,
                media: Some(vec![MediaFile {
                    file_type: Some("image".to_string()),
                    extension: Some("gif".to_string()),
                    authentication: Some(Authentication {
                        key: Some("".to_string()),
                        user: Some("".to_string()),
                    }),
                    url: "https://ipfs.io/ipfs/QmcbALnjvhWekHuzrTZA3r5MmU8bthHiJ5xne2n8Uw96Ck"
                        .to_string(),
                }]),
                protected_attributes: None,
            }),
            token_uri: None,
        }),
        private_metadata: Some(Metadata {
            extension: Some(Extension {
                image: None,
                image_data: None,
                external_url: None,
                description: None,
                name: Some("Secret Rocks Attractoor".to_string()),
                attributes: Some(vec![Trait {
                    display_type: None,
                    trait_type: Some("Attracted Secret Rock ID".to_string()),
                    value: secret_number.to_string(),
                    max_value: None,
                }]),
                background_color: None,
                animation_url: None,
                youtube_url: None,
                media: Some(vec![MediaFile {
                    file_type: Some("image".to_string()),
                    extension: Some("gif".to_string()),
                    authentication: Some(Authentication {
                        key: Some("".to_string()),
                        user: Some("".to_string()),
                    }),
                    url: "https://ipfs.io/ipfs/QmcbALnjvhWekHuzrTZA3r5MmU8bthHiJ5xne2n8Uw96Ck"
                        .to_string(),
                }]),
                protected_attributes: None,
            }),
            token_uri: None,
        }),
        memo: None,
        serial_number: None,
        royalty_info: None,
    };
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Info {} => query_info(deps),
    }
}

fn query_info<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<Binary> {
    let config_store = TypedStore::attach(&deps.storage);
    let config: Config = config_store.load(CONFIG_KEY)?;
    let nft_contract = config.nft_contract.unwrap().clone();

    let nft_current_count_response = secret_toolkit::snip721::num_tokens_query(
        &deps.querier,
        None,
        BLOCK_SIZE,
        nft_contract.contract_hash.clone(),
        nft_contract.address.clone(),
    )?;

    to_binary(&QueryAnswer::Info {
        nft_contract,
        mint_amount_cap_per_tx: config.mint_amount_cap_per_tx,
        max_total_supply: config.mint_limit,
        mint_current_count: nft_current_count_response.count,
        mint_current_left: config.mint_limit - nft_current_count_response.count,
        utilities: config.utilities,
        minted_current_utilities: config.minted_current_utilities,
    })
}
