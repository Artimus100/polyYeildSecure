// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Lock {
    address public owner;
    mapping(address => uint256) public lockedBalances;

    event TokensLocked(address indexed user, uint256 amount);

    constructor() {
        owner = msg.sender;
    }

    function lockTokens(uint256 amount) public {
        require(amount > 0, "Amount must be greater than zero");

        // Transfer the tokens from the user to this contract
        // Assuming the contract has a function to receive tokens
        // This can be done using ERC20 transferFrom function
        // token.transferFrom(msg.sender, address(this), amount);

        lockedBalances[msg.sender] += amount;

        emit TokensLocked(msg.sender, amount);
    }

    function getLockedBalance(address user) public view returns (uint256) {
        return lockedBalances[user];
    }
}
