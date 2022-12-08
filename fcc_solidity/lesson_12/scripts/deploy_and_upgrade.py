from scripts.helpful_scripts import get_account, encode_function_data, upgrade
from brownie import network, Box, ProxyAdmin, BoxV2, TransparentUpgradeableProxy, Contract


def main():
    account = get_account()
    print(f"Deploying to {network.show_active()}")
    # implementation contract
    box = Box.deploy({"from":account})
    print(box.retrieve())

    # attaching to proxy
    proxy_admin = ProxyAdmin.deploy({"from":account})

    # creating the initializer function, where the mapping is:
    # initializer = function_name, function_input_1, function_input_2...
    
    # encoding the initializer function
    #initializer = box.store, 1
    
    # box_encoded_initializer_function = encode_function_data(initializer)
    
    # for now we don't use the initializer function for the box contract
    # this is for the function selector
    box_encoded_initializer_function = encode_function_data()

    proxy = TransparentUpgradeableProxy.deploy(box.address, proxy_admin.address, box_encoded_initializer_function,
            {"from":account, "gas_limit":1000000})
    print(f"Proxy deployed to {proxy}, you ca now upgrade to v2!")

    # to call box.store(1) from the proxy we need to do the following
    # assigning the abi of the box contract (implementation contract) to the proxy contract
    # since this is the one which will eventually delegate all calls to the box contract.
    
    proxy_box = Contract.from_abi("Box", proxy.address, Box.abi)
    # this call will be delegated to the box contract although we are using the proxy.address
    proxy_box.store(1, {"from":account})
    print(proxy_box.retrieve())

    # upgrade the implementation contract
    box_v2 = BoxV2.deploy({"from":account})
    upgrade_tx = upgrade(
        account, 
        proxy,
        box_v2.address, 
        proxy_admin_contract=proxy_admin
    )

    upgrade_tx.wait(1)
    print("Proxy has been upgraded!")
    proxy_box = Contract.from_abi("BoxV2", proxy.address, BoxV2.abi)
    proxy_box.increment({"from":account})
    print(proxy_box.retrieve())
    
    # upgrade()
    proxy_box.increment({"from":account})
    print(proxy_box.retrieve())
    # this will now yield 2, since the proxy storage location already held the value = 1 since we stored it there when 
    # the proxy contract was still referencing the box (v1) contract.
    # Now, since it now references the v2 version and we increment the value (where in v2 the initial value is 0) 
    # it increments the value in the storage location of the proxy contract (which is 1 (not 0 as in box)), since it was already set.
    # ---> STORAGE Appending