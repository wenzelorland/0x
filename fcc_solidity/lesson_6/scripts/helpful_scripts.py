from brownie import network, config, accounts
from brownie import MockV3Aggregator

DECIMALS = 8
STARTING_PRICE = 2000 * 10**DECIMALS
LOCAL_BLOCKCHAIN_ENVIRONMENTS = ["development",  "ganache-local"]
FORKED_LOCAL_ENVIRONMENTS = ["mainnet-fork", 'mainnet-fork-dev']
# brownie run scripts/deploy.py --network mainnet-fork-dev

def get_account():
    if network.show_active() in LOCAL_BLOCKCHAIN_ENVIRONMENTS + FORKED_LOCAL_ENVIRONMENTS:
        return accounts[0]
    else:
        return accounts.add(config["wallets"]["from_key"])

def deploy_mocks():
    print(f"The active network is {network.show_active()}")
    print("Deploying Mocks ... ")
    # the contract class in brownie holds all deployed instances of its contract in an array which can be addresses through indexing
    if not len(MockV3Aggregator): # check of there are any aggregators already deployed, if not deploy a new one.
        # 2000 in Wei -> 2000 * 1e18
        MockV3Aggregator.deploy(DECIMALS, STARTING_PRICE, {"from":get_account()})
        print(f"Mock deployed at {MockV3Aggregator[-1]}")