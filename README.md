Pulsar 1 Testnet:

Minter Address: secret1u9nuy46z7h39j4d90vrdxss2h2we803p3uafdr
SNIP721 Address: secret18cg62qlfcwv80zl9z79v9xzw6gsfq9vanczqkh
sSCRT Address: secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg

Mint NFT:
msg=$(base64 -w 0 <<<'{"mint_nfts": {"count": 50, "entropy": "'"$RANDOM"'"}}')
secretcli tx compute execute secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg '{"send":{"recipient": "secret1u9nuy46z7h39j4d90vrdxss2h2we803p3uafdr", "amount": "5000", "msg": "'"$msg"'"}}' --from test1 -y --gas 900000 -b block

Info Query:
secretcli q compute query secret1u9nuy46z7h39j4d90vrdxss2h2we803p3uafdr '{"info":{}}' | jq .
