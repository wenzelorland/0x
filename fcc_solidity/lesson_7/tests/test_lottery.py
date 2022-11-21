from brownie import Lottery, accounts, config, network
from web3 import Web3


def test_get_entrance_fee():
    account = accounts[0]
    lottery = Lottery.deploy(config["networks"][network.show_active()]["eth_usd_price_feed"], {"from":account},
    )
    assert lottery.getEntranceFee() > 0.03 * 1e18
    assert lottery.getEntranceFee() < 0.1 * 1e18