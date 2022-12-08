from brownie import accounts, network, config

LOCAL_BLOCKCHAIN_ENVIRONMENTS = ["development",  "ganache-local"]
FORKED_LOCAL_ENVIRONMENTS = ["mainnet-fork", 'mainnet-fork-dev']
# brownie run scripts/deploy.py --network mainnet-fork-dev

def get_account(index=None, id=None):
    # accounts[0] -> brownie ganache accounts
    # accounts.add("env")
    # accounts.load("id")
    if index:
        return accounts[index]
    if id:
        return accounts.load(id)
    if network.show_active() in LOCAL_BLOCKCHAIN_ENVIRONMENTS + FORKED_LOCAL_ENVIRONMENTS:
        return accounts[0]
    
    return accounts.add(config["wallets"]["from_key"])

