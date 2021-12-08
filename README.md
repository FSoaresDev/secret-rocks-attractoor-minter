Pulsar 1 Testnet:

Minter Address: secret13ca2lrue7pdzuwygsjv9dzwhmsmxys42zxcjya
SNIP721 Address: secret14879t6mjnhvmc863679e5hgl22dlx5e6j929xl
sSCRT Address: secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg

Mint NFT:
msg=$(base64 -w 0 <<<'{"mint_nfts": {"count": 10, "entropy": "'"$RANDOM"'"}}')
secretcli tx compute execute secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg '{"send":{"recipient": "secret13ca2lrue7pdzuwygsjv9dzwhmsmxys42zxcjya", "amount": "90000000", "msg": "'"$msg"'"}}' --from test1 -y --gas 300000 -b block

Info Query:
secretcli q compute query secret13ca2lrue7pdzuwygsjv9dzwhmsmxys42zxcjya '{"info":{}}' | jq .
