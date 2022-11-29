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
- [x]  Lesson 9 - ERC20s, EIPs, and Token Standard
- [x]  Lesson 10 - Defi & Aave
- [x]  Lesson 11 - NFTs
  - [ ]  there is a bug when deploying to testnet, that is the transaction reverts because of gas estimation error within scripts/advanced_collectible/deploy_and_create.py -> line 18
- [x]  Lesson 12 - Upgrades
- [ ]  Lesson 13 - Full Stack Defi

## Smart Contracts
Smart contracts are a self executed set of instructions which is executed without the need of a 3rd party.

Chainlink allows for unlimited smart contract customization.

Hybrid smart contracts are smart contracts with an off-chain component.

Chainlink is blockchain agnostic.

### Features of Blockchains**

1. Decentralized → Blockchain is being run by independent node operators; no centralized party can control the whole system.
2. Transparency & Flexibility
   Everything is public and pseudo-anonymized through public - private key paris
    
3. Speed and Efficiency
4. Security and Immutability → cannot be tempered with or corrupted; even when mutliple operators fail; hacking the blockchain is so much harder than hacking a centralized system
5. Removal of counterparty risk → no conflict of interest for the potential intermediary in traditional systems
6. Allow for Trust Minimizied or Trustless Agreements → we do not have to trust the party that we engage with on a political brand based trust level since the code of the smart contract stands for itself
7. Hybrid Smart Contract combine on and off-chain worlds
   Enables a free(dom) & trustless Environment

dApp’s and smart contracts free the user of the instrinsic motivations behind intermediaries and deliver the instant and efficient nature of settlement of transactions. 

### DAOs

Decentralized Autonomous Organizations which are organized in a decentralized environment and are governed by the user of the respective tokens.

### Transaction Information

Transaction Hash uniquely identifies the transaction by its hash.

#### Gas

Gas is a unit of computational measure. The more computation a transaction uses, the more gass you have to pay for it. Every transaction that happens on-chain pays a gas-fee to node operators. Any time the state of the blockchain is changed it involves the the need to pay gas for the operation that led to the state transition. The amout of gas used and how much one pays depends on the computational complexity of the transaction. 

Sending ETH to 1 address, is cheaper than sending 1 ETH to 100 addresses. 

Gas: Measure of computation use

Gas Price: How much it costs per unit of gas

Gas Limit: Max amount of gas in a transaction

Transaction Fee: Gas Used x Gas Price

e.g. `21 000 gas @ 1 GWEI per gas = 21 000 GWEI = 21 000 * 1e-9 ETH`

