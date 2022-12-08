// SPDX-License-Identifer: MIT

pragma solidity ^0.8.0;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol"; // this is needed, since we are calling functions on the ERC20 token contract
import "@chainlink/contracts/src/v0.8/interfaces/AggregatorV3Interface.sol";

contract TokenFarm is Ownable{

    // Rerwards for Staking    
    // 100 ETH  --- 1:1 for every 1 ETH, we give 1 DappToken
    // 50 ETH and 50 DAI staked --- and we want to give a reward of 1 DAPP / 1DAI


    // mapping token address -> staker addres -> amount
    mapping(address => mapping(address => uint256)) public stakingBalance;
    mapping(address => uint256) public uniqueTokensStaked;
    mapping(address => address) public tokenPriceFeedMapping;
    address[] public stakers;
    address[] public allowedTokens;
    // global variable dappToken
    IERC20 public dappToken;

    constructor(address _dappTokenAddress) public {
        dappToken = IERC20(_dappTokenAddress);
    }

    function setPriceFeedContract(address _token, address _priceFeed) public onlyOwner {
        tokenPriceFeedMapping[_token] = _priceFeed;
    }

    // issueTokens -> [x] Done
    function issueTokens() public onlyOwner {
        // Issue tokens to all stakers
        for (uint256 stakersIndex = 0;
            stakersIndex < stakers.length;
            stakersIndex++
        ){
            address recipient = stakers[stakersIndex];
            uint256 userTotalValue = getUserTotalValue(recipient);
            // send the token reward to the staker
            // since the TokenFarm holds the dappTokens, it is also able to transfer them
            // how much tokens have they staked on the platform -> let's issue them the same amount
            dappToken.transfer(recipient, userTotalValue);
        }
    }

    // getValue of staked user tokens -> [x] Done
    function getUserTotalValue(address _user) public view returns (uint256){
        uint256 totalValue = 0;
        require(uniqueTokensStaked[_user] >0, "No tokens staked");
        for (
            uint256 allowedTokensIndex = 0;
            allowedTokensIndex < allowedTokens.length;
            allowedTokensIndex++
        ){
            totalValue = totalValue + getUserSingleTokenValue(_user, allowedTokens[allowedTokensIndex]);
        }
        return totalValue;
    }

    function getUserSingleTokenValue(address _user, address _token) public view returns (uint256) {
        /* This function calculates the conversion rate
            i.e. how much has been staked in this application
         */
        // 1 ETH -> $2,000 --> this should return 2000
        // 200 DAI -> 200 --> this should return 200
        if (uniqueTokensStaked[_user] <= 0){
            return 0;
        }
        // price of the token * stakingBalance[_token][user]
        (uint256 price, uint256 decimals) = getTokenValue(_token);
        // since everything is 
        // 10 ETH (in WEI = 10*1e18)
        // ETH/USD (from priceFeed) -> 100 ETH/USD (decimals are 8) * 1e8
        // 10*1e18 * 100 *1e8 = 1000 / (10**18) 
        // -> to have the same units
        return (stakingBalance[_token][_user] * price / (10**decimals));
    }

    function getTokenValue(address _token) public view returns (uint256, uint256) {
        // priceFeedAddress
        address priceFeedAddress = tokenPriceFeedMapping[_token];
        AggregatorV3Interface priceFeed = AggregatorV3Interface(priceFeedAddress);
        (  ,int256 price,,,) = priceFeed.latestRoundData();
        uint256 decimals = uint256(priceFeed.decimals()); // decimals() return uint8
        return (uint256(price), decimals);
    }

    // stakeTokens -> [x] DONE
    function stakeTokens(uint256 _amount, address _token) public {
        // what tokens can they stake?
        // how much can they stake?
        require(_amount > 0, "Amount must be more than 0");
        // require(_token is allowed???)
        require(tokenIsAllowed(_token), "Token is currently not allowed!");
        // the next function will send tokens of _amount to this TokenFarm contract
        IERC20(_token).transferFrom(msg.sender, address(this), _amount);
        updateUniqueTokensStaked(msg.sender, _token); // function to check how many different tokens the user has already staked
        stakingBalance[_token][msg.sender] = stakingBalance[_token][msg.sender] + _amount;
        if (uniqueTokensStaked[msg.sender] == 1){
            stakers.push(msg.sender);
        }
    }

    // unstakeToken -> [x] Done
    function unstakeTokens(address _token) public {
        uint256 balance = stakingBalance[_token][msg.sender];
        require(balance > 0, "Staking balance cannot be 0");
        IERC20(_token).transfer(msg.sender, balance);
        stakingBalance[_token][msg.sender] = 0;
        uniqueTokensStaked[msg.sender] = uniqueTokensStaked[msg.sender] -1;
    }

    function updateUniqueTokensStaked(address _user, address _token) internal {
        if (stakingBalance[_token][_user] <= 0) {
            uniqueTokensStaked[_user] = uniqueTokensStaked[_user] +1;
        }
    }
    
    // addAllowedTokens -> [x] Done
    function addAllowedTokens(address _token) public onlyOwner {
        allowedTokens.push(_token);
    }

    function tokenIsAllowed(address _token) public returns (bool) {
        for ( uint256 allowedTokensIndex=0; allowedTokensIndex < allowedTokens.length; allowedTokensIndex++) {
            if (allowedTokens[allowedTokensIndex] == _token) {
                return true;
            }
        }
        return true;

    }

}