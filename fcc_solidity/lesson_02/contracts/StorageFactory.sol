// SPDX-License-Identifier: MIT

// Employing the Factory Pattern to recycle modules from smart contract code
// and having a meta module which deploys other contracts, i.e. a smart contract able to deploy other smart contracts
// Inheritance, Factory Pattern, and Interacting with External Contracts
// -> Factory Pattern
// -> Imports
// -> Deploy a Contract from a Contract
// -> Interact with a deployed Contract

pragma solidity >=0.6.0 < 0.9.0;

// this is the equivalent of copying every line of code in the SimpleStorage.sol and pasting it here above
// the compiler will recognise both contracts and will offer to deploy any of them
import "./SimpleStorage.sol"; 

contract StorageFactory{
    
    // keeping track of all the SimpleStorage smart contracts being deployed
    SimpleStorage[] public simpleStorageArray;
    
    // a function which leads to the deployment a a pre-specified smart contract code lying outside this contract's code
    function createSimpleStorageContract() public {
        // create a SimpleStorage object with the name simpleStorage with no inputs
        SimpleStorage simpleStorage = new SimpleStorage();
        simpleStorageArray.push(simpleStorage);
    }

    // Interacting with the newly deployed smart contracts
    function sfStore(uint256 _simpleStorageIndex, uint256 _simpleStorageNumber) public {
        // Address - which is needed to locate the smart contract

        // ABI - Application Binary Interface
        // receiving the ABI from the imported code
        // SimpleStorage(address(simpleStorageArray[_simpleStorageIndex]));
        
        // accessing the smart contract
        //SimpleStorage simpleStorage = SimpleStorage(address(simpleStorageArray[_simpleStorageIndex]));
        // callig the function on the chosen smart contract
        //simpleStorage.store(_simpleStorageNumber);

        // this can also be a one-liner
        SimpleStorage(address(simpleStorageArray[_simpleStorageIndex])).store(_simpleStorageNumber);
    }

    function sfGet(uint _simpleStorageIndex) public view returns (uint256) {
        return SimpleStorage(address(simpleStorageArray[_simpleStorageIndex])).retrieve();
    }

}

// Inheritance
// all the functions of SimpleStorage will also be available within the StorageFactoryAdv smart contract
contract StorageFactoryAdv is SimpleStorage {
    
    // keeping track of all the SimpleStorage smart contracts being deployed
    SimpleStorage[] public simpleStorageArray;
    
    // a function which leads to the deployment a a pre-specified smart contract code lying outside this contract's code
    function createSimpleStorageContract() public {
        // create a SimpleStorage object with the name simpleStorage with no inputs
        SimpleStorage simpleStorage = new SimpleStorage();
        simpleStorageArray.push(simpleStorage);
    }

    // Interacting with the newly deployed smart contracts
    function sfStore(uint256 _simpleStorageIndex, uint256 _simpleStorageNumber) public {
        // Address - which is needed to locate the smart contract

        // ABI - Application Binary Interface
        // receiving the ABI from the imported code
        // SimpleStorage(address(simpleStorageArray[_simpleStorageIndex]));
        
        // accessing the smart contract
        //SimpleStorage simpleStorage = SimpleStorage(address(simpleStorageArray[_simpleStorageIndex]));
        // callig the function on the chosen smart contract
        //simpleStorage.store(_simpleStorageNumber);

        // this can also be a one-liner
        SimpleStorage(address(simpleStorageArray[_simpleStorageIndex])).store(_simpleStorageNumber);
    }

    function sfGet(uint _simpleStorageIndex) public view returns (uint256) {
        return SimpleStorage(address(simpleStorageArray[_simpleStorageIndex])).retrieve();
    }

}