[GasInfo](http://www.ethgasstation.info). Setting a higher Gas Price let you incentivice the node operators to pick up your transaction into the block faster than for transactions with a lower gas price. 

Gas price is based on the demand of computation on the particular blockchain and represents the “worth” of block space. 

#### Hash 

Unique fixed length string, meant to identify a piece of data. They are created by placing said data into a “hash function”. 

### Blockchain Information

[Blockchain Demo](https://andersbrownworth.com/blockchain/)

#### Node

A single instance in a network running the blockchain. 

#### Block Mining

Miners find a value for the integer Nonce, that together with the Data inside the block creates a hash which has a pre-determined amount of leading zero’s (determined by the difficulty target). They will essentially just try out multiple combinations until they have found one valid hash which meets the PoW criteria difficulty target. This can only be performed by brute-force trying. Blockchains may introduce slightly differenct concepts of mining, but mostly they are a variant of the above for proof of work blockchains. Mining is the process of finding the solution to the blockchain problem. Nodes get paid for mining blocks. A block is a list of transactions mined together.

[Blockchain Demo](https://andersbrownworth.com/blockchain/block)

#### Nonce

A number used once to find the solution to the blockchain number. In Ethereum it is also used to define the transaction number for an account / address.

### Blockchain

[Blockchain](https://andersbrownworth.com/blockchain/blockchain)

Because of the chained structure and the fact that each block is referencing its predecessor and the corresponding hash of the predecessor is also hashed to receive the current block’s hash, whenever someone tries to change / alter past data for past blocks they will instantly invalidate their blockchain state, as the hashes of the respective blocks would not reference the correct information anymore and would thus lead to an invalidation of the whole blockchain, so that node operators would not accept this blockchain version. → Immutability of blockchains.

#### Distributed

The decentralized nature of blockchains ensures that even if some operators fail, the blockchain still persists to exist and continues to work regardless of failing operating members. Since multiple operators are working on the same blockchain, it is infeasible for a single attacker to try to force an altered version of the blockchain to the whole network, since it would be instantly disregarded because their version is invalidated instantly by other operators when they try to validate the version of the attacker. The attacker then has essentially forked the blockchain which other operators will not accept and will continue to operate the original blockchain. They have no centralized point of authority. ⇒ Blockchains are resillient. A node acting maliciously will be swiftly cut out from the network.

[Blockchain Demo](https://andersbrownworth.com/blockchain/distributed)

#### Tokens 

Token transaction are placed into the block and then hashed with the block details. The transactions themselves are also hashed and referenced for later validation. This also leads to valid and temper-proof transactions which solve the double spending problem.

[Blockchain Demo](https://andersbrownworth.com/blockchain/tokens)

#### Coinbase Transaction

[Blockchain Demo](https://andersbrownworth.com/blockchain/coinbase)

#### Public and Private Keys

`Private Key` → is essentially a random integer of typically 256-bit length. 

`Public Key` → is the corresnponding value that corresponds to the public key value from a specific mathematical function (see elliptic curve cryptography)

One can sign a message with the private key, while no one can derive the private key from the signed message. 

However, they can validate with the help of the public key that it was indeed the private key that corresponds to that public key which was used to sign the message (signature). I.e. the signature matches to the corresponding public key without ever revealing the true private key.  

The Ethereum address is actually the public key hashed with Keccak-256 algorithm and take the last 20-bytes, which then corresponds to the address to that public key. 

#### Signing a transaction:

A “one way” process. Someone with a private key signs a transaction by their private key being hashed with their transaction data. Anyone can then verify this new transaction has with the corresponding public key. 

#### Consensus
Mechaism used in a network to agree on what the state of a blockchain is. 

**Traits of Consensus Mechanisms: 1) Chain Selection + 2) Sybil Resistance**

1. How do we know which current blockchain is the valid one? → which latest produce block is acutally the last valid one.
    1. E.g. whichever chains has the most number of blocks is declared the canonical chain, i.e. accepted as the valid current blockchain → Longest Chain Rule
    2. One can also use the number of confirmations as another way to determine whichever blockchain is the valid one
2. e.g. Proof of work or Proof of Stake, it defines a process to figure out who will be the block author and defines the ability of a blockchain to defend itself against multiple users creating pseudo-anon accounts to take over the blockchain by influencing the whole network
    1. In PoW a node has to go through a very expensive process of mining to come up with a valid block hash. This means that even if one tries to temper with the transaction data, one still needs to come up with the computational resources to actually create the transaction to be broadcasted to the network, making it prohibitively expensive to perform such Sybil attack

Nakamoto Consensus = Longest Chain is Valid Chain + Proof of Work, which Bitcoin uses.

Blocktimes correlate with the difficulty target, i.e. if it is more difficult to come up with the “right” block nonce, the block time will typically also be longer.

Block confirmations is the number of block added past the respective block. I.e. when the number =2 then 2 blocks have been added to the chain after this particular block which are chain-referencing this block and are ahead of it in the blockchain.

**Sybil Attack i) vs 51 % attack ii)**

i) references a situation where the attacker actually tries to attack the decentrality of the network by pretending to be multiple users while actually being one entity controlling multiple pseudo users.

ii) this refers to a situation where the attacker tries to gain 51% influence over the network so that she can decide which chain is the canonical and to enforce her version of the chain to be the canonical blockchain for the whole network to adapt, essentially forking the original chain which will then be adopted by the network.

The bigger the blockchain is the more difficult it is to attack it because of the decentralized nature of the blockchain.

## Proof of Stake (Pos)

Nodes put up collateral as a sybil resistance mechanism to show that they are going to behave in the network. When they misbehave, then their stake will be slashed as punishment. The nodes then become validors. Nodes are randomly chosen to become the proposer of new blocks. The rest of the validators then validate if the the proposer node has proposed a valid block. Since blockchains are fully deterministic systems, the randomness aspect is actually engineered through other means like RANDAO, which is a mechanism for generating “reasonably random” numbers in a decentralized way. 

The core idea behind RANDAO is to have a large group of people come together to generate a random number, instead of simply trusting one person to do it on everyone's behalf. This is what Ethereum uses.

### Sharding

Is essentially a concept of a a blockchain of blockchains which addresses the scalability aspect of blockchains. Since there are multiple blockchains (shards), the transaction throughput is siginificantly higher, as all these shards are ultimately merged into the “one” blockchain who keeps track of the overall cononical chain.  

**Layer 1**

Base layer blockchain implementation

**Layer 2**

Any application built on top of a layer 2.

**Rollups (layer 2s which also tackle the scalability aspect)**

A rollup is kind of like a sharded chain, which ultimately rolls up the transactions that happen on its layer to the layer 1 they are connected to. → See Optimism and Arbitrum for Ethereum. They adopt the security of the layer 1 while improving the scalability.

Sidechains are different from rollups as they do not derive the security from the layer 1 but from their own protocol implementation.

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

## Contract Interfaces
When saving a contract interface into brownie's *interfaces* folder, it is common practice to put an I (upper capital i) in front of the contract name.

I.e. when referring to Weth.sol one puts IWeth.sol.
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

Let brownie remember deployments to a ganache chain:
E.g. one can take a chain spun-up from the Ganache UI client.
```bash
brownie networks add Ethereum ganache-local host=http://127.0.0.1:7545 chainid=1337
```
or whatever parameters are specified for the local ganache blockchain.
To access this chain, we need to tell brownie which network to use
--> 
```bash
brownie run scripts/deploy.py --network ganache-local
```

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

Mainnet forking is the first choice for testing unit tests when no oracles are required.

If oracles are required, then development networks are the way to go since oracles can be mocked there.

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

## Adding Contract to Brownie
Contracts that are added into the contracts folder will all be compiled with brownie when ```brownie compile```.
This means that they are then accessible in the whole project context through 
```python
from brownie import CompiledContract
```

One can also add contracts through adding the associated interface of the contract into the interfaces folder.

When one is only interested in interacting with specific functions on a given contract, it is sufficient to just include those descriptions into the respective interface definition.

Interfaces can typically be found within the app docs, github repos or directly on etherscan (on the given contract address).


## Testing
One typically creates two folders in the tests folder -> `integration´ and *unit*
Within unit tests, you want to actually test every line of the smart contract.
Brownie command to run specific test on a particular network:
```bash
brownie test -k test_get_entrance_fee --network goerli
```

## Logging / Printing On-Chain
To record certain state changes and results of events within a smart contract, one can use events which will store the action and the corresponding result.
Events will be a container which one can push (emit) values to and then access it afterwards. 

## Brownie Mixes
github.com/brownie-nix/
-> this repo holds plenty resources for boilerplate code for smart contract deployment
To access any repo within this collection, you can just "brownie bake" it 
e.g. for the chainlink-mix repo do:

```bash
brownie bake chainlink-mix
```

## ERC-20 Tokens
[ERC20](https://eips.ethereum.org/EIPS/eip-20) tokens come with a very simple mapping of who holds the token and how much of it.
To create an ERC-20 token, one needs to create a contract with the functions and requirements as defined in [EIP-20](https://eips.ethereum.org/EIPS/eip-20).
There are multiple templates available open source, which can be used as a base to create an ERC-20 token from scratch.
Visit [OpenZeppelin Code Resources on ERC-20](https://docs.openzeppelin.com/contracts/4.x/erc20) for reference on building an ERC-20 contract.
In this setup, one can declare the ownership of the contract and so on. Also the initial distribution schedule can be difined here.
Additional functionalities, like a burn mechanism can also be added and some template can be found [here](https://github.com/OpenZeppelin/openzeppelin-contracts/tree/release-v4.8/contracts/token/ERC20).

ERC20 tokens have their own permission system. Every address that owns an ERC-20 token needs to specifically allow the use of the token by smart contract before interacting with a corresponding smart contract. This means that one has to approve that a contract account can withdraw these tokens.

The ERC20 contract itself holds this function 
```solidity
function approve(address spender, uint256 value) external returns (bool success);
```
## DeFi
### WETH (Gateway)
WETH is an ERC-20 version of ETH. 
WETH-Gateways are used to change the native ETH on Ethereum into WETH, which can then be used to interact with other ERC-20 tokens / contracts.
Depositing ETH into the WETH contract will mint WETH token and transfers it the sender address.

To get the ETH out, the withdraw function can be invoked which will burn the WETH tokens and return the ETH tokens to the sender address.

### Retrieving User Data from Aave
Aave LendingPool also offers to retrieve the UserInfoData -> getUserAccountData.

```python
(total_collateral_eth, 
    total_debt_eth, 
    available_borrow_eth, 
    current_liquidation_threshold, 
    ltv, 
    health_factor) = lending_pool.getUserAccountData(account.address)
```

Aave also offers [risk parameters](https://docs.aave.com/risk/v/master/asset-risk/risk-parameters) within its documenation.
E.g. Ethereum currently has an LTV of `75%`, which means that you can only borrow at max `75%`of your collateral in ETH. 
Whenever your LTV goes above this figure, the position will be liquidated.

## NFTs - ERC-721 Tokens
[ERC721](https://eips.ethereum.org/EIPS/eip-721) contracts have unique tokenId's and each Id has a unique owner. They have a specific URI (essentially metadata to the token / NFT).
The URI is a universal and unique indicator of what token / asset looks like and how the attributes look like.

### NFT URI ((distinct) Uniform Resource Identifier for a given asset) and Metadata
> A URI is a string containing characters that identify a physical or logical resource. URI follows syntax rules to ensure uniformity. Moreover, it also maintains extensibility via a hierarchical naming scheme. The full form of URI is Uniform Resource Identifier

A typical token URI returns the name of the token, the description, the imageUri (separate URI for the image) and any attributes.
In practice, this metadata is either stored on centralized solutions or within IPFS (which also has its own centralization aspect although being decentralized in the setup).

Images (metadata) are typically attached to NFTs the following way:
1. Get IPFS
2. Add tokenURI json file to IPFS
3. Add IPFS URI to NFT URI

The ERC721 is a factory contract. This means there is a main contract, wherin there is a list of all the NFTs and the owners of this type of NFT. 

### Metadata Attributes
Metadata attributes are which describe the functionality / utility of an NFT. For full functionality for your NFTs, one needs to put these attributes on-chain. The tokenURIs are then used just for the visuals.
Otherwise, if one doesn't store the attributes on-chain, they aren't temper proof and can be changed depending on where they are stored, e.g. on if not stored on IPFS but on a central server which can be altered by the owner.

### Viewing the NFT
The ERC721 has an metadata section, but storing image data on-chain is very(**!!!**) expensive. This is why they are typically stored off-chain.
The ERC721 token has a specific **metadata secion** where the tokenURI can be retrieved for outside reference for the NFT (type) contract. 

### IPFS (Inte-Planetary File Storage)
Anbody who is participating in IPFS can hook up images to their node that they are then replicating on their node. 
Not every image is automatically replicated across the whole network. This is because of hard-disc space constraints and network congestion.
If at least 1 node is hosting an image, then every node on the network has access to this image and host it itself.
[IPFS-Setup](https://docs.ipfs.tech/install/command-line/) and [concepts](https://docs.ipfs.tech/concepts/) and [documentation](https://docs.ipfs.tech/reference/http/gateway/#api), and [IPFS-RPC-documenation](https://docs.ipfs.tech/reference/kubo/rpc/#origin-based-security).

Everything within IPFS is hashed and has every data pieces stored there have an associated unique hash uniquely identifying the data.

**Note**
If you are the only one who is keeping this image of the data on your node, then whenever the nodes is not online, i.e. active, then no one can see the data that is being hosted on the node. E.g. when having uploaded an NFT image and your node is the only one hosting that image data, then whenever it is offline, the metadata will also be offline and not reachable for others.

#### Backup Upload to Pinata --> Pinning Data to IPFS
Because of this, it is best practice to also upload the metadata to some service which maintains its services around the clock so that at least whenever your node is down / not active, users / people still have access to the metadata of the image / NFT. 

Pinata is an IPFS file management service and they will pin whatever data your node is running, ensuring that you are at least not the only one hosting these images. These services are typically not for free, thought they also come with some free tiering. 

#### Starting Own IPFS Node
```bash
ipfs daemon
```

## Smart Contracts Upgrades
Smart contracts deployed on-chain in general are immutable. The state of the smart contract changes and may be changes but the implementation of the smart contract itself is immutable.

There are ways to make smart contracts upgradeable.
### Smart Contract Setups / Upgrade Methods
1. Not really upgradeable but parametrizable
2. Social Yeet / Migration
3. Proxies

#### Parametrization
This refers to just updating existing parameters within the smart contract without touching the actual implementation of the smart contract itself.
- Can't add new storage or state variables
- Can't add new logic
- Updating certain parameters

This is not really upgrading but interacting with the implemented logic which maybe has some switches in place to steer the contract behaviour.
It is really simple to implement but really not that flexible. 

The questions arises who has the privilige to alter these parameteres.
#### Social YEET / Migration
This referes to deploying a new contract without any dependencies to the old contract while cheering the community to migrate to the new version of the contract.
Pros:
- Truest to blockchain values
- Easiest to audit

Cons:
- Lot of work to convince users to move
- Different addresses

#### Proxies
Proxies refer to a setup where the user is interacting with a proxy instead of interacting with the corresponding **implementation** smart contract itself.
This means that the proxy takes care addressing / interacting with the corresponding smart contracts. 
In case a new version of the smart contract which is interacted with is deployed, then the address change / and behavior change can be addressed with the proxy itself without the user ever noticing.

It effectively means that the proxy takes the responsibility of taking care of all the low level code instructions through the low-level delegatecall method.

> ***Delegatecall*** - There exists a special variant of a message call, named `delegatecall`which is identical to a message call apart from the fact that the code at the target address is executed in the context of the calling contract `msg.sender`and `msg.value` do not change their values.
> The code of the target contract is executed in the context of the calling contract (proxy). 

This effectively means that if Contract**A** executes a delegatecall to Contract**B** then the code from Contract**B** is executed within the context of Contract**A**.
E.g. the delegecall calls a function to set a value to a certain figure. Then instead of setting this value within Contract**B**, the value is set for Contract**A**.

The proxy is there to be able to route to the corresponding implementation smart contracts which are currently in use. In case of an upgrade to one of the implementation smart contracts, one just needs to amend the respective address within the Proxy contract for the new implementation contract address.

For instance, one could create a function which takes care of setting the new implementation address.

```solidity
function upgrade (address newImplementation) external (
    if (msg.sender != admin) fallback () ;
    implementation = newImplementation;
}
```

##### Terminology

1. **<span style="color:blue"> The Implementation Contract </span>**

   Which has all our code of our protocol. When we upgrade, we launch a brand new implemenation contract.
2. **<span style="color:blue"> The Proxy Contract </span>**

   Which points to which implementation is the "correct" one, and routes everyone's function calls to that contract
3. **<span style="color:blue"> The User </span>**

   They make calls to the proxy
4. **<span style="color:blue"> The Admin </span>**

   This is the user (or group of users/voters) who upgrade to new implementation contracts.

#### Issues with Using Proxies

1. **Storage Clashes**

   In a delegateCall, one uses the logic of ContractB inside ContractA. The issue is that the delegateCall will change values in ContractA in the same storage location not necessarily the same value names.

   Contract A delegatesCall ContractB:

   Contract A:

   (value is overwritten in contractA)

   ```solidity
   function doDelegateCall() {
      callContractB(setValue())
   }
   ```

   Contract B (upgraded contract):

   ```solidity
   uint256 public differentValue;
   uint256 public value;

   function setDValue(uint256 _differentValue) {
      differentValue = _differentValue;
   }

   fuction setValue(uint256 _value) {
      value = _value;
   }
   ```

   Functions point to storage spots in solidity, not to the value names.

   ```solidity
   uint256 value;
   uint256 differentValue;

   function setValue(uint256) {
      value = 2;
   }
   ```

   Here **value** is at storage location 0, and **differentValue** is at storage location 1 (0 indexed). **setValue** actually sets the value of whatever is at storage location 0. This means that we can only apend new storage variables in new implementation contracts and one cannot reorder / change old ones.

2. **Function Selector Clashes**

   When a delegateCall is made, a function selector is used to find the corresponding functions. 
   > Function Selector:
   > A 4 ybte hash of a function name and function signature that define a function.

   It is possible that a function in the implementation contract has the same function selector as an admin fuction in the proxy contract, which may lead to clashes. These functions can be totally different but they can still have the same function selector.


### Storage Appending

Imagine we have proxy contract which references and delegates call to the implementation contract box.

- Now the proxy delegateCalls a function to set `public value = 1`
- Now we upgrade the proxy to reference boxV2, which has an increment function that increments the `value`by $+1$, with `value` being initialized as $0$. The proxy delegateCalls the increment function
- When one reads the value of `value` on the proxy contract, it will show **$2$**. The reason for this is because we are working on the storage of the proxy contract, where `value` is being incremented in the storage location of the proxy contract
- So instead of returning the `value` $1$ (which would be the case when we purely worked on boxV2 instead of delegating through proxy), it returns the `value` $2$.

### Proxy Implementation Patterns

Implementation methods to address the issues of proxy contracts in general.
There are also multiple EIPs which try to unite a common standard when handling with proxies. E.g. see [EIP-1967: Proxy Storage Slots](https://eips.ethereum.org/EIPS/eip-1967).

It is also common practive to set up a designated proxy admin. This can be done through e.g. a multi-sig account.

Typically, when working with proxies, implementation contracts do not have a constructor. To imitate the constructor, one typically has an "initializer" function which is called instead.

#### 1) Transparent Proxy Pattern

Admins can't call implementation contract functions and are only allowed to call admin functions which govern the upgrades. 

Users can only call functions in the implementation contract but not admin functions.

#### 2) Universal Upgradeable Proxies UUPs

Here all the logic of upgrading is actually put into the implementation contract itself.
AdminOnly Upgrade functions are in the implementation contracts instead of the proxy.

This also saves gas in the execution, since the admin check within the proxy contract does not have to be performed anymore. 
The issue arises when one deploys the implementation contract without any upgradeability aspect, which makes it impossible to use wihtin this pattern.

In this setup, the references within the upgradeable proxy are upgraded once the new version of the implementation contract has been deployed to correctly reference the new version -> see Lesson_12.

#### 3) Diamond Pattern

This allows for multiple implementation contracts. This allows to employ a split conde base which does not have to be maintained within one specific contract. Since it is composed of multiple parts, one does not have to change / upgrade the whole contract everytime when there is a need to fix a bug or implement a new feature. This can then just be performed on the respective implementation contract. 
The disadvantages here are that potentially the entire code base gets convoluted and harder to maintain with the split introducing more overall complexity and potentially leading to on average higher gas requirements.
