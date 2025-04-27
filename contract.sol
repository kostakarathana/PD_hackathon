// SPDX-License-Identifier: GPL-3.0

pragma solidity >=0.8.2 <0.9.0;

contract ScratchOff {
    address public owner;
    mapping(address => bool) public results;

    constructor() {
        owner = msg.sender;
    }

    function scratch() public returns (bool) {
        uint256 random = uint256(keccak256(abi.encodePacked(block.timestamp, block.number, msg.sender)));
        bool win = random % 2 == 0;

        results[msg.sender] = win;

        return win;
    }

    function checkResult(address player) public view returns (bool) {
        return results[player];
    }
}