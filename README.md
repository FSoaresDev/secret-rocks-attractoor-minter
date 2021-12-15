# Mainnet

## Minter Address: secret1844ks0ny2qgyru065mykut38cyph45wt6cxfj3

## NFT Collection Address: secret1kkj5ls0lfkk66e689ll68sntt0j3asvyw4nwnt

# Testnet

Pulsar 1 Testnet:

Minter Address: secret136kk6q75g9xly7jqufpaue56zc0nptp2ehsa54
SNIP721 Address: secret1vmvz3f4cg6zzttc70z347jxldmhgg5vpl38cpd
sSCRT Address: secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg

Mint NFT:
msg=$(base64 -w 0 <<<'{"mint_nfts": {"count": 10, "entropy": "'"$RANDOM"'"}}')
secretcli tx compute execute secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg '{"send":{"recipient": "secret12x6x35gs5zeg2dlqlf4g5n75lvglvlz2q3lv4g", "amount": "90000000", "msg": "'"$msg"'"}}' --from test1 -y --gas 300000 -b block

Info Query:
secretcli q compute query secret136kk6q75g9xly7jqufpaue56zc0nptp2ehsa54 '{"info":{}}' | jq .
