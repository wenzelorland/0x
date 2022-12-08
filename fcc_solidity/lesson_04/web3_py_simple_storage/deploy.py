from solcx import compile_standard, install_solc
import json
from web3 import Web3 
import os
from dotenv import load_dotenv

# loading the .env file contents
load_dotenv()

install_solc("0.6.0")
with open("./SimpleStorage.sol", "r") as file:
    simple_storage_file = file.read()
    print(simple_storage_file)

# Compile the solidity code
compiled_sol = compile_standard(
    {
        "language": "Solidity",
        "sources": {"SimpleStorage.sol": {"content": simple_storage_file}},
        "settings": {
            "outputSelection": {
                "*": {"*": ["abi", "metadata", "evm.bytecode", "evm.sourceMap"]}
            }
        },
    },
    solc_version="0.6.0",
)
with open("compile_code.json", "w") as file:
    json.dump(compiled_sol, file)

print(compiled_sol)

# Deploying the smart contract
bytecode = compiled_sol["contracts"]["SimpleStorage.sol"]["SimpleStorage"]["evm"]["bytecode"]["object"]

# get abi
abi = compiled_sol["contracts"]["SimpleStorage.sol"]["SimpleStorage"]["abi"]
print(abi)


### --- Using web3py --- ###
# choosing the HTTP provider RPC server
# for connecting to Ganache
w3 = Web3(Web3.HTTPProvider("http://127.0.0.1:7545"))
chain_id = 5777
# when deploying to different Testnets or Mainnets, one can check the corresponding values here:
# https://chainlist.org

# determine the address - pick one from ganache
my_address = "0x3dc15c8643DBA0fFd93Fae74eEb382eAe4833c3F"
# for private keys in python, one always needs to add the "0x" in front of the key as python is looking for the hexadecimal version of the key value
private_key =  os.getenv("PRIVATE_KEY")#"0x26d7dbd74713681c8abfb34f0fcd637e2c714fecc91604ed8ddf7c51917e5d8c"
print(private_key)
# Create the contract in python
SimpleStorage = w3.eth.contract(abi=abi, bytecode=bytecode)

# check web3py documentation at:  https://web3py.readthedocs.io/en/v5/

# Account Nonces
# -> Account nonces just track the number of transactions made by this account address.

# receiving the current account nonce
nonce = w3.eth.getTransactionCount(my_address)
print(nonce)

# Building the transaction to interact with the blockchain
# 1. Build the contract deploy
# 2. Sign the transaction
# 3. Send the transaction

# building the transaction object
transaction = SimpleStorage.constructor().build_transaction(
    {
    "gasPrice": w3.eth.gas_price,
    "chainId": w3.eth.chain_id, 
    "from":my_address, 
    "nonce":nonce}
)

# signing the transaction with the private key 
signed_txn = w3.eth.account.sign_transaction(transaction, private_key=private_key)

# send the signed transaction
print("Deploying Contract.")
tx_hash = w3.eth.send_raw_transaction(signed_txn.rawTransaction)

tx_receipt = w3.eth.wait_for_transaction_receipt(tx_hash)
print("Contract Deployed!")

# Working with the contract, one always needs
# Contract Address
# Contract ABI

simple_storage = w3.eth.contract(address=tx_receipt.contractAddress, abi=abi)

# Interaction with a Smart Contract
# We can interact with a smart contract in the following ways:
# 1. Call 
# --> Simulate making the call and getting a return value -> they do not initiate a state change on the blockchain.
# ---> this is the same to just viewing the current state values of the contract.
# ---> Call functions essentially simulate the interaction with a contract to retrieve viewable values from the contract.
# 2. Transaction 
# --> Actually make a state change on the chain.
# ---> One can always attempt to make a transaction to just view the value but it will automatically attempt a state change.
# ---> So you can also invoke a view function through a Transaction, but it will not lead to a state change.

print("Calling a view function.")
# Calling the smart contract
print(simple_storage.functions.retrieve().call())

# this call will not change the state, since it is not a transaction
print("Calling a state changing function which does not alter the state, as we do not transact.")
print(simple_storage.functions.store(15).call())

# transaction
# -> create transaction
print("Creating a transaction to update contract.")
store_transaction = simple_storage.functions.store(15).buildTransaction(
    {
    "gasPrice": w3.eth.gas_price,
    "chainId": w3.eth.chain_id, 
    "from":my_address, 
    "nonce":nonce+1 # since we need to increment it for every transaction of the address
    }
)

# -> sign transaction
signed_store_txn = w3.eth.account.sign_transaction(store_transaction, private_key=private_key)
# -> hash the transaction and send it
print("Sending the transaction to the contract.")
store_txn_hash = w3.eth.send_raw_transaction(signed_store_txn.rawTransaction)
# -> wait for transaction receipt, i.e. confirmation
txn_store_receipt = w3.eth.wait_for_transaction_receipt(store_txn_hash)
print("Contract updated.")
# calling the contract for the updated value
print("Calling the updated state value of the contract.")
print(simple_storage.functions.retrieve().call())

"""
::: Setting Environment Variables in MacOS for safety reasons and to NEVER hardcode them in code ::: 
-> CLI: export PRIVATE_KEY=0x26d7dbd74713681c8abfb34f0fcd637e2c714fecc91604ed8ddf7c51917e5d8c
-> access it: echo $PRIVATE_KEY
-> this is only valid for the session of the shell

Alternatively, put it in an .env file in the same folder and put
export PRIVATE_KEY=0x26d7dbd74713681c8abfb34f0fcd637e2c714fecc91604ed8ddf7c51917e5d8c
in it.
Then load the python-dotenv library to read the .env file.
Don't forget to include the .env files in the .gitignore file so that when committing the code, you do not share the private key etc.
"""


"""
Using ganache CLI (ganache-cli is deprecated and now ganache)
npm install ganache --global
ganache --deterministic
The flag guarantuees that on each startup, the account private keys are the same so that one does not need to alter them all the time.

"""