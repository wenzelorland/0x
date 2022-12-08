from scripts.helpful_scripts import get_account
from brownie import config, network, interface
from scripts.get_weth import get_weth

AMOUNT = 0.1 *1e18

def main():
    account = get_account()
    erc20_address = config["networks"][network.show_active()]["weth_token"]
    if network.show_active() in ["mainnet-fork"]:
        get_weth()
    
    # ABI
    # Address
    lending_pool = get_lending_pool()
    approve_erc20(AMOUNT,lending_pool.address, erc20_address, account)
    print("Depositing")
    # check in the documentation for the fields
    # -> token to be deposit, amount to be deposited, and on_behalf_address, i.e. for which address
    tx = lending_pool.deposit(erc20_address, AMOUNT, account.address, 0, {"from":account})
    tx.wait(1)
    print("Deposited.")

    ### How much can we borrow?
    # 0.1 ETH deposited
    # 0.0825 borrowable -> bceause of liquidation threshold
    borrowable_eth, total_debt = get_borrowable_data(lending_pool, account)
    print("Let's borrow!")
    # DAI in terms of ETH
    if network.show_active() == 'mainnet-fork':
        dai_eth_price = get_asset_price(config["networks"][network.show_active()]["dai_eth_price_feed"])
    elif network.show_active() == 'goerli':
        dai_usd_price = get_asset_price(config["networks"][network.show_active()]["dai_usd_price_feed"])
        eth_usd_price = get_asset_price(config["networks"][network.show_active()]["eth_usd_price_feed"])
        
        dai_eth_price = dai_usd_price / eth_usd_price
    # borrowable eth to borrowable dai * .95%
    amount_dai_to_borrow = (1/dai_eth_price) * (borrowable_eth * .95) # .95 as 5% buffer
    print(f"We are going to borrow {amount_dai_to_borrow} of DAI")

    # now the acutal borrowing
    dai_address = config["networks"][network.show_active()]["dai_token"]
    #.borrow(asset, amount, interestRateMode, referralCode, onBehalfOf)
    borrow_tx = lending_pool.borrow(dai_address, 
        amount_dai_to_borrow*1e18, 1, 0, account.address, {"from":account})
    borrow_tx.wait(1)
    print("We borrowed DAI from AAVE!")
    get_borrowable_data(lending_pool, account)

    repay_all(amount_dai_to_borrow*1e18, lending_pool, account)
    print("We just deposited, borrowed, and repaid with Aave, Brownie and Chainlink")
    get_borrowable_data(lending_pool, account)

def repay_all(amount, lending_pool, account):
    # need to approve the payback
    dai_token = config["networks"][network.show_active()]["dai_token"]
    approve_erc20(amount, 
        lending_pool, 
        dai_token,
        account
    )
    repay_tx = lending_pool.repay(
        dai_token,
        amount,
        1,
        account.address,
        {"from":account}
    )
    # wait for 1 confirmation
    repay_tx.wait(1)
    print("Repaid!")


def get_asset_price(price_feed_address):
    dai_eth_price_feed = interface.AggregatorV3Interface(price_feed_address)
    latest_price = dai_eth_price_feed.latestRoundData()[1] / 1e18
    print(f"The Price is {latest_price}")
    return float(latest_price)

def get_borrowable_data(lending_pool, account):
    (total_collateral_eth, 
    total_debt_eth, 
    available_borrow_eth, 
    current_liquidation_threshold, 
    ltv, 
    health_factor) = lending_pool.getUserAccountData(account.address)
    available_borrow_eth /= 1e18
    total_collateral_eth /= 1e18
    total_debt_eth /= 1e18
    print(f"You have {total_collateral_eth} worth of ETH deposited.")
    print(f"You have {total_debt_eth} worth of ETH borrowed.")
    print(f"You can borrow {available_borrow_eth} worth of ETH.")
    return (float(available_borrow_eth), float(total_debt_eth))

def approve_erc20(amount, spender, erc20_address, account):
    """
    Function to approve the right to the sender to withdraw / spend an amount of erc20 token with address erc20_address
    """
    print("Approving ERC20 token...")
    erc20 = interface.IERC20(erc20_address)
    tx = erc20.approve(spender, amount, {"from":account})
    tx.wait(1)
    print("Approved!")
    return tx


def get_lending_pool():
    lending_pool_addresses_provider = interface.ILendingPoolAddressesProvider(
        config["networks"][network.show_active()]["lending_pool_addresses_provider"]
    )
    # retrieving the currently valid lending pool address
    lending_pool_address = lending_pool_addresses_provider.getLendingPool()
    lending_pool = interface.ILendingPool(lending_pool_address)
    print(lending_pool)
    return lending_pool