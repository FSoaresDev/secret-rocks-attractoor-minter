TOKEN_ADDRESS=secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg
TOKEN_HASH=9587d60b8e6b078ace12014ceeee089530b9fabcd76535d93666a6c127ad8813
label=$(date +"%T")
test1_address=secret1l5sjtktcuh004gsfwht536t6xjru0meas6vhke

#NFT REFERENCE
#deployed=$(secretcli tx compute store "./snip721_reference_impl.wasm" --from test1 --gas 250000000 -b block -y)
#code_id=$(secretcli query compute list-code | jq '.[-1]."id"')
#code_hash=$(secretcli query compute list-code | jq '.[-1]."data_hash"')
#echo $code_id $code_hash
nft_code_id=136
nft_code_hash=414974411AA86579DF6E3BBF2B3717E503E60E4AC3C09FC3D76C87726070ADA6

#MINTER
#deployed=$(secretcli tx compute store "../target/wasm32-unknown-unknown/release/secret_rocks_attractoor_minter.wasm" --from test1 --gas 2500000 -b block -y)
#code_id=$(secretcli query compute list-code | jq '.[-1]."id"')
#code_hash=$(secretcli query compute list-code | jq '.[-1]."data_hash"')
#echo $code_id $code_hash
minter_code_id=137
minter_code_hash=C3B29D4E628AA586D0192383462B5283BE6A3D8BD68075E1BA17D27B87997DBE

secretcli tx compute instantiate $minter_code_id " \
  { \
    \"prng_seed\": \"ZW5pZ21hLXJvY2tzCg==\", \
	  \"token_contract\": { \"address\": \"$TOKEN_ADDRESS\", \"contract_hash\": \"$TOKEN_HASH\"}, \
    \"mint_limit\": 5, \
    \"mint_price\": \"100000\", \
    \"giveaways\": [\"$test1_address\"], \
    \"utilities\": [{\"traits\": [{\"trait_type\": \"Utility 1\",\"value\": \"Secret Rock Airdrop\"},{\"trait_type\": \"Utility 2\",\"value\": \"Burning Mechanism\"},{\"trait_type\": \"Utility 3\",\"value\": \"Unknown\"}]},{\"traits\": [{\"trait_type\": \"Utility 1\",\"value\": \"Secret Rock Airdrop\"},{\"trait_type\": \"Utility 2\",\"value\": \"Staking Mechanism\"},{\"trait_type\": \"Utility 3\",\"value\": \"Unknown\"}]}] \
  } \
  " --from test1 --gas 15000000 --label test1_$label -b block -y |
  jq .

mint_address=$(secretcli query compute list-contract-by-code $minter_code_id | jq -r '.[-1].address')

# NFT DEPLOY
secretcli tx compute instantiate $nft_code_id '{"name":"Secret Rocks Attractoors "'$label'"", "symbol":"SRATT", "admin": "'$mint_address'", "entropy":"ZW5pZ21hLXJvY2tzCg==", "royalty_info":{"decimal_places_in_rates": 2, "royalties": [{"recipient": "secret1r4gka3q0zcner6vg629e887a6wejpy00djwlk6", "rate": 10}]}, "config":{"public_token_supply":true,"public_owner":false,"enable_sealed_metadata":false,"unwrapped_metadata_is_private":false,"minter_may_update_metadata":false,"owner_may_update_metadata": false,"enable_burn": true}}' --label test2_$label --from test1 --gas 15000000 -y -b block | jq .

nft_address=$(secretcli query compute list-contract-by-code $nft_code_id | jq -r '.[-1].address')

echo "Mint Add: $mint_address"
echo "NFT Add: $nft_address"

echo "Adding NFT contract to minter"
secretcli tx compute execute $mint_address '{"add_nft_contract":{"contract": {"address": "'$nft_address'", "contract_hash": "'$nft_code_hash'"}}}' --from test1 -y --gas 1500000 -b block

echo "Mint giveaways"
#secretcli tx compute execute $mint_address '{"mint_giveaways":{}}' --from test1 -y --gas 1500000 -b block

echo "Enable Mints"
secretcli tx compute execute $mint_address '{"start_mint":{}}' --from test1 -y --gas 1500000 -b block

echo "Mints"
#msg=$(base64 -w 0 <<<'{"mint_nfts": {"count": 2, "entropy": "'"$RANDOM"'"}}')
#secretcli tx compute execute $devSNIP20_address  '{"send":{"recipient": "'$mint_address'", "amount": "200", "msg": "'"$msg"'"}}' --from test1 -y --gas 1500000 -b block


msg=$(base64 -w 0 <<<'{"mint_nfts": {"count": 2, "entropy": "'"$RANDOM"'"}}')
secretcli tx compute execute secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg  '{"send":{"recipient": "secret1atx5hpvpa5vr92gakjc0nxka6wt2t0rykt3ldl", "amount": "200000", "msg": "'"$msg"'"}}' --from test1 -y --gas 1500000 -b block
