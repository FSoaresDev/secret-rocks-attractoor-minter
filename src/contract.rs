use cosmwasm_std::{
    from_binary, to_binary, Api, Binary, Env, Extern, HandleResponse, HumanAddr, InitResponse,
    Querier, StdError, StdResult, Storage, Uint128,
};
use secret_toolkit::storage::{TypedStore, TypedStoreMut};
use secret_toolkit::utils::InitCallback;
use secret_toolkit::{crypto::sha_256, snip20::register_receive_msg};

use crate::msg::{HandleAnswer, QueryAnswer, ResponseStatus};
use crate::{
    msg::{HandleMsg, InitMsg, QueryMsg},
    state::Config,
};

pub const BLOCK_SIZE: usize = 256;

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
            prng_seed: prng_seed_hashed.to_vec(),
            mint_started: false,
            mint_price: msg.mint_price.clone(),
            mint_limit: msg.mint_limit.clone(),
        },
    )?;

    let additional_entropy = (0..1000).map(|_| "0").collect::<Vec<_>>().concat();

    let mut additional_entropy_store = TypedStoreMut::attach(&mut deps.storage);
    additional_entropy_store.store(ADDITIONAL_ENTROPY, &additional_entropy.to_string())?;

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

        HandleMsg::StartMint {} => start_mint(deps, env),
        _ => Err(StdError::generic_err("action not found!")),
    }
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
    let mut config_store = TypedStoreMut::attach(&mut deps.storage);
    let mut config: Config = config_store.load(CONFIG_KEY)?;

    if config.mint_started != true {
        return Err(StdError::generic_err(format!("Mint has not started yet!")));
    }

    return Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::MintNfts {
            status: ResponseStatus::Success,
        })?),
    });
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
    to_binary(&QueryAnswer::Info {
        mint_limit: 0,
        mint_count: 0,
    })
}
