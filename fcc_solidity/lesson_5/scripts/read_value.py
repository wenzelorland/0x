from brownie import SimpleStorage # this also serves as an array of previously deployed contracts
from brownie import accounts, config


# brownie run scripts/read_value.py --network goerli

def read_contract():
    simple_storage = SimpleStorage[-1] # this will access the most recent deployment made
    # ABI
    # Address
    # since this is an already deployed contract through brownie, brownie alreadys has the information on the ABI and address of the contract
    print(simple_storage.retrieve())

def main():
    read_contract()