from scripts.helpful_scripts import get_account, get_contract
from brownie import OurToken, network, config
from web3 import Web3
import time

INITIAL_SUPPLY = Web3.toWei(1000, "ether")

def main():
    account = get_account()
    our_token = OurToken.deploy(INITIAL_SUPPLY, {"from":account})