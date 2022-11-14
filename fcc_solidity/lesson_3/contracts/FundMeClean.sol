// SPDX-License-Identifier: MIT

pragma solidity >=0.6.6 < 0.9.0;

// importing the chainlink price feed contract from a chainlink repo

import "@chainlink/contracts/src/v0.8/interfaces/AggregatorV3Interface.sol";

// --- INTERFACES ---

// the interface is similar to structs as they define a certain type to interact with.
// Interfaces compile down to an ABI.
// The Application Binary Interface tells solidity and other programming languages how it can interface with another contract.
// I.e. what functions can be called on another contract.
// Anytime you want to interact with an already deployed smart contract, you will need an ABI.


// Chainlink Smart Contract Interface definition
// Note: Chainlink smart contracts only have information available on testnets, since only there are actually operating chainlink nodes available.
// This interface defines the "structure" of the smart contract which is deployed by chainlink
// and how to interact with it by calling the available functions on it.
// Copied from: https://github.com/smartcontractkit/chainlink/blob/develop/contracts/src/v0.8/interfaces/AggregatorV3Interface.sol

// we want this contract to be able to receive some kind of payment
contract FundMe {

    // --- CONSTRUCTOR --- //
    // The function that constructs the smart contract.
    // At the top the contract, one typically has a constructor
    // which is executed automatically when the contract is created.

    address public owner;

    constructor() public {
        // since this will be the very first call creating the smart contract, the owner will be the 
        // address which has created the smart contract.
        owner = msg.sender;
    }

    // create a mapping of addresses to sent amount
    mapping(address => uint256) public addressToAmountFunded;
    address[] public funders;

    // they function keyword "payable" makes this contract able to receive funds
    // this function can be used to pay for this -> specifically with Eth on Ethereum
    // this function will always have a value associated with it 
    function fund() public payable {

        // defining a minimum threshold
        uint256 minimumUSD = 50 * 1e18;

        // if requirement is not met, then execution will be stopped and reverted
        require(getConversionRate(msg.value) >= minimumUSD, "You need to spend more ETH!");

        // this is same as:
        /*
        if(msg.value < minimumUSD){
          revert?
        }
        */

        // msg. accesses the contents of the message call to this contract
        // msg.sender holds the address of the sender of the call and msg.value holds the sent amount
        addressToAmountFunded[msg.sender] += msg.value;
        funders.push(msg.sender);
        // what is the ETH -> USD conversion rate?
        // data.chain.link -> for data feeds on chainlink
    }


    // --- Withdraw function ---
    // the "this" keyword refers to the contract you are in.
    // in this context we would withdraw all the balance of the smart contract to the address which is calling the 
    // withdraw() function on this contract.
    // address(this) retreives the (this) contract's address
    
    /* 
    function withdraw() public payable {
      payable(msg.sender).transfer(address(this).balance);
    }
    */

    // --- MODIFIER ---
    // A modifier is used to change the behavior of a function in a declarative way.
    modifier onlyOwner{
      require(owner == msg.sender, "You are not the owner of this contract.");
      // run the rest of the code when require is checked
      _;
    } 

    // the withdraw() function can be executed by anyone - to avoid this we can do the following:
    function withdraw() public onlyOwner payable {
      // before executing the below, the contract will first execute the contents of the onlyOwner modifier
      payable(msg.sender).transfer(address(this).balance);
      // resetting the amount of contribution to the current balance of the contract to zero for each address
      for (uint256 funderIndex=0; funderIndex<funders.length; funderIndex++){
          address funder = funders[funderIndex];
          addressToAmountFunded[funder] = 0;
      }
      // reset the funders array
      funders = new address[](0);
    }

    // interacting with the Chainlink smart contract
    function getVersion() public view returns (uint256) {
        // type name = smartContract(addressOfTheSmartContract), where the address can be found in the chainlink documentation
        // e.g. 0xD4a33860578De61DBAbDc8BFdb98FD742fA7028e is the address of the smart contract for the Goerli testnet
        // this address is for ETH/USD. 
        // This can be deployed to the Goerli Testnet.
        AggregatorV3Interface priceFeed = AggregatorV3Interface(0xD4a33860578De61DBAbDc8BFdb98FD742fA7028e);
        // this line says that we have a contract that is located at the address 0xD4a33860578De61DBAbDc8BFdb98FD742fA7028e with the functions defined in the AggregatorV3Interface interface above
        return priceFeed.version(); // returning the version of the price feed
    }

    function getPrice() public view returns(uint256) {
        AggregatorV3Interface priceFeed = AggregatorV3Interface(0xD4a33860578De61DBAbDc8BFdb98FD742fA7028e);
        // safe the response in a tuple
        (
            uint80 roundId,
            int256 answer,
            uint256 startedAt,
            uint256 updatedAt,
            uint80 answeredInRound
        ) = priceFeed.latestRoundData();
        // ignoring values in the response values
        // (,int256 answer,,,) = priceFeed.latestRoundData();
        
        // type casting the return value
        return uint256(answer);
    }

    // retrieving the information of decimals of the returned smart contract value
    function getDecimals() public view returns(uint8) {
        AggregatorV3Interface priceFeed = AggregatorV3Interface(0xD4a33860578De61DBAbDc8BFdb98FD742fA7028e);
        return priceFeed.decimals();
    }

    // of 1 gwei == xUSD
    function getConversionRate(uint256 ethAmount) public view returns (uint256) {
        uint256 ethPrice = getPrice();
        uint8 Decimals = getDecimals();
        uint256 ethAmountInUsd = (ethPrice * ethAmount) / (uint256(Decimals) * 1e9);
        return ethAmountInUsd;
    } 
}