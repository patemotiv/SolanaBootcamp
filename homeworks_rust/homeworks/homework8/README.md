## Creating a fungible token

Create token:
```
patrick@Pats-MacBook-Pro-M3 ~: spl-token create-token
Creating token 2gngQvrGikG8SPYhGifwzeh8uVGLQ1gSTCGgDB3ytnDG under program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA

Address:  2gngQvrGikG8SPYhGifwzeh8uVGLQ1gSTCGgDB3ytnDG
Decimals:  9

Signature: 2Ejj8HB8aUeVSUixNmHYGYDntYAhh5A7xL9BKshWtU83m3sM7wJZUhU7X2bV5KJ46YdrzgVe3XHuTj38DLXFtR6i
```

Check supply:
```
patrick@Pats-MacBook-Pro-M3 ~: spl-token supply 2gngQvrGikG8SPYhGifwzeh8uVGLQ1gSTCGgDB3ytnDG
0
```

Create an account to hold the token:
```
patrick@Pats-MacBook-Pro-M3 ~: spl-token create-account 2gngQvrGikG8SPYhGifwzeh8uVGLQ1gSTCGgDB3ytnDG
Creating account 2yYRPmfu8gtaDW1da1uCRhipuPiNzRFVzBDiYtezoPyS

Signature: w2y2aK3e7kD71bjz5SANRmtHyopqb3e6GuPFzYQdNCP7Q3QJYbMmuUD1fAJmzd2fm1m6k7y9bvNV3bKZYfmd9A4
```

Mint some tokens into that account:
```
patrick@Pats-MacBook-Pro-M3 ~: spl-token mint 2gngQvrGikG8SPYhGifwzeh8uVGLQ1gSTCGgDB3ytnDG 1000
Minting 1000 tokens
  Token: 2gngQvrGikG8SPYhGifwzeh8uVGLQ1gSTCGgDB3ytnDG
  Recipient: 2yYRPmfu8gtaDW1da1uCRhipuPiNzRFVzBDiYtezoPyS

Signature: 2UutttsXfRSWhyzQT88gdxmKVNcwvkZCURuBX5gwEg5UCtDPtnsvEFcYjxBqhtrumjsHDmSBAmxcT7xtTs6PrroB
```

Check the token balance and supply:
```
patrick@Pats-MacBook-Pro-M3 ~: spl-token balance 2gngQvrGikG8SPYhGifwzeh8uVGLQ1gSTCGgDB3ytnDG
1000

patrick@Pats-MacBook-Pro-M3 ~: spl-token supply 2gngQvrGikG8SPYhGifwzeh8uVGLQ1gSTCGgDB3ytnDG
1000
```

Summary of the tokens that you own:
```
patrick@Pats-MacBook-Pro-M3 ~: spl-token accounts
Token                                         Balance
-----------------------------------------------------
2gngQvrGikG8SPYhGifwzeh8uVGLQ1gSTCGgDB3ytnDG  1000

patrick@Pats-MacBook-Pro-M3 ~: spl-token account-info 2gngQvrGikG8SPYhGifwzeh8uVGLQ1gSTCGgDB3ytnDG

SPL Token Account
  Address: 2yYRPmfu8gtaDW1da1uCRhipuPiNzRFVzBDiYtezoPyS
  Program: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
  Balance: 900
  Decimals: 9
  Mint: 2gngQvrGikG8SPYhGifwzeh8uVGLQ1gSTCGgDB3ytnDG
  Owner: 2inUkGEvNwYsFDRkUHv1KHfNHCXn6WtTgimCu3wJ3vok
  State: Initialized
  Delegation: (not set)
  Close authority: (not set)
```

Transfer some tokens to another account:
(use `--fund-recipient` if another account is not setup for that token)

```
patrick@Pats-MacBook-Pro-M3 ~: spl-token transfer --fund-recipient 2gngQvrGikG8SPYhGifwzeh8uVGLQ1gSTCGgDB3ytnDG 100 9fYuGnVB3y4hoicgME3DWT9km1zKXoWjJDTACEEnXyhs
Transfer 100 tokens
  Sender: 2yYRPmfu8gtaDW1da1uCRhipuPiNzRFVzBDiYtezoPyS
  Recipient: 9fYuGnVB3y4hoicgME3DWT9km1zKXoWjJDTACEEnXyhs
  Recipient associated token account: 7ZXMETXboxRyWUwuS5hY1Z2ccfxCH4SVeA7KU3oJzFGK
  Funding recipient: 7ZXMETXboxRyWUwuS5hY1Z2ccfxCH4SVeA7KU3oJzFGK

Signature: 5m4ZpQjHpA219urm5HEZvof1pQwHRAwA2BY3KfCDmLwqppvM565rDMzoPbTKCJ6QwT8hWV2g42YZpKKKcPXzyTUm
```

