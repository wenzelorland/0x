from brownie import network, AdvancedCollectible
import pytest
from scripts.helpful_scripts import LOCAL_BLOCKCHAIN_ENVIRONMENTS, get_contract, get_account
from scripts.advanced_collectible.deploy_and_create import deploy_and_create
import time

def test_can_create_advanced_collectible_integration():
    # Arrange
    if network.show_active() not in LOCAL_BLOCKCHAIN_ENVIRONMENTS:
         pytest.skip("Only for local testing")
    # Act
    advanced_collectible, creation_tx = deploy_and_create()
    time.sleep(60)
    # Assert
    assert advanced_collectible.tokenCounter() == 1
