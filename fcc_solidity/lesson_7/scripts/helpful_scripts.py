from brownie import network, config, accounts, Contract
from brownie import MockV3Aggregator, VRFCoordinatorMock, LinkToken

DECIMALS = 8
INITIAL_VALUE = 2000 * 10**DECIMALS
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

contract_to_mock = {
    "eth_usd_price_feed": MockV3Aggregator,
    "vrf_coordinator": VRFCoordinatorMock,
    "link_token": LinkToken,
}

def get_contract(contract_name):
    """ This function will grab the contract addresses from the brownie config if defined, otherwise, it will deploy a mock version of that contract, and return that mock contract.

        Args:
            contract_name (string)
        Returns:
            brownie.network.contract.ProjectContract: The most recently deployed version of this contract. -> [-1] 
    """
    contract_type = contract_to_mock[contract_name]
    if network.show_active() in LOCAL_BLOCKCHAIN_ENVIRONMENTS:
        # check if any mock contracts are already deployed
        if len(contract_type) < 1:
            # if none have been deployed, a new contract will be deployed
            deploy_mocks()
        contract = contract_type[-1]
    else:
        contract_address = config["networks"][network.show_active()][contract_name]
        # address
        # ABI
        contract = Contract.from_abi(contract_type._name, contract_address, contract_type.abi)
    return contract


def deploy_mocks(decimals=DECIMALS, initial_value=INITIAL_VALUE):
    account=get_account()
    MockV3Aggregator.deploy(decimals, initial_value, {"from":account})
    link_token = LinkToken.deploy({"from":account})
    VRFCoordinatorMock.deploy(link_token.address, {"from":account})
    
    print("Deployed!")

def fund_with_link(contract_address, account=None, link_token=None, amount=250000000000000000): #0.25 Link
    account = account if account else get_account()
    link_token = link_token if link_token else get_contract("link_token")
    tx = link_token.transfer(contract_address,amount, {"from":account})
    # instead of taking the link_token smart contract code directly, one can also interact with the interface of the smart contract
    # this is another way to interact with contracts that already exist
    # link_token_contract = interface.LinkTokenInterface(link_token.address)
    # tx = link_token_contract.transfer(contract_address, amount, {"from":account})
    tx.wait(1)
    print("Funded contract!")
    return tx