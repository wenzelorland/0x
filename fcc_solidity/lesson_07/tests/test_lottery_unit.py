from brownie import Lottery, accounts, config, network, exceptions
from scripts.deploy_lottery import deploy_lottery
from scripts.helpful_scripts import LOCAL_BLOCKCHAIN_ENVIRONMENTS, get_account, fund_with_link, get_contract
import pytest

def test_get_entrance_fee():
    if network.show_active() not in LOCAL_BLOCKCHAIN_ENVIRONMENTS:
        pytest.skip()
    # Arrange
    lottery = deploy_lottery()
    # Act
    expected_entrance_fee = 0.025 * 1e18
    entrance_fee = lottery.getEntranceFee()
    # Assert
    assert expected_entrance_fee == entrance_fee

def test_cant_enter_unless_starter():
    if network.show_active() not in LOCAL_BLOCKCHAIN_ENVIRONMENTS:
        pytest.skip()
    lottery = deploy_lottery()
    with pytest.raises(exceptions.VirtualMachineError):
        lottery.enter({"from":get_account(), "value":lottery.getEntranceFee()})
    
def test_can_start_and_enter_lottery():
    # Arrange 
    if network.show_active() not in LOCAL_BLOCKCHAIN_ENVIRONMENTS:
        pytest.skip()
    lottery = deploy_lottery()
    account = get_account()
    lottery.startLottery({"from":account})
    # Act
    lottery.enter({"from":account, "value":lottery.getEntranceFee()})
    # Assert
    assert lottery.players(0) == account

def test_can_end_lottery():
    if network.show_active() not in LOCAL_BLOCKCHAIN_ENVIRONMENTS:
        pytest.skip()
    lottery = deploy_lottery()
    account = get_account()
    lottery.startLottery({"from":account})
    lottery.enter({"from":account, "value":lottery.getEntranceFee()})
    fund_with_link(lottery)
    lottery.endLottery({"from":account})
    assert lottery.lottery_state() == 2
 
# this is test is mroe like an integration test than an unit test
def test_can_pick_winner_correctly():
    if network.show_active() not in LOCAL_BLOCKCHAIN_ENVIRONMENTS:
        pytest.skip()
    lottery = deploy_lottery()
    account = get_account()
    lottery.startLottery({"from":account})
    lottery.enter({"from":account, "value":lottery.getEntranceFee()})
    lottery.enter({"from":get_account(index=1), "value":lottery.getEntranceFee()})
    lottery.enter({"from":get_account(index=2), "value":lottery.getEntranceFee()})
    fund_with_link(lottery)

    # mocking responses from the oracle node
    transaction = lottery.endLottery({"from":account})
    requestId = transaction.events["RequestedRandomness"]["requestId"]
    STATIC_RNG = 777
    get_contract("vrf_coordinator").callBackWithRandomness(requestId, STATIC_RNG, lottery.address, {"from":account})
    
    starting_balance_of_account = account.balance()
    balance_of_lottery = lottery.balance()

    # since 777 (=STATIC_RNG) % 3 = 0 -> the winner based on this chosen mock random number will be account[0]
    assert lottery.recentWinner() == account
    assert lottery.balance() == 0
    assert account.balance() == starting_balance_of_account + balance_of_lottery
