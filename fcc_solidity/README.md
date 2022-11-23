# FreeCodeCamp Solidity Python Course
[Course-Link](https://github.com/smartcontractkit/full-blockchain-solidity-course-py)

## Status
- [x]  Lesson 0 - Welcome to Blockchain
- [x]  Lesson 1 - Welcome to Remix
- [x]  Lesson 2 - Storage Factory
- [x]  Lesson 3 - Fund Me
- [x]  Lesson 4 - [Web3.py](http://Web3.py) Simple Storage
- [x]  Lesson 5 - Brownie Simple Storage
- [x]  Lesson 6 - Brownie Fund Me
- [x]  Lesson 7 - Lottery - SmartContract-Mix
- [x]  Lesson 8 - Chainlink Mix
- [ ]  Lesson 9 - ERC20s, EIPs, and Token Standard
- [ ]  Lesson 10 - Defi & Aave
- [ ]  Lesson 11 - NFTs
- [ ]  Lesson 12 - Upgrades
- [ ]  Lesson 13 - Full Stack Defi

## Brownie Networks
Brownie comes with pre-configured networks which are addressable.
You can view the list of available networks through "brownie networks list".
Networks which are listed under the tab "Development" are all temporary networks.
After the scripts completes, these networks are erased and spun up fresh when a new script is executed.

Networks under "Ethereum" are persisted.

NOTE: For Infura, the PROJECT_ID is now called the API-Key under "ENDPOINTS" of the projects.
Deploying to the testnet:
```bash
brownie run scripts/deploy.py --network goerli
````

Once deployed to the network, everything that is deployed will be saved in the build/deployments folder for future references like contract addresses etc.
Changes to local blockchains will not be saved here.

Brownie console  
```bash
brownie console
``` 
, here we can directly interact with the deployed contracts and addresses.
It's essentially an interactive python shell with browny in the background

## External Dependencies
Telling brownie external dependencies by including this into the brownie-config.yaml file
dependencies:
'# - <organization/repo>@<version>
- smartcontractkit/chainlink-brownie-contracts@1.1.1
compiler:
  solc:
    remappings:
      - '@chainlink=smartcontractkit/chainlink-brownie-contracts@1.1.1'

This allows for the access to all elements in this github repo.
Furthermore, we tell the compiler that @chainlink refers to the github repo above through a remapping.

Brownie does also have a connection to etherscan.io, where one can process the verification of the smart contracts
in a programmatic way.
This is for publishing the source code of the smart contract.


## Mocking Oracle Feeds on Local Chains
Instead of referencing an external oracle source, one can deploy a mock contract feed on the local chain to 
receive a respective mock pricing feed for testing.
- creating a new test folder in the contracts folder - this is typically where the mock contracts are stored in the brownie folder structure.
- For ready to use mock oracle smart contracts, one can use chainlink-mix as a good reference.
https://github.com/smartcontractkit/chainlink-mix

## Attaching a local Ganache instance to Brownie
brownie networks list

Let brownie remeber deployments to a ganache chain:
```bash
brownie networks add Ethereum ganache-local host=http://127.0.0.1:7545 chainid=1337
```
or whatever parameters are specified for the local ganache blockchain.
To access this chain, we need to tell brownie which network to use
--> 
```bash
brownie run scripts/deploy.py --network ganache-local
````

For this we also need to amend the helpful_scripts.py, so that we expand the definition of "development" networks.
Since we have added the local blockchain no to the Development section of the networks, Brownie will actually save all interactions with contracts
in the deployment folder.
In case the local blockchain instance is deleted, the deployments folder referencing this chain needs to be deleted, as one will not be able to interact
with these contracts anymore.

Generally, in the networks section of the brownie-config.yaml the default is set to : development, i.e. when no --network flag is set, brownie will default and spun up a development chain
One can adjust this to another network, so that the default case will be adjusted.

## Mainnet Forking
A forked blockchain takes an exact copy of an existing blockchain. The advantage here is that the forked blockchain already comes with all 
the contracts deployed of the to be forked blockchain, including the Price feed contracts, transaction data and all protocol contracts.
This is why it is perfectly suited to use a forked Mainnet for simulation.

### Creating Own custom forked network
## Forking from infura has some performance issues though
```bash
brownie networks add development mainnet-fork-dev cmd=ganache-cli host=http://127.0.0.1 fork='https://mainnet.infura.io/v3/$WEB3_INFURA_PROJECT_ID' accounts=10 mnemonic=brownie port=7545
```
## Forking from alchemy
```bash
brownie networks add development mainnet-fork-dev cmd=ganache-cli host=http://127.0.0.1 fork='https://eth-mainnet.g.alchemy.com/v2/$WEB3_ALCHEMY_API_KEY' accounts=10 mnemonic=brownie port=7545
```
-> note: make sure the port you are using is the same as in the ganache GUI, when starting up ganache

## Test Deployment
Where should the test run?
1. Brownie Ganache Chain with Mocks : Should always pass.
2. Testnet: Always should pass (but only for Integeration testing)
3. Brownie mainnet-fork: Optional to pass
4. Custom mainnet-fork: Optional to pass
5. Self/Local Ganache: Not necessary, but good for tinkering

## Lesson_7
1. Users can enter lottery with ETH based on a USD fee
2. An admin will choose when the lottery is over
3. The lottery will select a random winner

How to Test:
1. mainnet-fork
2. development
3. testnet

## Randomness in Decentralized Systems
- since blockchain is deterministic system, random numbers cannot be just generated the classic way since validation of random numbers is not feasible,
- as results within validation computations could vary vastly and thus no consensus would be found eventually
- i.e. the parties will not be able to agree on the random number

- solution: base the random numbers based on attributes from the system itself -> pseudo-random numbers
- truly random numbers are impossable for decentralized systems by design!

Pseudo-random numbers can be hacked / exploited, when insecure pseudo-random number methods are used!
- a very bad way of generating a pseudo-random number is taking a globally available number in the system, e.g. a chainid, blocknumber etc and hash it or e.g. msg.value
- msg.send is also a globally available variables which can be found in the solidity documentation, e.b. block.difficulty
- you are essentially taking seemingly random numbers and shuffling them together and hashing then
- the problem is that it can be reverse engineered, as the hashing functions fully deterministic
- hashing does not introduce any randomness

## Secure Random Numbers in Decentralized Systems

https://docs.chain.link/vrf/v2/subscription/examples/get-a-random-number 

Chainlink offers a solution for this with Chainlink VRF (verifiable random function) -> provides verifiable randomness.

In essence, there is an onchain smart contract by chainlink which verifies the random number that is originating from the chainlink node.
This on-chain consumer contract needs to be funded first, to make the transaction to the oracle to be able to pay the gas needed for the oracle request.
First the contract is requesting oracle node, then the oracle node executes a function within the smart contract and effectively stores the random value into the smart contract by changing its state.
This way, we can now access the random number on the smart contract that we have deployed.

The process is represented by 2 -asynchroneous transactions.
Technically, the smart contract is calling the VRF coordinator, which the node reads. The node than calls the VRF coordinator to call the fulfillRandomness callback function on the deployed smart contract.
First, requestRandomness is called from the VRFConsumerBase contract and then the callback rawfulfillRandomness (in the VRFConsumerBase contract) is called from the chainlink node (through the VRFCoordinator) preocessing the random number and thus storing it within the VRFConsumerBase deployed contract.

For a chainlink oracle one has to pay oracle gas, which is denominated in LINK.
For price-feeds, there are available sponsors which are paying for these oracles and thus they are offered for free.

Getting a random number follows the request and receive route.

## Addind Contract to Brownie
Contracts that are added into the contracts folder will all be compiled with brownie when brownie compile.
This means that they are then accessible in the whole project context through 
```python
from brownie import 
```

## Testing ###
One typically creates two folders in the tests folder -> `integration´ and *unit*
Within unit tests, you want to actually test every line of the smart contract.
Brownie command to run specific test on a particular network:
```bash
brownie test -k test_get_entrance_fee --network goerli
```

### Logging / Printing On-Chain ###
To record certain state changes and results of events within a smart contract, one can use events which will store the action and the corresponding result.
Events will be a container which one can push (emit) values to and then access it afterwards. 

### Brownie Mixes ###
github.com/brownie-nix/
-> this repo holds plenty resources for boilerplate code for smart contract deployment
To access any repo within this collection, you can just "brownie bake" it 
e.g. for the chainlink-mix repo do:

````bash
brownie bake chainlink-mix