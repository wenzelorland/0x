from brownie import accounts, network, config
LOCAL_BLOCKCHAIN_ENVIRONMENTS = ["hardhat", "development", "ganache", "ganache-local", "mainnet-fork"]
import eth_utils


def get_account(index=None, id=None):
    if index:
        return accounts[index]
    if network.show_active() in LOCAL_BLOCKCHAIN_ENVIRONMENTS:
        return accounts[0]
    if id:
        return accounts.load(id)
    return accounts.add(config["wallets"]["from_key"])


def get_contract(contract_name):
    
    """
    This function will either:
        - Get an address from the config
        - Or deploy a Mock to use for a network that doesn't have the contract

    Args:
        contract_name (string): This is the name of the contract that we will get
        from the config or deploy

    Returns:
        brownie.network.contract.ProjectContract: This is the most recently deployed
        Contract of the type specified by a dictionary. This could either be a mock
        or a 'real' contract on a live network.
    """
    # link_token
    # LinkToken
    contract_type = contract_to_mock[contract_name]
    if network.show_active() in LOCAL_BLOCKCHAIN_ENVIRONMENTS:
        if len(contract_type) <= 0:
            deploy_mocks()
        contract = contract_type[-1]
    else:
        contract_address = config["networks"][network.show_active()][contract_name]
        contract = Contract.from_abi(
            contract_type._name, contract_address, contract_type.abi
        )
    return contract

# initializer = box.store, 1
 
def encode_function_data(initializer=None, *args):
    """Encodes the function call so we can work with an initializer.
        Args:
            initializer ([brownie.network.contract.Contract×], optional):
            The initializer function we want to call. Example: 'box.store.
            Defaults to None.
            args (Any, optional):
            The arguments to pass to the initializer function
        Returns:
            [bytes]: Return the encoded bytes.
    """
    if not len(args) or not initializer:
        return eth_utils.to_bytes(hexstr="0x")
    return initializer.encode(input(*args))

def upgrade(account, proxy, new_implementation_address, proxy_admin_contract=None, initializer=None, *args):
    transaction = None
    if proxy_admin_contract:
        if initializer:
            encoded_function_call = encode_function_data(initializer, *args)
            transaction = proxy_admin_contract.upgradeAndCall(
                proxy.address,
                new_implementation_address,
                encoded_function_call,
                {"from":account}
            )
        else:
            transaction = proxy_admin_contract.upgrade(proxy.address, new_implementation_address, {"from":account})
    
    else:
        if initializer:
            encoded_function_call = encode_function_data(initializer, *args)
            transaction = proxy.upgradeToAndCall(new_implementation_address, {"from":account})
        else:
            transaction = proxy.upgradeTo(new_implementation_address, {"from":account})
    return transaction