#!/bin/bash

function secretd() {
  export docker_name=secretdev
  docker exec --interactive "$docker_name" secretd "$@";
}

function wait_for_tx() {
  until (secretd q tx "$1"); do
      sleep 5
  done
}

export SGX_MODE=SW
export deployer_name=a
export wasm_path=/root/code

export deployer_address=$(secretd keys show -a $deployer_name)
echo "Deployer address: '$deployer_address'"

label=$(date +"%T")

secretd tx compute store "${wasm_path}/devChain/snip20_reference_impl.wasm" --from "$deployer_name" --gas 4000000 -b block -y
export devSNIP20_code_id=$(secretd query compute list-code | jq -r '.[-1]."id"')
export devSNIP20_code_hash=$(secretd query compute list-code | jq -r '.[-1]."data_hash"')
echo "Stored SNIP20: '$devSNIP20_code_id', '$devSNIP20_code_hash'"

secretd tx compute store "${wasm_path}/devChain/snip721_reference_impl.wasm" --from "$deployer_name" --gas 400000000 -b block -y
export nfts_code_id=$(secretd query compute list-code | jq -r '.[-1]."id"')
export nfts_code_hash=$(secretd query compute list-code | jq -r '.[-1]."data_hash"')
echo "Stored nft: '$nfts_code_id', '$nfts_code_hash'"

secretd tx compute store "${wasm_path}/target/wasm32-unknown-unknown/release/secret_rocks_attractoor_minter.wasm" --from "$deployer_name" --gas 4000000 -b block -y
export minter_code_id=$(secretd query compute list-code | jq -r '.[-1]."id"')
export minter_code_hash=$(secretd query compute list-code | jq -r '.[-1]."data_hash"')
echo "Stored minter: '$minter_code_id', '$minter_code_hash'"

echo "Deploying devSNIP20..."
export TX_HASH=$(
  secretd tx compute instantiate $devSNIP20_code_id '{"name":"devSNIP20","symbol":"DSNIP","decimals":6,"initial_balances":[{"address":"'$deployer_address'","amount":"1000000000000"}],"prng_seed":"ZW5pZ21hLXJvY2tzCg==","config":{"public_total_supply":false,"enable_deposit":false,"enable_redeem":false,"enable_mint":false,"enable_burn":true}}' --label devSEFI_$label --from $deployer_name --gas 15000000 -y -b block |
  jq -r .txhash
)
wait_for_tx "$TX_HASH" "Waiting for tx to finish on-chain..."
secretd q compute tx $TX_HASH

export devSNIP20_address=$(secretd query compute list-contract-by-code $devSNIP20_code_id | jq -r '.[-1].address')
echo "devSNIP20 address: '$devSNIP20_address'"

echo "Deploying Minter.."
export TX_HASH=$(
  secretd tx compute instantiate $minter_code_id " \
  { \
    \"prng_seed\": \"ZW5pZ21hLXJvY2tzCg==\", \
	  \"token_contract\": { \"address\": \"$devSNIP20_address\", \"contract_hash\": \"$devSNIP20_code_hash\"}, \
    \"mint_limit\": 5, \
    \"mint_price\": \"100\", \
    \"giveaways\": [\"$deployer_address\"], \
    \"utilities\": [{\"traits\": [{\"trait_type\": \"Utility 1\",\"value\": \"Secret Rock Airdrop\"},{\"trait_type\": \"Utility 2\",\"value\": \"Burning Mechanism\"},{\"trait_type\": \"Utility 3\",\"value\": \"Unknown\"}]},{\"traits\": [{\"trait_type\": \"Utility 1\",\"value\": \"Secret Rock Airdrop\"},{\"trait_type\": \"Utility 2\",\"value\": \"Staking Mechanism\"},{\"trait_type\": \"Utility 3\",\"value\": \"Unknown\"}]}] \
  } \
  " --from $deployer_name --gas 15000000 --label srattrs_minter_$label -b block -y |
  jq -r .txhash
)
wait_for_tx "$TX_HASH" "Waiting for tx to finish on-chain..."

export minter_address=$(secretd query compute list-contract-by-code $minter_code_id | jq -r '.[-1].address')
echo "minter address: '$minter_address'"

echo "Deploying NFT Collection..."
export TX_HASH=$(
  secretd tx compute instantiate $nfts_code_id '{"name":"Secret Rocks Attractoors","symbol":"SRATT", "admin": "'$minter_address'", "entropy":"ZW5pZ21hLXJvY2tzCg==", "royalty_info":{"decimal_places_in_rates": 2, "royalties": [{"recipient": "secret1r4gka3q0zcner6vg629e887a6wejpy00djwlk6", "rate": 10}]}, "config":{"public_token_supply":true,"public_owner":false,"enable_sealed_metadata":false,"unwrapped_metadata_is_private":false,"minter_may_update_metadata":false,"owner_may_update_metadata": false,"enable_burn": true}}' --label srattrs_$label --from $deployer_name --gas 15000000 -y -b block |
  jq -r .txhash
)
wait_for_tx "$TX_HASH" "Waiting for tx to finish on-chain..."

export nfts_address=$(secretd query compute list-contract-by-code $nfts_code_id | jq -r '.[-1].address')
echo "nft address: '$nfts_address'"

echo "Adding NFT contract to minter"
secretd tx compute execute $minter_address '{"add_nft_contract":{"contract": {"address": "'$nfts_address'", "contract_hash": "'$nfts_code_hash'"}}}' --from $deployer_name -y --gas 1500000 -b block

echo "Mint giveaways"
secretd tx compute execute $minter_address '{"mint_giveaways":{}}' --from $deployer_name -y --gas 1500000 -b block

echo "Enable Mints"
secretd tx compute execute $minter_address '{"start_mint":{}}' --from $deployer_name -y --gas 1500000 -b block

echo "Mints"
msg=$(base64 -w 0 <<<'{"mint_nfts": {"count": 2, "entropy": "'"$RANDOM"'"}}')
secretd tx compute execute $devSNIP20_address  '{"send":{"recipient": "'$minter_address'", "amount": "200", "msg": "'"$msg"'"}}' --from $deployer_name -y --gas 1500000 -b block

secretd q compute query $nfts_address '{"num_tokens":{}}' | jq .
secretd q compute query $nfts_address '{"nft_info":{"token_id":"0"}}' | jq .

echo "=========================================================================="
echo "=== UserA: $deployer_address"
echo "=== Nfts: $nfts_address"
echo "=== Minter: $minter_address"
echo "=== SNIP20Token: $devSNIP20_address"
echo "=========================================================================="