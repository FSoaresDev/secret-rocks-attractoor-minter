Pulsar 1 Testnet:

Minter Address: secret12x6x35gs5zeg2dlqlf4g5n75lvglvlz2q3lv4g
SNIP721 Address: secret14ua3vl0dypf944966rlv5z4qcaqj7v3mcjrnje
sSCRT Address: secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg

Mint NFT:
msg=$(base64 -w 0 <<<'{"mint_nfts": {"count": 10, "entropy": "'"$RANDOM"'"}}')
secretcli tx compute execute secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg '{"send":{"recipient": "secret12x6x35gs5zeg2dlqlf4g5n75lvglvlz2q3lv4g", "amount": "90000000", "msg": "'"$msg"'"}}' --from test1 -y --gas 300000 -b block

Info Query:
secretcli q compute query secret12x6x35gs5zeg2dlqlf4g5n75lvglvlz2q3lv4g '{"info":{}}' | jq .
