from scripts.helpful_scripts import get_account
from brownie import interface, config, interface, network

def get_weth():
    """
    Mints WETH by depositing ETH
    """
    account = get_account()    
    weth = interface.IWeth(config["networks"][network.show_active()]["weth_token"])
    tx = weth.deposit({"from":account, "value": 0.1 * 1e18})
    tx.wait(1)
    print("Received 0.1 WETH")
    return tx

def main():
    get_weth()

