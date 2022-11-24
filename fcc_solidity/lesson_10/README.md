## Setup
1. Swap ETH for WETH
2. Deposit some ETH into Aave
3. Borrow some asset wit the ETH collateral
   1. Sell that borrowed asset (short seeling)
4. Repay everything back

## Aave Setup
- AddressesProvider
  LendingPool address -> this will be retrieved from the `LendingPoolAddressesProvider`.
  This contracts holds an addresses register of the protocol for a particular market and it is immutable. 
  The `LendingPoolAddressesProviderRegistry` holds the register of active `LendingPoolAddressesProvider`.

While addresses of lending pools might change, the addresses of these two contracts never change.

## Testing
For integration testing we are using **Goerlie**.
For unit testing we ware using a **Mainnet-fork**.
