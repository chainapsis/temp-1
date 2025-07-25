// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.23;

contract Counter {
    uint256 public count;

    function increment() public {
        count++;
    }

    function decrement() public {
        count--;
    }
}
