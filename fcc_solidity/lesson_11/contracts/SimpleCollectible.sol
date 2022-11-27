// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";

// inherit the functions from the ERC721 contract
contract SimpleCollectible is ERC721 {
    
    uint256 public tokenCounter;

                           // name and symbol
    constructor () public ERC721 ("Dogie", "DOG") {
        tokenCounter = 0;
    }

    // function to mint the NFT
    // this will be assigning a tokenId to the owner calling this function
    function createCollectible(string memory tokenURI) public returns (uint256) {
        uint256 newTokenId = tokenCounter;
        _safeMint(msg.sender, newTokenId); // inherited from ERC721
        _setTokenURI(newTokenId, tokenURI); // set the tokenURI for accessing the actual image
        tokenCounter = tokenCounter +1;
        return newTokenId;
    }

    // token URI

}
