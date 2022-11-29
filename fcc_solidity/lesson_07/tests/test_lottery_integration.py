import pytest
from brownie import network
from scripts.helpful_scripts import get_account, get_contract, LOCAL_BLOCKCHAIN_ENVIRONMENTS, fund_with_link
from scripts.deploy_lottery import deploy_lottery
import time

#execute this with brownie test -k test_can_pick_winner --network goerli -s

def test_can_pick_winner():
    if network.show_active() in LOCAL_BLOCKCHAIN_ENVIRONMENTS:
        pytest.skip()
    lottery = deploy_lottery()
    account = get_account()
    lottery.startLottery({"from":account})
    lottery.enter({"from":account, "value":lottery.getEntraceFee()})
    lottery.enter({"from":account, "value":lottery.getEntraceFee()})
    fund_with_link(lottery)
    lottery.endLottery({"from":account})
    # wait for the chainlink node to respond
    time.sleep(60)
    assert lottery.recentWinner() == account
    assert lottery.balance() == 0