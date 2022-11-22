// SPDX-License-Identifier: MIT

pragma solidity ^0.6.6;

import "@chainlink/contracts/src/v0.6/interfaces/AggregatorV3Interface.sol";
import "@openzeppelin/contracts/access/Ownable.sol"; // we can also take the onlyOwner accessControl from openzeppelin and we also need to add it to the brownie-config.yaml
import "@chainlink/contracts/src/v0.6/VRFConsumerBase.sol";

// Lottery is inhertiting from VRFConsumerBase and Ownable
contract Lottery is VRFConsumerBase, Ownable{
    address payable[] public players;
    address payable public recentWinner;
    uint256 public randomness;
    uint256 public usdEntryFee;
    AggregatorV3Interface internal ethUsdPriceFeed;
    // enums are similar to struct, however, they can directly be converted to integer types,
    // which means that OPEN represents state 0, CLOSED = state 1, CALCULATING_WINNER = state 2
    enum LOTTERY_STATE {
        OPEN, 
        CLOSED,  
        CALCULATING_WINNER}
    LOTTERY_STATE public lottery_state;
    uint256 public fee;
    bytes32 public keyhash;
    
    // adding an additional constructor for the VRFConsumerBase contract
    // -> multi-level constructor to account for constructors for multiple contracts which are deployed in the process
    constructor(
        address _priceFeedAddress, 
        address _vrfCoordinator, 
        address _link,
        uint256 _fee,
        bytes32 _keyhash
    ) 
        public 
        
        VRFConsumerBase(_vrfCoordinator, _link) 
    {
        usdEntryFee = 50 * 1e18; // in Wei
        ethUsdPriceFeed = AggregatorV3Interface(_priceFeedAddress);
        lottery_state = LOTTERY_STATE.CLOSED; // i.e. initialized as CLOSED -> same as lottery_state = 1; 
        // needed for the requestRandomness function from VRFConsumerBase contract which is inherited
        fee = _fee;
        keyhash = _keyhash;
        // owner = msg.sender; // this would be required in the old setup
    }

    function enter() public payable {
        // $50 minimum
        require(lottery_state == LOTTERY_STATE.OPEN);
        require(msg.value >= getEntranceFee(), "Not enough ETH!");
        players.push(msg.sender);

    }
    function getEntranceFee() public view returns (uint256) {
        (, int256 price, , , ) = ethUsdPriceFeed.latestRoundData();
        uint256 adjustedPrice = uint256(price) * 1e10; // to get to 18 decimals, since the price retrieved is in 8 decimals
        // $50, $2,000 / ETH
        // 50/2,000
        // need to multiply with big integer scaling to Wei, since solidity does not have any decimals
        uint256 costToEnter = (usdEntryFee * 1e18) / adjustedPrice;
        return costToEnter;
    }

    /* // from previous setup
    modifier onlyOwner() {
        require(msg.sender == owner);
        _;
    }    
    */

    function startLottery() public onlyOwner {
        require(lottery_state == LOTTERY_STATE.CLOSED, "Can't start a new lottery yet!");
        lottery_state = LOTTERY_STATE.OPEN;
    }

    // request
    function endLottery() public {
        /*
        // a very bad way of generating a pseudo-random number is taking a globally available number in the system, e.g. a chainid, blocknumber etc and hash it
        // or e.g. msg.value
        // msg.send is also a globally available variables which can be found in the solidity documentation, e.b. block.difficulty
        // you are essentially taking seemingly random numbers and shuffling them together and hashing then
        // the problem is that it can be reverse engineered, as the hashing functions fully deterministic
        // hashing does not introduce any randomness
        uint256(
            keccak256(
                abi.encodePacked(
                    nonce, // nonce is predictable ( aka, transaction number)
                    msg.sender,  // msg.sender is predictable
                    block.difficulty, // can actually be manipulated by the miners! 
                    block.timestamp // timestamp is predictable
                    )
                )
            ) % players.length; 
    }
    */
    lottery_state = LOTTERY_STATE.CALCULATING_WINNER;
    bytes32 requestId = requestRandomness(keyhash, fee);
    }

    // receive
    // this function is internal, since only the VRFCoordinator contract should be able to access / invoke it
    // override means that the initial declaration will be overwritten, i.e. _randomness
    function fulfillRandomness(bytes32 _requestId, uint256 _randomness) internal override {
        // check if the state is correct
        require(lottery_state==LOTTERY_STATE.CALCULATING_WINNER, "You are not there yet!");
        // check that random number was delivered, i.e. the response has arrived   
        require(_randomness >0, "random-not-found");        
        // picking a random winner
        uint256 indexWinner = _randomness % players.length;
        recentWinner = players[indexWinner];
        recentWinner.transfer(address(this).balance);

        // Resetting the lottery by initiating an new empty array of size 0
        players = new address payable[](0); 
        randomness = _randomness;
    }


}