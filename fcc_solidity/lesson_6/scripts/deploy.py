from brownie import FundMe, MockV3Aggregator, network, config
from scripts.helpful_scripts import get_account, deploy_mocks

def deploy_fund_me():
    account = get_account()
    # because this is a state change, we need to specify the account
    # passing the address of the price feed 
    # --> if we are on a persistent network, like Goerli, use the associated address, otherwise deploy mocks
    # -> Here we can differentiate which networks we want to access and which respective address to pull
    
    # --> This setup allows for local and testnet agnostic deployments <-- 
    if network.show_active() != 'development':
        price_feed_address = config["networks"][network.show_active()]["eth_usd_price_feed"]
    else:
        deploy_mocks()
        price_feed_address = MockV3Aggregator[-1].address # use most recently deployed mock aggregator contract 


    fund_me = FundMe.deploy(
        price_feed_address,
        {"from":account},
        publish_source=config["networks"][network.show_active()].get("verify"), # publish_source works only on the test_net, so instead of = True, we make it agnostic to testnet and local chains by specifying the corresponding flag in the brownie-config.yaml
    )

    print(f"Contract deployted to {fund_me.address}") # ,publish_source=True) for automatically publishing the associated code of the deployed smart contract



def main():
    deploy_fund_me()