## Creating a Non-fungible token

Create a token with zero decimal places:
```
patrick@Pats-MacBook-Pro-M3 ~: spl-token create-token --decimals 0 
Creating token EcCioXri2nzoKMymNaexRZo6MViKkHKBNQfRsThRsgeY under program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA

Address:  EcCioXri2nzoKMymNaexRZo6MViKkHKBNQfRsThRsgeY
Decimals:  0

Signature: 4gPqWinXzrztUXCDknJfoQmkZq91HUfHtFvnnpKM6fEjbAWQAcc6PsCTSaEXsMgScNrgqq7iUZT5Am6hZp2gdH2b
```

Setup the account as for a fungible token:
```
patrick@Pats-MacBook-Pro-M3 ~: spl-token create-account EcCioXri2nzoKMymNaexRZo6MViKkHKBNQfRsThRsgeY
Creating account 3oQpb9gFBybpXqSxD46SAAqmov1uJeRThePG2wEtfomY

Signature: sWHqm7Auo8udosd3nd63g1Y9vrVPuT1U1AsUDSekxA9qDA8nYKyuRJSb9M5yPsXog7zgAtGZeXgEe3rbq8Bz6Bi
```

Mint 1 token to that account:
```
patrick@Pats-MacBook-Pro-M3 ~: spl-token mint EcCioXri2nzoKMymNaexRZo6MViKkHKBNQfRsThRsgeY 1 3oQpb9gFBybpXqSxD46SAAqmov1uJeRThePG2wEtfomY
Minting 1 tokens
  Token: EcCioXri2nzoKMymNaexRZo6MViKkHKBNQfRsThRsgeY
  Recipient: 3oQpb9gFBybpXqSxD46SAAqmov1uJeRThePG2wEtfomY

Signature: 2MrrC2sophkxde7gR79Cu8rwmdSs7ZWA6Kw34ATHjYLXPB1iVZD8rjJ1oxuKuZQvVrTUbyN1fHrYY1TjTqTJmZ6u
```

Disable future minting:
```
patrick@Pats-MacBook-Pro-M3 ~: spl-token authorize EcCioXri2nzoKMymNaexRZo6MViKkHKBNQfRsThRsgeY mint --disable 
Updating EcCioXri2nzoKMymNaexRZo6MViKkHKBNQfRsThRsgeY
  Current mint: 2inUkGEvNwYsFDRkUHv1KHfNHCXn6WtTgimCu3wJ3vok
  New mint: disabled

Signature: B7rRCbpzyVyiPfdVPodZBHY4dRvTuqY9xT63CXS4uTW71F7fBfVcgSTbEGDf4Cz1Uzjcm6Ph4CVY9uFicojYcWL
```

Check the token details:
```
patrick@Pats-MacBook-Pro-M3 ~: spl-token account-info EcCioXri2nzoKMymNaexRZo6MViKkHKBNQfRsThRsgeY

SPL Token Account
  Address: 3oQpb9gFBybpXqSxD46SAAqmov1uJeRThePG2wEtfomY
  Program: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
  Balance: 1
  Decimals: 0
  Mint: EcCioXri2nzoKMymNaexRZo6MViKkHKBNQfRsThRsgeY
  Owner: 2inUkGEvNwYsFDRkUHv1KHfNHCXn6WtTgimCu3wJ3vok
  State: Initialized
  Delegation: (not set)
  Close authority: (not set)

patrick@Pats-MacBook-Pro-M3 ~: spl-token supply EcCioXri2nzoKMymNaexRZo6MViKkHKBNQfRsThRsgeY
1

patrick@Pats-MacBook-Pro-M3 ~: spl-token balance EcCioXri2nzoKMymNaexRZo6MViKkHKBNQfRsThRsgeY
1
```
