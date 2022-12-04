from brownie import network, exceptions
from scripts.helpful_scripts import LOCAL_BLOCKCHAIN_ENVIRONMENTS, get_account, get_contract, INITIAL_VALUE
from scripts.deploy import deploy_token_farm_and_dapp_token
import pytest

# ### Testing all functionalities from TokenFarm.sol in an isolated manner


# Testing if the price feeds are correctly set
def test_set_price_feed_contract():
    # Arrange
    if network.show_active() not in LOCAL_BLOCKCHAIN_ENVIRONMENTS:
        pytest.skip("Only for local testing!")
    account = get_account()
    non_owner = get_account(index=1)
    token_farm, dapp_token = deploy_token_farm_and_dapp_token()
    # Act
    eth_price_feed_address = get_contract("eth_usd_price_feed")
    token_farm.setPriceFeedContract(dapp_token.address, eth_price_feed_address, {"from":account})

    # Assert
    assert token_farm.tokenPriceFeedMapping(dapp_token.address) == eth_price_feed_address
    ## Testing if non-owner can set the price feed -> must fail if they can!
    with pytest.raises(exceptions.VirtualMachineError):
        token_farm.setPriceFeedContract(dapp_token.address, eth_price_feed_address, 
            {"from":non_owner})

def test_stake_tokens(amount_staked): # amount_staked is defined in conftest.py as a fixture
    # Arrange
    if network.show_active() not in LOCAL_BLOCKCHAIN_ENVIRONMENTS:
        pytest.skip("Only for local testing!")
    account = get_account()
    token_farm, dapp_token = deploy_token_farm_and_dapp_token()

    # Act
    dapp_token.approve(token_farm.address, amount_staked, {"from":account})
    token_farm.stakeTokens(amount_staked, dapp_token.address, {"from":account})

    # Assert
    assert (
        # since stakingBalance is a mapping to a mapping, 
        # we need to pass it two parameters, one for each level of the mapping
        token_farm.stakingBalance(dapp_token.address, account.address) == amount_staked
    )
    
    assert token_farm.uniqueTokensStaked(account.address) == 1
    assert token_farm.stakers(0) == account.address
    return token_farm, dapp_token

def test_issue_tokens(amount_staked):
    # Arrange
    if network.show_active() not in LOCAL_BLOCKCHAIN_ENVIRONMENTS:
        pytest.skip("Only for local testing!")
    account = get_account()
    token_farm, dapp_token = test_stake_tokens(amount_staked)
    starting_balance = dapp_token.balanceOf(account.address)
    # Act
    token_farm.issueTokens({"from":account})

    # Arrange
    # we are staking 1 dapp_token == in price to 1 ETH
    # ---> we should get 2,000 dapp tokens in reward
    # ---> 
    assert (
        dapp_token.balanceOf(account.address) == starting_balance + INITIAL_VALUE
    )
