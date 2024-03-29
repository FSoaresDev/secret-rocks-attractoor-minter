TOKEN_ADDRESS=secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg
TOKEN_HASH=9587d60b8e6b078ace12014ceeee089530b9fabcd76535d93666a6c127ad8813
label=$(date +"%T")

deployer_name=test1
deployer_address=$(secretcli keys show -a $deployer_name)
echo "Deployer address: '$deployer_address'"

#NFT REFERENCE
#deployed=$(secretcli tx compute store "./snip721_reference_impl.wasm" --from test1 --gas 250000000 -b block -y)
#code_id=$(secretcli query compute list-code | jq '.[-1]."id"')
#code_hash=$(secretcli query compute list-code | jq '.[-1]."data_hash"')
#echo $code_id $code_hash
nft_code_id=136
nft_code_hash=414974411AA86579DF6E3BBF2B3717E503E60E4AC3C09FC3D76C87726070ADA6

#MINTER
#deployed=$(secretcli tx compute store "../target/wasm32-unknown-unknown/release/secret_rocks_attractoor_minter.wasm" --from test1 --gas 2000000 -b block -y)
#code_id=$(secretcli query compute list-code | jq '.[-1]."id"')
#code_hash=$(secretcli query compute list-code | jq '.[-1]."data_hash"')
#echo $code_id $code_hash
minter_code_id=351
minter_code_hash=177B34037F1BAD937FF38D8416A005CC510A881AC471EA53099DB5A228238253

owner_address=secret1r4gka3q0zcner6vg629e887a6wejpy00djwlk6
secretcli tx compute instantiate $minter_code_id " \
  { \
    \"prng_seed\": \"ZW5pZ21hLXJvY2tzCg==\", \
	  \"token_contract\": { \"address\": \"$TOKEN_ADDRESS\", \"contract_hash\": \"$TOKEN_HASH\"}, \
    \"mint_limit\": 10101, \
    \"mint_amount_cap_per_tx\": 50, \
    \"mint_price\": \"9000000\", \
    \"giveaways\": [\"$owner_address\",\"$owner_address\",\"$owner_address\",\"$owner_address\",\"$owner_address\",\"$owner_address\",\"$owner_address\",\"$owner_address\"], \
    \"utilities\": [{\"traits\": [{\"trait_type\": \"Utility 1\",\"value\": \"Secret Rock Airdrop\"},{\"trait_type\": \"Utility 2\",\"value\": \"Burning Mechanism\"},{\"trait_type\": \"Utility 3\",\"value\": \"Unknown\"}]},{\"traits\": [{\"trait_type\": \"Utility 1\",\"value\": \"Secret Rock Airdrop\"},{\"trait_type\": \"Utility 2\",\"value\": \"Staking Mechanism\"},{\"trait_type\": \"Utility 3\",\"value\": \"Unknown\"}]}] \
  } \
  " --from $deployer_name --gas 270000 --label "Secret Rocks Attractoor Minter $label" -b block -y |
  jq .

mint_address=$(secretcli query compute list-contract-by-code $minter_code_id | jq -r '.[-1].address')

# NFT DEPLOY
secretcli tx compute instantiate $nft_code_id '{"name":"Secret Rocks Attractoors '$label'", "symbol":"SRATT", "admin": "'$mint_address'", "entropy":"ZW5pZ21hLXJvY2tzCg==", "royalty_info":{"decimal_places_in_rates": 2, "royalties": [{"recipient": "'$owner_address'", "rate": 10}]}, "config":{"public_token_supply":true,"public_owner":false,"enable_sealed_metadata":false,"unwrapped_metadata_is_private":false,"minter_may_update_metadata":false,"owner_may_update_metadata": false,"enable_burn": false}}' --label "Secret Rocks Attractoor NFT Collection $label" --from $deployer_name --gas 40000 -y -b block | jq .

nft_address=$(secretcli query compute list-contract-by-code $nft_code_id | jq -r '.[-1].address')

echo "Adding NFT contract to minter"
secretcli tx compute execute $mint_address '{"add_nft_contract":{"contract": {"address": "'$nft_address'", "contract_hash": "'$nft_code_hash'"}}}' --from $deployer_name -y --gas 35000 -b block

echo "Mint giveaways"
secretcli tx compute execute $mint_address '{"mint_giveaways":{}}' --from $deployer_name -y --gas 200000 -b block

echo "Enable Mints"
secretcli tx compute execute $mint_address '{"start_mint":{}}' --from $deployer_name -y --gas 28000 -b block

echo "Mints"
#msg=$(base64 -w 0 <<<'{"mint_nfts": {"count": 50, "entropy": "'"$RANDOM"'"}}')
#secretcli tx compute execute $TOKEN_ADDRESS  '{"send":{"recipient": "'$mint_address'", "amount": "5000", "msg": "'"$msg"'"}}' --from $deployer_name -y --gas 1500000 -b block

echo "=================================="
echo "Mint Add: $mint_address"
echo "NFT Add: $nft_address"
echo "=================================="

#secretcli q compute query secret1zt0u5psm4m5y93wsqm6mad5yql9rmx7s20cctc '{"num_tokens":{}}' | jq .
#secretcli q compute query secret1slh9s2nh3tfx9ugd08u04atrsul9andk7gyx80 '{"info":{}}' | jq .
#secretcli tx compute execute secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg '{"deposit": {}}' --amount 10000000uscrt --from test1 -y --gas 35000 -b block


secretcli tx compute execute secret136kk6q75g9xly7jqufpaue56zc0nptp2ehsa54 '{"start_mint":{}}' --from test1 -y --gas 28000 -b block
secretcli tx compute execute secret136kk6q75g9xly7jqufpaue56zc0nptp2ehsa54 '{"stop_mint":{}}' --from test1 -y --gas 28000 -b block

secretcli tx compute execute secret136kk6q75g9xly7jqufpaue56zc0nptp2ehsa54 '{"change_admin":{"admin": "secret1mdz5rze0xfws4gjl8pw38qnfe77p276qnm82f8"}}' --from test1 -y --gas 28000 -b block