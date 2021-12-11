Pulsar 1 Testnet:

Minter Address: secret1lv7ph6v02p68sa43tmfwtvnkgy9vmlad38cekg
SNIP721 Address: secret1cte9aq8kjn60hw5ny2sy568q27vghg5wxuea3g
sSCRT Address: secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg

Mint NFT:
msg=$(base64 -w 0 <<<'{"mint_nfts": {"count": 10, "entropy": "'"$RANDOM"'"}}')
secretcli tx compute execute secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg '{"send":{"recipient": "secret13ca2lrue7pdzuwygsjv9dzwhmsmxys42zxcjya", "amount": "90000000", "msg": "'"$msg"'"}}' --from test1 -y --gas 300000 -b block

Info Query:
secretcli q compute query secret13ca2lrue7pdzuwygsjv9dzwhmsmxys42zxcjya '{"info":{}}' | jq .
