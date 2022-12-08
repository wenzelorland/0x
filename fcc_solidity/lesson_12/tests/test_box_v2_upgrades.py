from scripts.helpful_scripts import get_account, encode_function_data, upgrade
from brownie import BoxV2, Box, Contract, TransparentUpgradeableProxy, ProxyAdmin, exceptions
import pytest

def test_proxy_upgrades():
    account = get_account()
    box = Box.deploy({"from":account})
    proxy_admin = ProxyAdmin.deploy({"from":account})
    box_encoded_initializer_function = encode_function_data()
    proxy = TransparentUpgradeableProxy.deploy(
        box.address,
        proxy_admin.address,
        box_encoded_initializer_function,
        {"from":account, "gas_limit":1000000}
    )

    # deploy boxV2
    box_v2 = BoxV2.deploy({"from":account})
    proxy_box = Contract.from_abi("BoxV2", proxy.address, BoxV2.abi)
    # the next will pass if the error is thrown, i.e. when the transaction reverts
    with pytest.raises(exceptions.VirtualMachineError):
        proxy_box.increment({"from":account}) # this can only revert since the box2 has not been implemented for proxy_box -> not upgraded yet
    
    upgrade(account, proxy, box_v2, proxy_admin_contract=proxy_admin)
    assert proxy_box.retrieve() == 0
    proxy_box.increment({"from":account})
    assert proxy_box.retrieve() == 1
    