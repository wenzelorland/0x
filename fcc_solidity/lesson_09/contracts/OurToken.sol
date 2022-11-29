// contracts/OurToken.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

//https://github.com/OpenZeppelin/openzeppelin-contracts/tree/v4.4.0/contracts/token/ERC20
// view for implementation here
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract OurToken is ERC20 {
    // innitialSupply is measured in wei, i.e. in 1e18 units. initialSupply of 1 => 1e18. 
    constructor(uint256 initialSupply) ERC20("OurToken", "OT") {
        _mint(msg.sender, initialSupply);
    }
}
