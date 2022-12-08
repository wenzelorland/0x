// SPDX-License-Identifier: MIT

pragma solidity >=0.6.0 < 0.9.0; //sdadas
//pragma solidity ^0.6.0;// means that the version is above 0.6.0, but below 0.7.0

// contract initiatlization
contract SimpleStorage {

    // value types
    //uint256 favoriteNumber = 5; // same as uint 
    bool favoriteBool = false;
    string favoriteString = "String"; // strings are actually not a value type but an array of bytes
    int256 favoriteInt = 5;
    address favoriteAddress = 0xAFb71C8b4Fe63E0cF2945AE34A6366189e3e77CC;
    bytes32 favoriteBytes = "cat"; // max size is bytes32

    // declaring variables as public will lead to them be accessible from outside the smart contract
    // i.e. this can actually be explored from outside the contract
    // uint256 public favoriteNumber;
    uint256 favoriteNumber; // it will initialize to = 0
    
    // visibility within Solidity //
    // external
    // -> part of the contract interface, i.e. can be called from other contracts via transactions
    // -> they cannot be called internally, i.e. f() does not work, but this.f() works.

    // public
    // -> part of contract interface and can either be called internally or via messages
    // -> for public state variables, an automatic getter function is generated

    // internal
    // -> default visiblity level for state variables
    // -> functions and state variables can only be accessed internally, i.e. only from current contract or contract deriving from it
    // -> without using "this".

    // private
    // -> only visible for the contract they are defined in and not in derived contracts

    // structs allow to define new types in solidity
    struct People {
        uint256 favoriteNumber;
        string name;
    }

    // initializing the defined People type
    People public person = People({favoriteNumber:2, name:"Patrick"});
    
    // initializing dynamic arrays; fixed size arrays have an integer in the brackets [1]
    People[] public people;

    // mapping (like dictionaries for python)
    mapping(string => uint256) public nameToFavoriteNumber;

    // functions
    function store(uint256 _favoriteNumber) public {
        // since favoriteNumber is defined in the global scope, we can access it here
        favoriteNumber = _favoriteNumber;
        uint256 test = 5; // this is only available in this scope
    }

    // Defining return types of functions with keyword view and pure
    // view -> means that we are just reading the value, i.e. this does not invoke a state change
    // pure -> functions that execute some operation without setting any value
    function retrieve() public view returns(uint256) {
        return favoriteNumber;
    }

    function pure_add_self(uint256 favoriteNumber) public pure {
        favoriteNumber + favoriteNumber;
    }

    // public variables automatically are also "view" functions
    // State-Changing function calls are transactions

    // keyword ::: memory vs storage
    // the keyword "memory" means that the value will only be stored during execution
    // the keyword "storage" means that the value will persist even after the execution of the function
    // because string is technically an object, we have to decide where to store it; i.e. in memory or in storage

    // storage means keep it forever
    function addPerson(string memory _name, uint256 _favoriteNumber) public {
        people.push(People(_favoriteNumber, _name));
        nameToFavoriteNumber[_name] = _favoriteNumber;
    }
}
