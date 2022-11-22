1. Users can enter lottery with ETH based on a USD fee
2. An admin will choose when the lottery is over
3. The lottery will select a random winner

How to Test:
1. mainnet-fork
2. development
3. testnet

### Randomness in Decentralized Systems
-> since blockchain is deterministic system, random numbers cannot be just generated the classic way since validation of random numbers is not feasible,
-> as results within validation computations could vary vastly and thus no consensus would be found eventually
-> i.e. the parties will not be able to agree on the random number

--> solution: base the random numbers based on attributes from the system itself -> pseudo-random numbers
--> truly random numbers are impossable for decentralized systems by design!

-> pseudo-random numbers can be hacked / exploited, when insecure pseudo-random number methods are used!
// a very bad way of generating a pseudo-random number is taking a globally available number in the system, e.g. a chainid, blocknumber etc and hash it
// or e.g. msg.value
// msg.send is also a globally available variables which can be found in the solidity documentation, e.b. block.difficulty
// you are essentially taking seemingly random numbers and shuffling them together and hashing then
// the problem is that it can be reverse engineered, as the hashing functions fully deterministic
// hashing does not introduce any randomness

### -> Secure random numbers <-- ###
-> https://docs.chain.link/vrf/v2/subscription/examples/get-a-random-number 
Chainling offers a solution for this with Chainlink VRF (verifiable random function) -> provides verifiable randomness.
In essence, there is an onchain smart contract by chainlink which verifies the random number that is originating from the chainlink node.
This on-chain consumer contract needs to be funded first, to make the transaction to the oracle to be able to pay the gas needed for the oracle request.
First the contract is requesting oracle node, then the oracle node executes a function within the smart contract and effectively stores the random value into the smart contract by changing its state.
This way, we can now access the random number on the smart contract that we have deployed.
The process is represented by 2 -asynchroneous transactions.
Technically, the smart contract is calling the VRF coordinator, which the node reads. The node than calls the VRF coordinator to call the fulfillRandomness callback function on the deployed smart contract.
First, requestRandomness is called from the VRFConsumerBase contract and then the callback rawfulfillRandomness (in the VRFConsumerBase contract) is called from the chainlink node (through the VRFCoordinator) preocessing the random number and thus storing it within the VRFConsumerBase deployed contract.

For chainlink oracle, one has to pay oracle gas, which is denominated in LINK.
For price-feeds, there are available sponsors which are paying for these oracles and thus they are offered for free.

Getting a random number follows the request and receive route.


### Addind Contract to Brownie ###
Contracts that are added into the contracts folder will all be compiled with brownie when brownie compile.
This means that they are then accessible in the whole project context through from brownie import ...

### Testing ###
One typically creates two folders in the tests folder -> `integration´ and *unit*
Within unit tests, you want to actually test every line of the smart contract.
Brownie command to run specific test on a particular network:
brownie test -k test_get_entrance_fee --network goerli
