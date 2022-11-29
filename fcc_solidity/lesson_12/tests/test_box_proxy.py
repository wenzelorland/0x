from scripts.helpful_scripts import encode_function_data, get_account
from brownie import Box, ProxyAdmin

def test_proxy_delegates_calls():
    account = get_account()
    box = Box.deploy({"from":account})
    proxy_admin = ProxyAdmin.deploy({"from":account})
    box_encoded_initializer_function = encode_function_data()
