# Dapp Development

## ERC20 Notes
ERC20 contracts have a function `transfer` and `transferFrom`, where the first can be called from the owner of the tokens.
The function `transferFrom` is used for withdraw workflows, allowing contracts to transfer token on the user's behalf. This can be used for example to allow a contract to transfer tokens on your behalf and/or to charge fees in sub-currencies.

For instance, the contract can transfer your tokens from your address to its own address on your behalf.
