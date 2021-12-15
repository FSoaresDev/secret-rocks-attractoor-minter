TOKEN_ADDRESS=secret1k0jntykt7e4g3y88ltc60czgjuqdy4c9e8fzek
TOKEN_HASH=af74387e276be8874f07bec3a87023ee49b0e7ebe08178c49d0a49c3c98ed60e
label=$(date +"%T")

deployer_name=attractor_minter_admin
deployer_address=$(secretcli keys show -a $deployer_name)
echo "Deployer address: '$deployer_address'"

#NFT REFERENCE
#deployed=$(secretcli tx compute store "./snip721_reference_impl.wasm" --from $deployer_name --gas 4200000 -b block -y)
#code_id=$(secretcli query compute list-code | jq '.[-1]."id"')
#code_hash=$(secretcli query compute list-code | jq '.[-1]."data_hash"')
#echo $code_id $code_hash
nft_code_id=210
nft_code_hash=414974411AA86579DF6E3BBF2B3717E503E60E4AC3C09FC3D76C87726070ADA6

#MINTER
#secretcli tx compute store "../target/wasm32-unknown-unknown/release/secret_rocks_attractoor_minter.wasm" --from $deployer_name --gas 2000000 -b block -y
#secretcli query compute list-code | jq '.[-1]."id"'
#secretcli query compute list-code | jq '.[-1]."data_hash"'
#echo $code_id $code_hash
minter_code_id=211
minter_code_hash=177B34037F1BAD937FF38D8416A005CC510A881AC471EA53099DB5A228238253

owner_address=secret1r4gka3q0zcner6vg629e887a6wejpy00djwlk6
#secretcli tx compute instantiate 211 " \
#  { \
#    \"prng_seed\": \"\", \
#	\"token_contract\": { \"address\": \"secret1k0jntykt7e4g3y88ltc60czgjuqdy4c9e8fzek\", \"contract_hash\": \"af74387e276be8874f07bec3a87023ee49b0e7ebe08178c49d0a49c3c98ed60e\"}, \
#    \"mint_limit\": 10101, \
#    \"mint_amount_cap_per_tx\": 50, \
#    \"mint_price\": \"9000000\", \
#    \"giveaways\": [\"$owner_address\",\"$owner_address\",\"$owner_address\",\"$owner_address\",\"$owner_address\",\"$owner_address\",\"$owner_address\",\"$owner_address\"], \
#    \"utilities\": [{\"traits\": [{\"trait_type\": \"Utility 1\",\"value\": \"Secret Rock Airdrop\"},{\"trait_type\": \"Utility 2\",\"value\": \"Burning Mechanism\"},{\"trait_type\": \"Utility 3\",\"value\": \"Unknown\"}]},{\"traits\": [{\"trait_type\": \"Utility 1\",\"value\": \"Secret Rock Airdrop\"},{\"trait_type\": \"Utility 2\",\"value\": \"Staking Mechanism\"},{\"trait_type\": \"Utility 3\",\"value\": \"Unknown\"}]}] \
#  } \
#  " --from $deployer_name --gas 270000 --label "Secret Rocks Attractoor Minter" -b block -y |
#  jq .

#mint_address=$(secretcli query compute list-contract-by-code 211 | jq -r '.[-1].address')
secret1844ks0ny2qgyru065mykut38cyph45wt6cxfj3

# NFT DEPLOY
#secretcli tx compute instantiate 210 '{"name":"Secret Rocks Attractoors", "symbol":"SRATT", "admin": "'$mint_address'", "entropy":"", "royalty_info":{"decimal_places_in_rates": 2, "royalties": [{"recipient": "'$owner_address'", "rate": 10}]}, "config":{"public_token_supply":true,"public_owner":false,"enable_sealed_metadata":false,"unwrapped_metadata_is_private":false,"minter_may_update_metadata":false,"owner_may_update_metadata": false,"enable_burn": false}}' --label "Secret Rocks Attractoor NFT Collection" --from $deployer_name --gas 40000 -y -b block | jq .

#nft_address=$(secretcli query compute list-contract-by-code 210 | jq -r '.[-1].address')
secret1kkj5ls0lfkk66e689ll68sntt0j3asvyw4nwnt

echo "Adding NFT contract to minter"
#secretcli tx compute execute $mint_address '{"add_nft_contract":{"contract": {"address": "'$nft_address'", "contract_hash": "'$nft_code_hash'"}}}' --from $deployer_name -y --gas 35000 -b block

echo "Mint giveaways"
#secretcli tx compute execute $mint_address '{"mint_giveaways":{}}' --from $deployer_name -y --gas 200000 -b block

# CHANGE ADMIN TO OTHER WALLET...
#secretcli tx compute execute secret1844ks0ny2qgyru065mykut38cyph45wt6cxfj3 '{"change_admin":{"admin": "secret1mdz5rze0xfws4gjl8pw38qnfe77p276qnm82f8"}}' --from $deployer_name -y --gas 30000 -b block
#secretcli tx compute execute secret1844ks0ny2qgyru065mykut38cyph45wt6cxfj3 '{"start_mint":{}}' --from $deployer_name -y --gas 30000 -b block
#secretcli tx compute execute secret1844ks0ny2qgyru065mykut38cyph45wt6cxfj3 '{"stop_mint":{}}' --from $deployer_name -y --gas 30000 -b block

#echo "Enable Mints"
#secretcli tx compute execute $mint_address '{"start_mint":{}}' --from $deployer_name -y --gas 28000 -b block

echo "Mints"
#msg=$(base64 -w 0 <<<'{"mint_nfts": {"count": 50, "entropy": "'"$RANDOM"'"}}')
#secretcli tx compute execute $TOKEN_ADDRESS  '{"send":{"recipient": "'$mint_address'", "amount": "5000", "msg": "'"$msg"'"}}' --from $deployer_name -y --gas 1500000 -b block

echo "=================================="
echo "Mint Add: secret1844ks0ny2qgyru065mykut38cyph45wt6cxfj3"
echo "NFT Add: secret1kkj5ls0lfkk66e689ll68sntt0j3asvyw4nwnt"
echo "=================================="

#secretcli q compute query secret1kkj5ls0lfkk66e689ll68sntt0j3asvyw4nwnt '{"num_tokens":{}}' | jq .
#secretcli q compute query secret1844ks0ny2qgyru065mykut38cyph45wt6cxfj3 '{"info":{}}' | jq .