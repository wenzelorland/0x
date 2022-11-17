# Brownie manages most of the low level stuff which needed to be done manually in the previous lesson_4
# - it automatically dumps the JSON abi into build/contracts/
# - it spins up a local blockchain

# to run this script, type: brownie run scripts/deploy.py

# the accounts are a list of mapped accounts that are automatically mapped
# from the spun up local blockchain

# you can also add your own accounts to have preconfigured settings which may be more convenient to work with

from brownie import accounts, config # config imports the contents of the brownie-config.yaml
from brownie import SimpleStorage
from brownie import network
import os


# brownie run scripts


def deploy_simple_storage():
    # this is the account which is automatically created in the ganache session
    #account = accounts[0] # just take the first one which is initialized by Brownie
    account = get_account() # flexible switch between local and testnet
    #print(account)
    
    ### Accessing preconfigured / own private keys ###
    # retrieving an environment variable from the account list after configured brownie-config.yaml
    #account = accounts.add(os.getenv("PRIVATE_KEY"))
    
    # without using the os package and having configured the brownie-config.yaml file, one can use the following
    #account = accounts.add(config["wallets"]["from_key"])
    
    # this way, there is only one place where the private key is stored and one does not have to change it all the time
    # i.e. there is one canonical way for the private_key to be accessed
    print(account)

    # when deploying to the chain, one needs to specify from which address the call / transaction is send

    # Instead of having to specify everything and doing the full chain of signing, hashing and sending the transaction
    # with brownie this is all streamlined to be ready in one single call
    simple_storage = SimpleStorage.deploy({"from":account}) # state change functions need to have the "from" attribute 
    print(simple_storage)
    stored_value = simple_storage.retrieve() # since retrieve is a view function, we use a call and hence do not need to provide the account address
    print(stored_value)
    transaction = simple_storage.store(15, {"from":account})
    transaction.wait(1) # waiting for X confirmations
    updated_stored_value = simple_storage.retrieve()
    print(updated_stored_value)

def get_account():
    if(network.show_active() == "development"):
        return accounts[0]
    else:
        return accounts.add(config["wallets"]["from_key"])


def main():
    deploy_simple_storage()